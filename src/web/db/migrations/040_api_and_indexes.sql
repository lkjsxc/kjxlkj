CREATE TABLE IF NOT EXISTS resource_daily_views (
    space_id UUID NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
    resource_id CHAR(26) NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    view_date DATE NOT NULL,
    view_count BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (space_id, resource_id, view_date)
);

CREATE TABLE IF NOT EXISTS external_embed_cache (
    space_id UUID NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
    url_hash CHAR(64) NOT NULL,
    url TEXT NOT NULL,
    canonical_url TEXT,
    provider TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('bookmark', 'image', 'video', 'audio', 'frame', 'social')),
    title TEXT,
    description TEXT,
    site_name TEXT,
    author_name TEXT,
    thumbnail_url TEXT,
    thumbnail_width INTEGER,
    thumbnail_height INTEGER,
    fetched_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    last_error TEXT,
    error_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (space_id, url_hash),
    UNIQUE(space_id, url)
);

CREATE TABLE IF NOT EXISTS service_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    space_id UUID NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_by UUID REFERENCES users(id),
    disabled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS api_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_account_id UUID NOT NULL REFERENCES service_accounts(id) ON DELETE CASCADE,
    space_id UUID NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    scopes JSONB NOT NULL DEFAULT '[]'::JSONB,
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS audit_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    space_id UUID REFERENCES spaces(id) ON DELETE SET NULL,
    actor_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    actor_service_account_id UUID REFERENCES service_accounts(id) ON DELETE SET NULL,
    event_type TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_resources_space_alias
    ON resources(space_id, alias)
    WHERE alias IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_resources_space_updated
    ON resources(space_id, updated_at DESC, id ASC) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_resources_space_favorites
    ON resources(space_id, favorite_position ASC, id ASC)
    WHERE deleted_at IS NULL AND is_favorite = TRUE;
CREATE INDEX IF NOT EXISTS idx_resources_search
    ON resources USING GIN(search_document);
CREATE INDEX IF NOT EXISTS idx_resource_daily_views_rank
    ON resource_daily_views(space_id, view_date, view_count DESC, resource_id);
CREATE INDEX IF NOT EXISTS idx_external_embed_cache_expires
    ON external_embed_cache(space_id, expires_at);
