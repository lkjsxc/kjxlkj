-- Migration 007: Attachments
-- Spec: /docs/spec/domain/attachments.md

CREATE TABLE IF NOT EXISTS attachments (
    attachment_id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    note_id         UUID NOT NULL REFERENCES note_streams(note_id) ON DELETE CASCADE,
    filename        TEXT NOT NULL,
    content_type    TEXT NOT NULL,
    size_bytes      BIGINT NOT NULL,
    checksum_sha256 TEXT NOT NULL,
    storage_path    TEXT NOT NULL,
    uploaded_by     UUID NOT NULL REFERENCES users(user_id),
    created_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_attachments_note ON attachments(note_id);
CREATE INDEX IF NOT EXISTS idx_attachments_checksum ON attachments(checksum_sha256);

-- Attachment chunks for large files
CREATE TABLE IF NOT EXISTS attachment_chunks (
    attachment_id   UUID NOT NULL REFERENCES attachments(attachment_id) ON DELETE CASCADE,
    chunk_index     INTEGER NOT NULL,
    chunk_data      BYTEA NOT NULL,
    PRIMARY KEY (attachment_id, chunk_index)
);
