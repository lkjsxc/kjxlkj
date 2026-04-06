CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS pg_trgm;
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'records' AND column_name = 'id'
        AND character_maximum_length = 22
    ) THEN
        DROP TABLE IF EXISTS record_revisions;
        DROP TABLE IF EXISTS records;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.tables WHERE table_name = 'records'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'records' AND column_name IN ('alias', 'is_favorite')
        GROUP BY table_name HAVING COUNT(*) < 2
    ) THEN
        DROP TABLE IF EXISTS record_revisions;
        DROP TABLE IF EXISTS records;
    END IF;
END $$;
CREATE TABLE IF NOT EXISTS admin_user (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES admin_user(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
CREATE TABLE IF NOT EXISTS records (
    id CHAR(26) PRIMARY KEY,
    alias TEXT,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    body TEXT NOT NULL DEFAULT '',
    is_favorite BOOLEAN NOT NULL DEFAULT FALSE,
    favorite_position BIGINT,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    view_count_total BIGINT NOT NULL DEFAULT 0,
    last_viewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    search_document TSVECTOR GENERATED ALWAYS AS (
        setweight(to_tsvector('simple', COALESCE(alias, '')), 'A') ||
        setweight(to_tsvector('simple', COALESCE(title, '')), 'A') ||
        setweight(to_tsvector('simple', COALESCE(body, '')), 'B')
    ) STORED
);

ALTER TABLE records ADD COLUMN IF NOT EXISTS favorite_position BIGINT;
ALTER TABLE records ADD COLUMN IF NOT EXISTS view_count_total BIGINT NOT NULL DEFAULT 0;
ALTER TABLE records ADD COLUMN IF NOT EXISTS last_viewed_at TIMESTAMPTZ;

WITH ordered AS (
    SELECT id, ROW_NUMBER() OVER (ORDER BY updated_at DESC, id ASC) AS favorite_position
    FROM records
    WHERE deleted_at IS NULL AND is_favorite = TRUE AND favorite_position IS NULL
)
UPDATE records
SET favorite_position = ordered.favorite_position
FROM ordered
WHERE records.id = ordered.id;

CREATE INDEX IF NOT EXISTS idx_records_updated ON records(updated_at DESC, id ASC);
CREATE INDEX IF NOT EXISTS idx_records_created ON records(created_at ASC, id ASC);
CREATE INDEX IF NOT EXISTS idx_records_active ON records(deleted_at)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_records_favorite_position ON records(favorite_position ASC, id ASC)
    WHERE deleted_at IS NULL AND is_favorite = TRUE;
CREATE INDEX IF NOT EXISTS idx_records_search ON records USING GIN(search_document);
CREATE UNIQUE INDEX IF NOT EXISTS idx_records_alias ON records(alias)
    WHERE alias IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_records_alias_trgm ON records USING GIN(alias gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_records_title_trgm ON records USING GIN(title gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_records_body_trgm ON records USING GIN(body gin_trgm_ops);

CREATE TABLE IF NOT EXISTS record_revisions (
    id CHAR(26) PRIMARY KEY,
    record_id CHAR(26) NOT NULL REFERENCES records(id),
    alias TEXT,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    body TEXT NOT NULL,
    is_private BOOLEAN NOT NULL,
    snapshot_number INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(record_id, snapshot_number)
);

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'record_revisions' AND column_name = 'id'
        AND data_type = 'integer'
    ) THEN
        ALTER TABLE record_revisions DROP CONSTRAINT IF EXISTS record_revisions_pkey;
        ALTER TABLE record_revisions RENAME COLUMN id TO legacy_id;
    END IF;
END $$;

ALTER TABLE record_revisions ADD COLUMN IF NOT EXISTS id CHAR(26);
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'record_revisions' AND column_name = 'revision_number'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'record_revisions' AND column_name = 'snapshot_number'
    ) THEN
        ALTER TABLE record_revisions RENAME COLUMN revision_number TO snapshot_number;
    END IF;
END $$;
ALTER TABLE record_revisions ADD COLUMN IF NOT EXISTS snapshot_number INTEGER;
ALTER TABLE record_revisions ADD COLUMN IF NOT EXISTS alias TEXT;
ALTER TABLE record_revisions ADD COLUMN IF NOT EXISTS title TEXT;
ALTER TABLE record_revisions ADD COLUMN IF NOT EXISTS summary TEXT;

CREATE INDEX IF NOT EXISTS idx_revisions_lookup
    ON record_revisions(record_id, snapshot_number DESC);

CREATE TABLE IF NOT EXISTS record_daily_views (
    record_id CHAR(26) NOT NULL REFERENCES records(id) ON DELETE CASCADE,
    view_date DATE NOT NULL,
    view_count BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (record_id, view_date)
);

CREATE INDEX IF NOT EXISTS idx_record_daily_views_rank
    ON record_daily_views(view_date, view_count DESC, record_id);

CREATE TABLE IF NOT EXISTS app_settings (
    id SMALLINT PRIMARY KEY,
    home_recent_limit BIGINT NOT NULL DEFAULT 5,
    home_favorite_limit BIGINT NOT NULL DEFAULT 5,
    home_popular_limit BIGINT NOT NULL DEFAULT 5,
    home_intro_markdown TEXT NOT NULL DEFAULT '',
    home_recent_visible BOOLEAN NOT NULL DEFAULT TRUE,
    home_favorite_visible BOOLEAN NOT NULL DEFAULT TRUE,
    home_popular_visible BOOLEAN NOT NULL DEFAULT TRUE,
    home_recent_position BIGINT NOT NULL DEFAULT 2,
    home_favorite_position BIGINT NOT NULL DEFAULT 3,
    home_popular_position BIGINT NOT NULL DEFAULT 1,
    search_results_per_page BIGINT NOT NULL DEFAULT 20,
    session_timeout_minutes BIGINT NOT NULL DEFAULT 1440,
    default_new_note_is_private BOOLEAN NOT NULL DEFAULT FALSE,
    site_name TEXT NOT NULL DEFAULT 'kjxlkj',
    site_description TEXT NOT NULL DEFAULT 'Markdown note system for LLM-operated workflows.',
    public_base_url TEXT NOT NULL DEFAULT '',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS home_popular_limit BIGINT NOT NULL DEFAULT 5;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS home_intro_markdown TEXT NOT NULL DEFAULT '';
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS home_recent_visible BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS home_favorite_visible BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS home_popular_visible BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS home_recent_position BIGINT NOT NULL DEFAULT 2;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS home_favorite_position BIGINT NOT NULL DEFAULT 3;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS home_popular_position BIGINT NOT NULL DEFAULT 1;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS session_timeout_minutes BIGINT NOT NULL DEFAULT 1440;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS default_new_note_is_private BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS site_name TEXT NOT NULL DEFAULT 'kjxlkj';
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS site_description TEXT NOT NULL DEFAULT 'Markdown note system for LLM-operated workflows.';
ALTER TABLE app_settings
    ADD COLUMN IF NOT EXISTS public_base_url TEXT NOT NULL DEFAULT '';
ALTER TABLE app_settings
    DROP COLUMN IF EXISTS home_title;
ALTER TABLE records
    ALTER COLUMN is_private SET DEFAULT FALSE;
ALTER TABLE app_settings
    ALTER COLUMN home_recent_limit SET DEFAULT 5;
ALTER TABLE app_settings
    ALTER COLUMN home_favorite_limit SET DEFAULT 5;
ALTER TABLE app_settings
    ALTER COLUMN home_popular_limit SET DEFAULT 5;
ALTER TABLE app_settings
    ALTER COLUMN session_timeout_minutes SET DEFAULT 1440;
ALTER TABLE app_settings
    ALTER COLUMN default_new_note_is_private SET DEFAULT FALSE;
ALTER TABLE app_settings
    ALTER COLUMN site_name SET DEFAULT 'kjxlkj';
ALTER TABLE app_settings
    ALTER COLUMN site_description SET DEFAULT 'Markdown note system for LLM-operated workflows.';
ALTER TABLE app_settings
    ALTER COLUMN public_base_url SET DEFAULT '';
ALTER TABLE app_settings
    DROP COLUMN IF EXISTS default_vim_mode;

INSERT INTO app_settings (id) VALUES (1)
ON CONFLICT (id) DO NOTHING;
UPDATE app_settings SET default_new_note_is_private = FALSE WHERE id = 1;
UPDATE app_settings SET site_name = 'kjxlkj' WHERE id = 1 AND (site_name IS NULL OR btrim(site_name) = '');
UPDATE app_settings
SET site_description = 'Markdown note system for LLM-operated workflows.'
WHERE id = 1 AND (site_description IS NULL OR btrim(site_description) = '');
UPDATE app_settings
SET public_base_url = ''
WHERE id = 1 AND public_base_url IS NULL;
