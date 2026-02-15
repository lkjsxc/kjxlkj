-- Migration 006: automation, saved views, dashboards, attachments
-- Per /docs/spec/domain/automation.md, /docs/spec/domain/export.md,
-- /docs/spec/domain/attachments.md

CREATE TABLE automation_rules (
    id              UUID PRIMARY KEY,
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name            TEXT NOT NULL DEFAULT '',
    trigger_type    TEXT NOT NULL,
    condition_json  JSONB NOT NULL DEFAULT '{}',
    action_json     JSONB NOT NULL DEFAULT '{}',
    enabled         BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE automation_runs (
    id          UUID PRIMARY KEY,
    rule_id     UUID NOT NULL REFERENCES automation_rules(id) ON DELETE CASCADE,
    status      TEXT NOT NULL DEFAULT 'pending'
                CHECK (status IN ('pending','running','completed','failed')),
    started_at  TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    result_json JSONB NOT NULL DEFAULT '{}',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE saved_views (
    id              UUID PRIMARY KEY,
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name            TEXT NOT NULL DEFAULT '',
    query_json      JSONB NOT NULL DEFAULT '{}',
    sort            TEXT,
    filters         JSONB NOT NULL DEFAULT '{}',
    owner_user_id   UUID NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE dashboard_widgets (
    id              UUID PRIMARY KEY,
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    widget_type     TEXT NOT NULL,
    config_json     JSONB NOT NULL DEFAULT '{}',
    layout          JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE attachments (
    id          UUID PRIMARY KEY,
    note_id     UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    filename    TEXT NOT NULL,
    mime        TEXT NOT NULL,
    size_bytes  BIGINT NOT NULL,
    sha256      TEXT NOT NULL,
    chunk_count INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE attachment_chunks (
    attachment_id   UUID NOT NULL REFERENCES attachments(id) ON DELETE CASCADE,
    chunk_index     INT NOT NULL,
    data            BYTEA NOT NULL,
    PRIMARY KEY (attachment_id, chunk_index)
);

CREATE TABLE export_jobs (
    id          UUID PRIMARY KEY,
    job_type    TEXT NOT NULL CHECK (job_type IN ('markdown','sql_backup')),
    status      TEXT NOT NULL DEFAULT 'pending'
                CHECK (status IN ('pending','running','completed','failed')),
    artifact_path TEXT,
    started_at  TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
