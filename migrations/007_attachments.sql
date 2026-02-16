-- Migration 007: Attachments (chunked storage)
-- Spec: /docs/spec/domain/attachments.md
-- Idempotent: uses IF NOT EXISTS

CREATE TABLE IF NOT EXISTS attachments (
    id              UUID PRIMARY KEY,
    note_id         UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    filename        TEXT NOT NULL,
    content_type    TEXT NOT NULL,
    size_bytes      BIGINT NOT NULL,
    sha256          TEXT NOT NULL,
    chunk_count     INTEGER NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS attachment_chunks (
    attachment_id   UUID NOT NULL REFERENCES attachments(id) ON DELETE CASCADE,
    chunk_index     INTEGER NOT NULL,
    data            BYTEA NOT NULL,
    sha256          TEXT NOT NULL,
    PRIMARY KEY (attachment_id, chunk_index)
);

CREATE INDEX IF NOT EXISTS idx_attachments_note ON attachments(note_id);

-- Metadata key-value per /docs/spec/domain/metadata.md
CREATE TABLE IF NOT EXISTS note_metadata (
    note_id         UUID NOT NULL REFERENCES note_streams(id) ON DELETE CASCADE,
    key             TEXT NOT NULL,
    value           JSONB NOT NULL,
    updated_at      TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (note_id, key)
);
