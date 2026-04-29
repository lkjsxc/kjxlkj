CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS pg_trgm;

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

CREATE TABLE IF NOT EXISTS resources (
    id CHAR(26) PRIMARY KEY,
    kind TEXT NOT NULL CHECK (kind IN ('note', 'media')),
    alias TEXT,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    body TEXT NOT NULL DEFAULT '',
    media_family TEXT CHECK (media_family IN ('image', 'video', 'file')),
    file_key TEXT,
    content_type TEXT,
    byte_size BIGINT,
    sha256_hex TEXT,
    original_filename TEXT,
    width INTEGER,
    height INTEGER,
    duration_ms BIGINT,
    media_variants JSONB,
    owner_note_id CHAR(26) REFERENCES resources(id),
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
        setweight(to_tsvector('simple', COALESCE(body, '')), 'B') ||
        setweight(to_tsvector('simple', COALESCE(original_filename, '')), 'B') ||
        setweight(to_tsvector('simple', kind), 'C')
    ) STORED,
    CHECK (
        (kind = 'note' AND media_family IS NULL AND file_key IS NULL AND content_type IS NULL
         AND byte_size IS NULL AND sha256_hex IS NULL AND original_filename IS NULL
         AND width IS NULL AND height IS NULL AND duration_ms IS NULL AND owner_note_id IS NULL)
        OR
        (kind = 'media' AND media_family IS NOT NULL AND file_key IS NOT NULL
         AND content_type IS NOT NULL AND byte_size IS NOT NULL AND sha256_hex IS NOT NULL
         AND original_filename IS NOT NULL)
    )
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_resources_alias ON resources(alias)
    WHERE alias IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_resources_updated ON resources(updated_at DESC, id ASC);
CREATE INDEX IF NOT EXISTS idx_resources_created ON resources(created_at ASC, id ASC);
CREATE INDEX IF NOT EXISTS idx_resources_active ON resources(deleted_at)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_resources_favorite_position
    ON resources(favorite_position ASC, id ASC)
    WHERE deleted_at IS NULL AND is_favorite = TRUE;
CREATE INDEX IF NOT EXISTS idx_resources_search ON resources USING GIN(search_document);
CREATE INDEX IF NOT EXISTS idx_resources_alias_trgm ON resources USING GIN(alias gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_resources_title_trgm ON resources USING GIN(title gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_resources_body_trgm ON resources USING GIN(body gin_trgm_ops);

CREATE TABLE IF NOT EXISTS resource_snapshots (
    id CHAR(26) PRIMARY KEY,
    resource_id CHAR(26) NOT NULL REFERENCES resources(id),
    kind TEXT NOT NULL CHECK (kind IN ('note', 'media')),
    alias TEXT,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    body TEXT NOT NULL,
    media_family TEXT CHECK (media_family IN ('image', 'video', 'file')),
    file_key TEXT,
    content_type TEXT,
    byte_size BIGINT,
    sha256_hex TEXT,
    original_filename TEXT,
    width INTEGER,
    height INTEGER,
    duration_ms BIGINT,
    media_variants JSONB,
    owner_note_id CHAR(26) REFERENCES resources(id),
    is_private BOOLEAN NOT NULL,
    snapshot_number INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(resource_id, snapshot_number)
);

CREATE INDEX IF NOT EXISTS idx_resource_snapshots_lookup
    ON resource_snapshots(resource_id, snapshot_number DESC);

CREATE TABLE IF NOT EXISTS resource_daily_views (
    resource_id CHAR(26) NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    view_date DATE NOT NULL,
    view_count BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (resource_id, view_date)
);

CREATE INDEX IF NOT EXISTS idx_resource_daily_views_rank
    ON resource_daily_views(view_date, view_count DESC, resource_id);

CREATE TABLE IF NOT EXISTS app_settings (
    id SMALLINT PRIMARY KEY,
    home_recent_limit BIGINT NOT NULL DEFAULT 5,
    home_favorite_limit BIGINT NOT NULL DEFAULT 5,
    home_popular_limit BIGINT NOT NULL DEFAULT 5,
    home_intro_markdown TEXT NOT NULL DEFAULT '',
    home_recent_visible BOOLEAN NOT NULL DEFAULT TRUE,
    home_favorite_visible BOOLEAN NOT NULL DEFAULT TRUE,
    home_popular_visible BOOLEAN NOT NULL DEFAULT TRUE,
    home_recent_position BIGINT NOT NULL DEFAULT 1,
    home_favorite_position BIGINT NOT NULL DEFAULT 2,
    home_popular_position BIGINT NOT NULL DEFAULT 3,
    search_results_per_page BIGINT NOT NULL DEFAULT 20,
    session_timeout_minutes BIGINT NOT NULL DEFAULT 1440,
    default_new_resource_is_private BOOLEAN NOT NULL DEFAULT FALSE,
    media_webp_quality BIGINT NOT NULL DEFAULT 82,
    site_name TEXT NOT NULL DEFAULT 'kjxlkj',
    site_description TEXT NOT NULL DEFAULT 'Markdown-first resource system for LLM-operated workflows.',
    public_base_url TEXT NOT NULL DEFAULT '',
    nostr_names JSONB NOT NULL DEFAULT '{}'::JSONB,
    nostr_relays JSONB NOT NULL DEFAULT '[]'::JSONB,
    live_default_source TEXT NOT NULL DEFAULT 'camera' CHECK (live_default_source IN ('screen', 'camera')),
    live_default_camera_facing TEXT NOT NULL DEFAULT 'environment' CHECK (live_default_camera_facing IN ('environment', 'user')),
    live_default_height BIGINT NOT NULL DEFAULT 1080 CHECK (live_default_height IN (360, 480, 720, 1080, 1440, 2160)),
    live_default_fps BIGINT NOT NULL DEFAULT 60 CHECK (live_default_fps IN (15, 30, 45, 60, 120)),
    live_default_microphone_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    google_maps_embed_api_key TEXT NOT NULL DEFAULT '',
    site_icon_key TEXT,
    site_icon_content_type TEXT,
    site_icon_updated_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE resources ADD COLUMN IF NOT EXISTS media_variants JSONB;
ALTER TABLE resources ADD COLUMN IF NOT EXISTS owner_note_id CHAR(26) REFERENCES resources(id);
ALTER TABLE resource_snapshots ADD COLUMN IF NOT EXISTS media_variants JSONB;
ALTER TABLE resource_snapshots ADD COLUMN IF NOT EXISTS owner_note_id CHAR(26) REFERENCES resources(id);
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS media_webp_quality BIGINT NOT NULL DEFAULT 82;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS nostr_names JSONB NOT NULL DEFAULT '{}'::JSONB;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS nostr_relays JSONB NOT NULL DEFAULT '[]'::JSONB;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS live_default_source TEXT NOT NULL DEFAULT 'camera';
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'app_settings' AND column_name = 'live_default_camera_facing'
    ) THEN
        ALTER TABLE app_settings ADD COLUMN live_default_camera_facing TEXT NOT NULL DEFAULT 'environment';
        UPDATE app_settings SET live_default_source = 'camera' WHERE live_default_source = 'screen';
    END IF;
END $$;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS live_default_height BIGINT NOT NULL DEFAULT 1080;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS live_default_fps BIGINT NOT NULL DEFAULT 60;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS live_default_microphone_enabled BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS google_maps_embed_api_key TEXT NOT NULL DEFAULT '';
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS site_icon_key TEXT;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS site_icon_content_type TEXT;
ALTER TABLE app_settings ADD COLUMN IF NOT EXISTS site_icon_updated_at TIMESTAMPTZ;

CREATE TABLE IF NOT EXISTS password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES admin_user(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE password_reset_tokens ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES admin_user(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_password_reset_tokens_active
    ON password_reset_tokens(expires_at)
    WHERE used_at IS NULL;

INSERT INTO app_settings (id) VALUES (1)
ON CONFLICT (id) DO NOTHING;
