ALTER TABLE article_search_index
    ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ADD COLUMN IF NOT EXISTS last_history_commit_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_article_search_index_created_at
    ON article_search_index (created_at ASC, slug ASC);

UPDATE article_search_index
SET created_at = updated_at
WHERE created_at IS DISTINCT FROM updated_at
  AND created_at > updated_at;
