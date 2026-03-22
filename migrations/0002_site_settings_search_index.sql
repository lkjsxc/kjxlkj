CREATE TABLE IF NOT EXISTS site_settings (
    id SMALLINT PRIMARY KEY CHECK (id = 1),
    site_title TEXT NOT NULL,
    session_timeout_minutes INTEGER NOT NULL CHECK (session_timeout_minutes >= 5),
    search_last_reindex_at TIMESTAMPTZ
);

INSERT INTO site_settings (id, site_title, session_timeout_minutes)
VALUES (1, 'Knowledge Base', 1440)
ON CONFLICT (id) DO NOTHING;

CREATE TABLE IF NOT EXISTS article_search_index (
    slug TEXT PRIMARY KEY,
    title TEXT,
    body TEXT NOT NULL,
    private BOOLEAN NOT NULL DEFAULT false,
    trashed BOOLEAN NOT NULL DEFAULT false,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_article_search_index_fts
ON article_search_index
USING GIN (to_tsvector('simple', coalesce(slug, '') || ' ' || coalesce(title, '') || ' ' || coalesce(body, '')));

CREATE OR REPLACE FUNCTION search_articles(query TEXT, include_private BOOLEAN)
RETURNS TABLE (slug TEXT, title TEXT, private BOOLEAN, snippet TEXT)
LANGUAGE SQL
AS $$
SELECT
    i.slug,
    i.title,
    i.private,
    ts_headline(
        'simple',
        coalesce(i.body, ''),
        websearch_to_tsquery('simple', query),
        'StartSel=<mark>, StopSel=</mark>, MaxWords=20, MinWords=8, ShortWord=2'
    ) AS snippet
FROM article_search_index i
WHERE
    i.trashed = false
    AND (include_private OR i.private = false)
    AND to_tsvector('simple', coalesce(i.slug, '') || ' ' || coalesce(i.title, '') || ' ' || coalesce(i.body, ''))
        @@ websearch_to_tsquery('simple', query)
ORDER BY
    ts_rank(
        to_tsvector('simple', coalesce(i.slug, '') || ' ' || coalesce(i.title, '') || ' ' || coalesce(i.body, '')),
        websearch_to_tsquery('simple', query)
    ) DESC,
    i.slug ASC;
$$;
