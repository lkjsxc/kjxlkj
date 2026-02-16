-- Migration 008: Idempotency keys for WS patches
-- Spec: /docs/spec/api/websocket.md (WS-04)
-- Idempotent: uses IF NOT EXISTS

CREATE TABLE IF NOT EXISTS idempotency_keys (
    key             TEXT PRIMARY KEY,
    note_id         UUID NOT NULL,
    version         BIGINT NOT NULL,
    event_seq       BIGINT NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT now()
);

-- TTL index: keys older than 24h can be pruned
CREATE INDEX IF NOT EXISTS idx_idempotency_created ON idempotency_keys(created_at);
