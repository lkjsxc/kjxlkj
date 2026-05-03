DO $$
BEGIN
    CREATE TYPE resource_visibility AS ENUM ('public', 'space', 'private');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS spaces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug CITEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    owner_user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS space_memberships (
    space_id UUID NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role space_role NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (space_id, user_id)
);

CREATE TABLE IF NOT EXISTS space_settings (
    space_id UUID PRIMARY KEY REFERENCES spaces(id) ON DELETE CASCADE,
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
    default_new_resource_visibility resource_visibility NOT NULL DEFAULT 'public',
    media_webp_quality BIGINT NOT NULL DEFAULT 82,
    site_name TEXT NOT NULL DEFAULT 'kjxlkj',
    site_description TEXT NOT NULL DEFAULT 'Markdown-first resource system for LLM-operated workflows.',
    public_base_url TEXT NOT NULL DEFAULT '',
    nostr_names JSONB NOT NULL DEFAULT '{}'::JSONB,
    nostr_relays JSONB NOT NULL DEFAULT '[]'::JSONB,
    live_default_source TEXT NOT NULL DEFAULT 'camera',
    live_default_camera_facing TEXT NOT NULL DEFAULT 'environment',
    live_default_height BIGINT NOT NULL DEFAULT 1080,
    live_default_fps BIGINT NOT NULL DEFAULT 60,
    live_default_microphone_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    site_icon_key TEXT,
    site_icon_content_type TEXT,
    site_icon_updated_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
