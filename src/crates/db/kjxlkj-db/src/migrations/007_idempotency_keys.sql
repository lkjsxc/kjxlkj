-- Migration 007: idempotency keys for WS patch deduplication
-- Per /docs/spec/api/websocket.md: duplicate idempotency_key for same
-- note MUST replay-safe-return existing commit identity.

CREATE TABLE idempotency_keys (
    idempotency_key TEXT NOT NULL,
    note_id         UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    event_id        UUID NOT NULL,
    version         BIGINT NOT NULL,
    event_seq       BIGINT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (note_id, idempotency_key)
);

-- Index for cleanup of old idempotency keys
CREATE INDEX idx_idempotency_keys_created ON idempotency_keys(created_at);

-- Add note_metadata table for per-note key-value typed metadata
-- Per /docs/spec/domain/metadata.md
CREATE TABLE note_metadata (
    note_id     UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    key         TEXT NOT NULL CHECK (length(key) <= 64),
    value       JSONB NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (note_id, key)
);
