CREATE TABLE IF NOT EXISTS note_streams (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    project_id UUID NULL REFERENCES projects(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    note_kind TEXT NOT NULL CHECK (note_kind IN ('markdown', 'settings', 'media_image', 'media_video')),
    access_scope TEXT NOT NULL CHECK (access_scope IN ('workspace', 'project', 'private')),
    current_version INT NOT NULL,
    deleted_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS note_projections (
    note_id UUID PRIMARY KEY REFERENCES note_streams(id) ON DELETE CASCADE,
    workspace_id UUID NOT NULL,
    project_id UUID NULL,
    title TEXT NOT NULL,
    note_kind TEXT NOT NULL,
    version INT NOT NULL,
    markdown TEXT NOT NULL,
    rendered_html TEXT NOT NULL DEFAULT '',
    metadata_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    search_vector tsvector GENERATED ALWAYS AS (
        to_tsvector('english', coalesce(title, '') || ' ' || coalesce(markdown, ''))
    ) STORED
);

CREATE TABLE IF NOT EXISTS note_events (
    event_id UUID PRIMARY KEY,
    note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    seq INT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    actor_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (note_id, seq)
);

CREATE TABLE IF NOT EXISTS note_snapshots (
    note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    version INT NOT NULL,
    markdown TEXT NOT NULL,
    metadata_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (note_id, version)
);

CREATE TABLE IF NOT EXISTS note_metadata (
    note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value_json JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (note_id, key)
);

CREATE TABLE IF NOT EXISTS note_tags (
    note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    tag TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (note_id, tag)
);

CREATE TABLE IF NOT EXISTS note_backlinks (
    note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    target_title TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (note_id, target_title)
);

CREATE TABLE IF NOT EXISTS note_patch_idempotency (
    note_id UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    idempotency_key TEXT NOT NULL,
    event_seq INT NOT NULL,
    version INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (note_id, idempotency_key)
);

CREATE TABLE IF NOT EXISTS workspace_events (
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    seq INT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    actor_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (workspace_id, seq)
);

CREATE INDEX IF NOT EXISTS idx_note_streams_workspace_id_deleted_at ON note_streams(workspace_id, deleted_at);
CREATE INDEX IF NOT EXISTS idx_note_streams_project_id ON note_streams(project_id);
CREATE INDEX IF NOT EXISTS idx_note_events_note_id_seq ON note_events(note_id, seq);
CREATE INDEX IF NOT EXISTS idx_note_backlinks_target_title ON note_backlinks(target_title);
CREATE INDEX IF NOT EXISTS idx_note_projections_search_vector ON note_projections USING GIN(search_vector);
CREATE INDEX IF NOT EXISTS idx_workspace_events_workspace_id_seq ON workspace_events(workspace_id, seq);
