-- Migration 008: Idempotency and cursor tracking
-- Spec: /docs/spec/api/websocket.md

-- WebSocket cursor tracking per connection
CREATE TABLE IF NOT EXISTS ws_cursors (
    connection_id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    note_id         UUID REFERENCES note_streams(note_id) ON DELETE CASCADE,
    workspace_id    UUID REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    event_seq       BIGINT NOT NULL DEFAULT 0,
    last_ack_at     TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ws_cursors_user ON ws_cursors(user_id);
CREATE INDEX IF NOT EXISTS idx_ws_cursors_note ON ws_cursors(note_id);

-- Extend idempotency_keys for general use
ALTER TABLE idempotency_keys
    ADD COLUMN IF NOT EXISTS response_data JSONB;
