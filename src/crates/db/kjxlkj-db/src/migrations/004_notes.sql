-- Migration 004: note streams, events, and projections
-- Per /docs/spec/domain/events.md, /docs/spec/domain/notes.md

CREATE TABLE note_streams (
    id              UUID PRIMARY KEY,
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    project_id      UUID REFERENCES projects(id) ON DELETE SET NULL,
    title           TEXT NOT NULL DEFAULT 'Untitled',
    note_kind       TEXT NOT NULL DEFAULT 'markdown'
                    CHECK (note_kind IN ('markdown','settings','media_image','media_video')),
    access_scope    TEXT NOT NULL DEFAULT 'workspace'
                    CHECK (access_scope IN ('workspace','project','private')),
    current_version BIGINT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_note_streams_workspace ON note_streams(workspace_id);
CREATE INDEX idx_note_streams_project ON note_streams(project_id);

CREATE TABLE note_events (
    event_id    UUID PRIMARY KEY,
    note_id     UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    seq         BIGINT NOT NULL,
    event_type  TEXT NOT NULL,
    payload_json JSONB NOT NULL DEFAULT '{}',
    actor_id    UUID NOT NULL REFERENCES users(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (note_id, seq)
);

CREATE TABLE note_projections (
    note_id         UUID PRIMARY KEY REFERENCES note_streams(id) ON DELETE CASCADE,
    workspace_id    UUID NOT NULL,
    project_id      UUID,
    title           TEXT NOT NULL DEFAULT 'Untitled',
    note_kind       TEXT NOT NULL DEFAULT 'markdown',
    version         BIGINT NOT NULL DEFAULT 0,
    markdown        TEXT NOT NULL DEFAULT '',
    rendered_html   TEXT NOT NULL DEFAULT '',
    metadata_json   JSONB NOT NULL DEFAULT '{}',
    search_vector   TSVECTOR
);

CREATE INDEX idx_note_projections_search ON note_projections
    USING GIN(search_vector);

CREATE TABLE note_snapshots (
    note_id     UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    at_seq      BIGINT NOT NULL,
    markdown    TEXT NOT NULL,
    metadata_json JSONB NOT NULL DEFAULT '{}',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (note_id, at_seq)
);
