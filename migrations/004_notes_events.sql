-- Migration 004: Notes, Events, Projections
-- Spec: /docs/spec/domain/notes.md, /docs/spec/domain/events.md
-- Idempotent: uses IF NOT EXISTS

CREATE TABLE IF NOT EXISTS note_streams (
    id              UUID PRIMARY KEY,
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    project_id      UUID REFERENCES projects(id) ON DELETE SET NULL,
    title           TEXT NOT NULL,
    note_kind       TEXT NOT NULL DEFAULT 'markdown'
                    CHECK (note_kind IN ('markdown','settings','media_image','media_video')),
    access_scope    TEXT NOT NULL DEFAULT 'workspace'
                    CHECK (access_scope IN ('workspace','project')),
    state           TEXT NOT NULL DEFAULT 'active'
                    CHECK (state IN ('active','soft_deleted')),
    current_version BIGINT NOT NULL DEFAULT 0,
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    updated_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS note_events (
    id              UUID PRIMARY KEY,
    note_id         UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    seq             BIGINT NOT NULL,
    event_type      TEXT NOT NULL,
    actor_type      TEXT NOT NULL CHECK (actor_type IN ('user','agent','system')),
    actor_id        UUID NOT NULL,
    payload         JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    UNIQUE (note_id, seq)
);

CREATE TABLE IF NOT EXISTS note_projections (
    note_id         UUID PRIMARY KEY REFERENCES note_streams(id) ON DELETE CASCADE,
    title           TEXT NOT NULL,
    version         BIGINT NOT NULL DEFAULT 0,
    markdown        TEXT NOT NULL DEFAULT '',
    metadata_json   JSONB NOT NULL DEFAULT '{}',
    updated_at      TIMESTAMP NOT NULL DEFAULT now()
);

-- Snapshot every 100 events per /docs/spec/domain/events.md
CREATE TABLE IF NOT EXISTS note_snapshots (
    note_id         UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    at_seq          BIGINT NOT NULL,
    markdown        TEXT NOT NULL,
    metadata_json   JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (note_id, at_seq)
);

CREATE INDEX IF NOT EXISTS idx_note_streams_workspace ON note_streams(workspace_id);
CREATE INDEX IF NOT EXISTS idx_note_streams_project ON note_streams(project_id);
CREATE INDEX IF NOT EXISTS idx_note_events_note ON note_events(note_id, seq);
CREATE INDEX IF NOT EXISTS idx_note_streams_state ON note_streams(state);

-- Workspace events
CREATE TABLE IF NOT EXISTS workspace_events (
    id              UUID PRIMARY KEY,
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    seq             BIGINT NOT NULL,
    event_type      TEXT NOT NULL,
    actor_type      TEXT NOT NULL CHECK (actor_type IN ('user','agent','system')),
    actor_id        UUID NOT NULL,
    payload         JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    UNIQUE (workspace_id, seq)
);
