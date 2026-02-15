//! Migration runner per /docs/spec/technical/migrations.md.

/// SQL migration script — creates all tables needed for the domain.
/// Per /docs/spec/technical/migrations.md, covers users, sessions, workspaces,
/// membership, projects, notes, events, projections, tags, backlinks,
/// automation rules/runs, attachments, saved views, dashboard widgets.
pub const INIT_MIGRATION: &str = r#"
-- Users and sessions
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'editor',
    status TEXT NOT NULL DEFAULT 'active',
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    csrf_token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Workspaces and membership
CREATE TABLE IF NOT EXISTS workspaces (
    id UUID PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    owner_user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS workspace_members (
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    user_id UUID NOT NULL REFERENCES users(id),
    role TEXT NOT NULL DEFAULT 'editor',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (workspace_id, user_id)
);

-- Projects
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(workspace_id, name)
);

-- Note streams
CREATE TABLE IF NOT EXISTS note_streams (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    project_id UUID REFERENCES projects(id),
    title TEXT NOT NULL DEFAULT 'Untitled',
    note_kind TEXT NOT NULL DEFAULT 'markdown',
    access_scope TEXT NOT NULL DEFAULT 'workspace',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    current_version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ
);

-- Note events (append-only)
CREATE TABLE IF NOT EXISTS note_events (
    event_id UUID PRIMARY KEY,
    note_id UUID NOT NULL REFERENCES note_streams(id),
    seq BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL DEFAULT '{}',
    actor_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(note_id, seq)
);

-- Workspace events (append-only)
CREATE TABLE IF NOT EXISTS workspace_events (
    event_id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    seq BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL DEFAULT '{}',
    actor_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(workspace_id, seq)
);

-- Note projections
CREATE TABLE IF NOT EXISTS note_projections (
    note_id UUID PRIMARY KEY REFERENCES note_streams(id),
    workspace_id UUID NOT NULL,
    project_id UUID,
    title TEXT NOT NULL DEFAULT '',
    note_kind TEXT NOT NULL DEFAULT 'markdown',
    version BIGINT NOT NULL DEFAULT 0,
    markdown TEXT NOT NULL DEFAULT '',
    metadata_json JSONB NOT NULL DEFAULT '{}',
    search_vector TSVECTOR
);

-- Note snapshots per /docs/spec/domain/events.md snapshot policy
CREATE TABLE IF NOT EXISTS note_snapshots (
    note_id UUID NOT NULL REFERENCES note_streams(id),
    at_seq BIGINT NOT NULL,
    markdown TEXT NOT NULL,
    metadata_json JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (note_id, at_seq)
);

-- Tags
CREATE TABLE IF NOT EXISTS note_tags (
    note_id UUID NOT NULL REFERENCES note_streams(id),
    tag TEXT NOT NULL,
    PRIMARY KEY (note_id, tag)
);

-- Backlinks
CREATE TABLE IF NOT EXISTS backlinks (
    source_note_id UUID NOT NULL REFERENCES note_streams(id),
    target_title TEXT NOT NULL,
    PRIMARY KEY (source_note_id, target_title)
);

-- FTS index
CREATE INDEX IF NOT EXISTS idx_note_projections_search
    ON note_projections USING GIN(search_vector);

-- Automation rules
CREATE TABLE IF NOT EXISTS automation_rules (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    trigger TEXT NOT NULL,
    condition_json JSONB NOT NULL DEFAULT '{}',
    action_json JSONB NOT NULL DEFAULT '{}',
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Automation runs
CREATE TABLE IF NOT EXISTS automation_runs (
    id UUID PRIMARY KEY,
    rule_id UUID NOT NULL REFERENCES automation_rules(id),
    status TEXT NOT NULL DEFAULT 'queued',
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finished_at TIMESTAMPTZ,
    result_json JSONB
);

-- Attachments
CREATE TABLE IF NOT EXISTS attachments (
    id UUID PRIMARY KEY,
    note_id UUID NOT NULL REFERENCES note_streams(id),
    filename TEXT NOT NULL,
    mime TEXT NOT NULL,
    size_bytes BIGINT NOT NULL DEFAULT 0,
    sha256 TEXT NOT NULL DEFAULT '',
    chunk_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS attachment_chunks (
    attachment_id UUID NOT NULL REFERENCES attachments(id) ON DELETE CASCADE,
    chunk_index INTEGER NOT NULL,
    data BYTEA NOT NULL,
    PRIMARY KEY (attachment_id, chunk_index)
);

-- Saved views
CREATE TABLE IF NOT EXISTS saved_views (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    name TEXT NOT NULL DEFAULT '',
    query_json JSONB NOT NULL DEFAULT '{}',
    sort TEXT NOT NULL DEFAULT 'updated_at_desc',
    filters JSONB NOT NULL DEFAULT '{}',
    owner_user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Dashboard widgets (optional extension)
CREATE TABLE IF NOT EXISTS dashboard_widgets (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    widget_type TEXT NOT NULL DEFAULT '',
    config_json JSONB NOT NULL DEFAULT '{}',
    layout JSONB
);

-- Idempotency tracking for WS patches
CREATE TABLE IF NOT EXISTS idempotency_keys (
    note_id UUID NOT NULL REFERENCES note_streams(id),
    idempotency_key TEXT NOT NULL,
    event_id UUID NOT NULL,
    version BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (note_id, idempotency_key)
);

-- Export/backup jobs
CREATE TABLE IF NOT EXISTS jobs (
    id UUID PRIMARY KEY,
    job_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'queued',
    workspace_id UUID,
    result_json JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finished_at TIMESTAMPTZ
);
"#;

/// Run the init migration against a pool.
pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(INIT_MIGRATION).execute(pool).await?;
    Ok(())
}
