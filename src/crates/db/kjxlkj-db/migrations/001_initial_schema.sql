-- Initial schema migration for kjxlkj.
-- Covers: users, sessions, workspaces, members, projects, notes,
--         events, projections, metadata, search, automation, agent KV.

-- Users
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL DEFAULT '',
    password_hash TEXT NOT NULL,
    is_disabled BOOLEAN NOT NULL DEFAULT false,
    is_owner BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Sessions
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    csrf_token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);

-- Workspaces
CREATE TABLE IF NOT EXISTS workspaces (
    id UUID PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    owner_user_id UUID NOT NULL REFERENCES users(id),
    state TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Workspace members
CREATE TABLE IF NOT EXISTS workspace_members (
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'viewer',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (workspace_id, user_id)
);

-- Projects
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    is_archived BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(workspace_id, name)
);

-- Note streams
CREATE TABLE IF NOT EXISTS note_streams (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    project_id UUID REFERENCES projects(id),
    title TEXT NOT NULL,
    note_kind TEXT NOT NULL DEFAULT 'markdown',
    access_scope TEXT NOT NULL DEFAULT 'workspace',
    current_version BIGINT NOT NULL DEFAULT 0,
    is_deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_notes_workspace ON note_streams(workspace_id)
    WHERE NOT is_deleted;
CREATE INDEX IF NOT EXISTS idx_notes_project ON note_streams(project_id)
    WHERE project_id IS NOT NULL AND NOT is_deleted;

-- Note events (append-only)
CREATE TABLE IF NOT EXISTS note_events (
    id UUID PRIMARY KEY,
    note_id UUID NOT NULL REFERENCES note_streams(id),
    seq BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}',
    actor_id UUID NOT NULL,
    actor_type TEXT NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(note_id, seq)
);

CREATE INDEX IF NOT EXISTS idx_note_events_note ON note_events(note_id, seq);

-- Note projections (current state cache)
CREATE TABLE IF NOT EXISTS note_projections (
    note_id UUID PRIMARY KEY REFERENCES note_streams(id),
    version BIGINT NOT NULL DEFAULT 0,
    markdown TEXT NOT NULL DEFAULT '',
    metadata_json JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Note snapshots (every 100 events per stream)
CREATE TABLE IF NOT EXISTS note_snapshots (
    id UUID PRIMARY KEY,
    note_id UUID NOT NULL REFERENCES note_streams(id),
    at_seq BIGINT NOT NULL,
    markdown TEXT NOT NULL,
    metadata_json JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(note_id, at_seq)
);

-- Workspace events (append-only)
CREATE TABLE IF NOT EXISTS workspace_events (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    seq BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}',
    actor_id UUID NOT NULL,
    actor_type TEXT NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(workspace_id, seq)
);

-- Per-note metadata
CREATE TABLE IF NOT EXISTS note_metadata (
    note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (note_id, key)
);

-- Backlinks
CREATE TABLE IF NOT EXISTS backlinks (
    source_note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    target_note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    PRIMARY KEY (source_note_id, target_note_id)
);

CREATE INDEX IF NOT EXISTS idx_backlinks_target ON backlinks(target_note_id);

-- Full-text search (lexical via tsvector + GIN)
CREATE TABLE IF NOT EXISTS note_search_index (
    note_id UUID PRIMARY KEY REFERENCES note_streams(id) ON DELETE CASCADE,
    tsv TSVECTOR NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_note_search_gin ON note_search_index USING GIN(tsv);

-- Embedding vectors for semantic search
CREATE TABLE IF NOT EXISTS note_embeddings (
    note_id UUID PRIMARY KEY REFERENCES note_streams(id) ON DELETE CASCADE,
    embedding BYTEA NOT NULL,
    model TEXT NOT NULL,
    dimensions INT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Automation rules
CREATE TABLE IF NOT EXISTS automation_rules (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    trigger_kind TEXT NOT NULL,
    condition_json JSONB NOT NULL DEFAULT '{}',
    action_json JSONB NOT NULL DEFAULT '{}',
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_auto_rules_ws ON automation_rules(workspace_id);

-- Automation runs
CREATE TABLE IF NOT EXISTS automation_runs (
    id UUID PRIMARY KEY,
    rule_id UUID NOT NULL REFERENCES automation_rules(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'queued',
    started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at TIMESTAMPTZ,
    result_json JSONB,
    idempotency_key TEXT,
    UNIQUE(rule_id, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_auto_runs_rule ON automation_runs(rule_id);

-- Agent KV memory store
CREATE TABLE IF NOT EXISTS agent_kv_store (
    agent_name TEXT NOT NULL,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (agent_name, workspace_id, key)
);

-- Attachments
CREATE TABLE IF NOT EXISTS attachments (
    id UUID PRIMARY KEY,
    note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'application/octet-stream',
    total_size BIGINT NOT NULL,
    checksum_sha256 TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_attachments_note ON attachments(note_id);

-- Attachment chunks
CREATE TABLE IF NOT EXISTS attachment_chunks (
    attachment_id UUID NOT NULL REFERENCES attachments(id) ON DELETE CASCADE,
    chunk_index INT NOT NULL,
    data BYTEA NOT NULL,
    checksum_sha256 TEXT NOT NULL,
    PRIMARY KEY (attachment_id, chunk_index)
);
