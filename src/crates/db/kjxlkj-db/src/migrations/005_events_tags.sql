-- Migration 005: workspace events, tags, backlinks
-- Per /docs/spec/domain/events.md, /docs/spec/domain/search.md

CREATE TABLE workspace_events (
    event_id        UUID PRIMARY KEY,
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    seq             BIGINT NOT NULL,
    event_type      TEXT NOT NULL,
    payload_json    JSONB NOT NULL DEFAULT '{}',
    actor_id        UUID NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (workspace_id, seq)
);

CREATE TABLE tags (
    id          UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    UNIQUE (workspace_id, name)
);

CREATE TABLE note_tags (
    note_id     UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    tag_id      UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (note_id, tag_id)
);

CREATE TABLE backlinks (
    source_note_id  UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    target_note_id  UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    PRIMARY KEY (source_note_id, target_note_id)
);
