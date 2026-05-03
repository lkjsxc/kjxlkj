CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE IF NOT EXISTS platform_migrations (
    key TEXT PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM platform_migrations
        WHERE key = 'fresh_multi_user_schema'
    ) THEN
        DROP TABLE IF EXISTS api_tokens CASCADE;
        DROP TABLE IF EXISTS service_accounts CASCADE;
        DROP TABLE IF EXISTS audit_events CASCADE;
        DROP TABLE IF EXISTS external_embed_cache CASCADE;
        DROP TABLE IF EXISTS resource_daily_views CASCADE;
        DROP TABLE IF EXISTS resource_snapshots CASCADE;
        DROP TABLE IF EXISTS resources CASCADE;
        DROP TABLE IF EXISTS space_settings CASCADE;
        DROP TABLE IF EXISTS space_memberships CASCADE;
        DROP TABLE IF EXISTS spaces CASCADE;
        DROP TABLE IF EXISTS password_reset_tokens CASCADE;
        DROP TABLE IF EXISTS user_sessions CASCADE;
        DROP TABLE IF EXISTS user_local_credentials CASCADE;
        DROP TABLE IF EXISTS users CASCADE;
        DROP TABLE IF EXISTS app_settings CASCADE;
        DROP TABLE IF EXISTS sessions CASCADE;
        DROP TABLE IF EXISTS admin_user CASCADE;
        DROP TYPE IF EXISTS resource_visibility CASCADE;
        DROP TYPE IF EXISTS space_role CASCADE;
        INSERT INTO platform_migrations (key)
        VALUES ('fresh_multi_user_schema');
    END IF;
END $$;
