DO $$
BEGIN
    CREATE TYPE resource_visibility AS ENUM ('public', 'space', 'private');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS resources (
    id CHAR(26) PRIMARY KEY,
    space_id UUID NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
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
    visibility resource_visibility NOT NULL DEFAULT 'public',
    owner_user_id UUID REFERENCES users(id),
    created_by_user_id UUID REFERENCES users(id),
    updated_by_user_id UUID REFERENCES users(id),
    created_by_service_account_id UUID,
    updated_by_service_account_id UUID,
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
    ) STORED
);

CREATE TABLE IF NOT EXISTS resource_snapshots (
    id CHAR(26) PRIMARY KEY,
    space_id UUID NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
    resource_id CHAR(26) NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
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
    visibility resource_visibility NOT NULL,
    snapshot_number INTEGER NOT NULL,
    created_by_user_id UUID REFERENCES users(id),
    created_by_service_account_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(resource_id, snapshot_number)
);
