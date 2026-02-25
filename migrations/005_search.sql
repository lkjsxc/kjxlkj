-- Migration 005: Search indexes (lexical + vector)
-- Spec: /docs/spec/domain/search.md

-- Lexical search via tsvector + GIN
ALTER TABLE note_projections
    ADD COLUMN IF NOT EXISTS tsv tsvector;

CREATE INDEX IF NOT EXISTS idx_note_projections_tsv
    ON note_projections USING GIN(tsv);

-- Update tsvector on insert/update
CREATE OR REPLACE FUNCTION note_projections_tsv_update() RETURNS trigger AS $$
BEGIN
    NEW.tsv := setweight(to_tsvector('english', COALESCE(NEW.title, '')), 'A')
            || setweight(to_tsvector('english', COALESCE(NEW.markdown, '')), 'B');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS note_projections_tsv_trigger ON note_projections;
CREATE TRIGGER note_projections_tsv_trigger
    BEFORE INSERT OR UPDATE ON note_projections
    FOR EACH ROW EXECUTE FUNCTION note_projections_tsv_update();

-- Backlink projection table
CREATE TABLE IF NOT EXISTS backlinks (
    source_note_id  UUID NOT NULL REFERENCES note_streams(note_id) ON DELETE CASCADE,
    target_note_id  UUID NOT NULL REFERENCES note_streams(note_id) ON DELETE CASCADE,
    created_at      TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (source_note_id, target_note_id)
);

CREATE INDEX IF NOT EXISTS idx_backlinks_target ON backlinks(target_note_id);

-- Vector embedding storage for semantic search (requires pgvector)
-- CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS note_embeddings (
    note_id         UUID PRIMARY KEY REFERENCES note_streams(note_id) ON DELETE CASCADE,
    embedding       BYTEA NOT NULL,
    model           TEXT NOT NULL,
    dimensions      INTEGER NOT NULL,
    updated_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_note_embeddings_model ON note_embeddings(model);
