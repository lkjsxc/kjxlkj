// Database migration support per /docs/spec/technical/migrations.md
use sqlx::PgPool;

/// Run all migrations in order.
/// Covers schema domains from /docs/spec/technical/migrations.md.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(SCHEMA_SQL).execute(pool).await?;
    tracing::info!("database migrations applied successfully");
    Ok(())
}

/// Full schema SQL covering all migration domains:
/// users, sessions, workspaces, membership, projects,
/// views, notes, events, projections, tags, backlinks,
/// automation, attachments.
const SCHEMA_SQL: &str = r#"
-- Users and sessions
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'viewer',
    status TEXT NOT NULL DEFAULT 'active',
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    token TEXT NOT NULL UNIQUE,
    csrf_token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Workspaces and membership
CREATE TABLE IF NOT EXISTS workspaces (
    id UUID PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    owner_user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS workspace_members (
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    user_id UUID NOT NULL REFERENCES users(id),
    role TEXT NOT NULL DEFAULT 'viewer',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (workspace_id, user_id)
);

-- Projects
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(workspace_id, name)
);

-- Saved views
CREATE TABLE IF NOT EXISTS saved_views (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    query_json JSONB NOT NULL DEFAULT '{}',
    sort TEXT,
    filters JSONB,
    owner_user_id UUID NOT NULL REFERENCES users(id)
);

-- Note streams
CREATE TABLE IF NOT EXISTS note_streams (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    project_id UUID REFERENCES projects(id),
    title TEXT NOT NULL,
    note_kind TEXT NOT NULL DEFAULT 'markdown',
    access_scope TEXT NOT NULL DEFAULT 'workspace',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
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
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(note_id, seq)
);

-- Note projections
CREATE TABLE IF NOT EXISTS note_projections (
    note_id UUID PRIMARY KEY REFERENCES note_streams(id),
    workspace_id UUID NOT NULL,
    project_id UUID,
    title TEXT NOT NULL,
    note_kind TEXT NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    markdown TEXT NOT NULL DEFAULT '',
    metadata_json JSONB NOT NULL DEFAULT '{}',
    search_vector tsvector
);

CREATE INDEX IF NOT EXISTS idx_note_proj_search
    ON note_projections USING GIN (search_vector);

-- Note snapshots (every 100 events)
CREATE TABLE IF NOT EXISTS note_snapshots (
    note_id UUID NOT NULL REFERENCES note_streams(id),
    version BIGINT NOT NULL,
    markdown TEXT NOT NULL,
    metadata_json JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (note_id, version)
);

-- Tags
CREATE TABLE IF NOT EXISTS tags (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS note_tags (
    note_id UUID NOT NULL REFERENCES note_streams(id),
    tag_id UUID NOT NULL REFERENCES tags(id),
    PRIMARY KEY (note_id, tag_id)
);

-- Backlinks
CREATE TABLE IF NOT EXISTS backlinks (
    source_note_id UUID NOT NULL REFERENCES note_streams(id),
    target_note_id UUID NOT NULL REFERENCES note_streams(id),
    PRIMARY KEY (source_note_id, target_note_id)
);

-- Workspace events
CREATE TABLE IF NOT EXISTS workspace_events (
    event_id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    seq BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL DEFAULT '{}',
    actor_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(workspace_id, seq)
);

-- Automation rules
CREATE TABLE IF NOT EXISTS automation_rules (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    trigger TEXT NOT NULL,
    condition_json JSONB NOT NULL DEFAULT '{}',
    action_json JSONB NOT NULL DEFAULT '{}',
    enabled BOOLEAN NOT NULL DEFAULT true
);

-- Automation runs
CREATE TABLE IF NOT EXISTS automation_runs (
    id UUID PRIMARY KEY,
    rule_id UUID NOT NULL REFERENCES automation_rules(id),
    status TEXT NOT NULL DEFAULT 'pending',
    started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at TIMESTAMPTZ,
    result_json JSONB
);

-- Attachments
CREATE TABLE IF NOT EXISTS attachments (
    id UUID PRIMARY KEY,
    note_id UUID NOT NULL REFERENCES note_streams(id),
    filename TEXT NOT NULL,
    mime TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    sha256 TEXT NOT NULL,
    chunk_count INT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS attachment_chunks (
    attachment_id UUID NOT NULL REFERENCES attachments(id),
    chunk_index INT NOT NULL,
    data BYTEA NOT NULL,
    PRIMARY KEY (attachment_id, chunk_index)
);
"#;
