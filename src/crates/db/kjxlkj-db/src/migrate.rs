// Database migration support per /docs/spec/technical/migrations.md
// Migrations MUST be ordered and deterministic.
// Forward migrations MUST be idempotent in deployment scripts.
// Migration failures MUST fail startup readiness.
use sqlx::PgPool;

/// Run all migrations in order.
/// Covers schema domains from /docs/spec/technical/migrations.md.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create migration tracking table
    sqlx::query(MIGRATION_TABLE_SQL).execute(pool).await?;

    // Apply each migration in order, skipping already-applied ones
    for (version, name, sql) in MIGRATIONS {
        let applied: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM _migrations WHERE version = $1)",
        )
        .bind(version)
        .fetch_one(pool)
        .await?;

        if !applied {
            sqlx::query(sql).execute(pool).await?;
            sqlx::query("INSERT INTO _migrations (version, name) VALUES ($1, $2)")
                .bind(version)
                .bind(name)
                .execute(pool)
                .await?;
            tracing::info!(version, name, "migration applied");
        }
    }
    tracing::info!("database migrations applied successfully");
    Ok(())
}

const MIGRATION_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS _migrations (
    version INT PRIMARY KEY,
    name TEXT NOT NULL,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
"#;

/// Ordered migrations. Each tuple: (version, name, sql).
const MIGRATIONS: &[(i32, &str, &str)] = &[
    (1, "initial_schema", SCHEMA_V1),
    (2, "librarian_and_jobs", SCHEMA_V2),
];

/// V1: Core schema covering all initial domains.
const SCHEMA_V1: &str = r#"
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

/// V2: Librarian run reports, operation audit logs, export/backup jobs.
/// Per /docs/spec/domain/automation.md and /docs/spec/domain/export.md.
const SCHEMA_V2: &str = r#"
-- Librarian run reports (per /docs/spec/technical/librarian-agent.md)
CREATE TABLE IF NOT EXISTS librarian_run_reports (
    run_id UUID PRIMARY KEY REFERENCES automation_runs(id),
    provider_kind TEXT NOT NULL,
    model TEXT NOT NULL,
    prompt_hash TEXT NOT NULL,
    parser_version TEXT NOT NULL DEFAULT '1',
    raw_request TEXT,
    raw_response TEXT,
    parse_warnings JSONB NOT NULL DEFAULT '[]',
    operations_proposed INT NOT NULL DEFAULT 0,
    operations_accepted INT NOT NULL DEFAULT 0,
    operations_rejected INT NOT NULL DEFAULT 0
);

-- Librarian operation audit logs
CREATE TABLE IF NOT EXISTS librarian_operations (
    id UUID PRIMARY KEY,
    run_id UUID NOT NULL REFERENCES automation_runs(id),
    operation_index INT NOT NULL,
    kind TEXT NOT NULL,
    target_note_id UUID,
    title TEXT,
    body_markdown TEXT,
    reason TEXT,
    confidence REAL,
    status TEXT NOT NULL DEFAULT 'pending',
    review_decision TEXT,
    reviewer_id UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    UNIQUE(run_id, operation_index)
);

-- Add triggering event tracking to automation_runs
ALTER TABLE automation_runs
    ADD COLUMN IF NOT EXISTS triggering_event_id UUID,
    ADD COLUMN IF NOT EXISTS error_detail TEXT;

-- Unique constraint for idempotent runs per (rule_id, triggering_event_id)
CREATE UNIQUE INDEX IF NOT EXISTS idx_runs_idempotent
    ON automation_runs(rule_id, triggering_event_id)
    WHERE triggering_event_id IS NOT NULL;

-- Export/backup jobs (per /docs/spec/domain/export.md)
CREATE TABLE IF NOT EXISTS jobs (
    id UUID PRIMARY KEY,
    job_type TEXT NOT NULL,
    workspace_id UUID REFERENCES workspaces(id),
    status TEXT NOT NULL DEFAULT 'queued',
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    artifact_path TEXT,
    error_detail TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Add workspace event sequence tracking
CREATE SEQUENCE IF NOT EXISTS workspace_event_seq;
CREATE SEQUENCE IF NOT EXISTS note_event_seq;
"#;
