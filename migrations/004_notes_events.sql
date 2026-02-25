-- Migration 004: Notes, events, and projections
-- Spec: /docs/spec/domain/notes.md, /docs/spec/domain/events.md

-- Note streams (canonical identity)
CREATE TABLE IF NOT EXISTS note_streams (
    note_id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id    UUID NOT NULL REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    project_id      UUID REFERENCES projects(project_id) ON DELETE SET NULL,
    note_kind       TEXT NOT NULL DEFAULT 'note' CHECK (note_kind IN ('note', 'template', 'document', 'meeting')),
    access_scope    TEXT NOT NULL DEFAULT 'private' CHECK (access_scope IN ('private', 'workspace', 'public')),
    version         BIGINT NOT NULL DEFAULT 1,
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    updated_at      TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at      TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_note_streams_workspace ON note_streams(workspace_id);
CREATE INDEX IF NOT EXISTS idx_note_streams_project ON note_streams(project_id);
CREATE INDEX IF NOT EXISTS idx_note_streams_deleted ON note_streams(deleted_at) WHERE deleted_at IS NULL;

-- Note projections (materialized view for reads)
CREATE TABLE IF NOT EXISTS note_projections (
    note_id         UUID PRIMARY KEY REFERENCES note_streams(note_id) ON DELETE CASCADE,
    title           TEXT NOT NULL,
    markdown        TEXT NOT NULL DEFAULT '',
    workspace_id    UUID NOT NULL,
    project_id      UUID,
    note_kind       TEXT NOT NULL,
    access_scope    TEXT NOT NULL,
    version         BIGINT NOT NULL,
    created_at      TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_note_projections_workspace ON note_projections(workspace_id);
CREATE INDEX IF NOT EXISTS idx_note_projections_title ON note_projections(title);

-- Note events (immutable append-only log)
CREATE TABLE IF NOT EXISTS note_events (
    event_id        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    note_id         UUID NOT NULL REFERENCES note_streams(note_id) ON DELETE CASCADE,
    event_seq       BIGINT NOT NULL,
    version         BIGINT NOT NULL,
    actor_type      TEXT NOT NULL CHECK (actor_type IN ('user', 'agent')),
    actor_id        UUID NOT NULL,
    timestamp       TIMESTAMP NOT NULL DEFAULT now(),
    payload         JSONB NOT NULL,
    UNIQUE (note_id, event_seq)
);

CREATE INDEX IF NOT EXISTS idx_note_events_note ON note_events(note_id);
CREATE INDEX IF NOT EXISTS idx_note_events_seq ON note_events(note_id, event_seq);

-- Idempotency keys for deduplication
CREATE TABLE IF NOT EXISTS idempotency_keys (
    idempotency_key UUID PRIMARY KEY,
    note_id         UUID NOT NULL REFERENCES note_streams(note_id) ON DELETE CASCADE,
    event_id        UUID NOT NULL REFERENCES note_events(event_id) ON DELETE CASCADE,
    created_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_idempotency_note ON idempotency_keys(note_id);
