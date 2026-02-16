-- Migration 002: Workspaces and Membership
-- Spec: /docs/spec/domain/workspaces.md
-- Idempotent: uses IF NOT EXISTS

CREATE TABLE IF NOT EXISTS workspaces (
    id              UUID PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    name            TEXT NOT NULL,
    owner_user_id   UUID NOT NULL REFERENCES users(id),
    state           TEXT NOT NULL DEFAULT 'active'
                    CHECK (state IN ('active','archived','deleted')),
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    updated_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS workspace_members (
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role            TEXT NOT NULL CHECK (role IN ('viewer','editor','admin','owner')),
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (workspace_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_workspaces_slug ON workspaces(slug);
CREATE INDEX IF NOT EXISTS idx_workspace_members_user ON workspace_members(user_id);
