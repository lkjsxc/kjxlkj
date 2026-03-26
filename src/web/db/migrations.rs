//! Database migrations

use super::DbPool;
use crate::error::AppError;

pub async fn run_migrations(pool: &DbPool) -> Result<(), AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(format!("Connection failed: {e}")))?;

    client
        .batch_execute(
            r#"
            CREATE EXTENSION IF NOT EXISTS pgcrypto;

            DO $$
            BEGIN
                IF EXISTS (
                    SELECT 1 FROM information_schema.columns
                    WHERE table_name = 'records' AND column_name = 'slug'
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
                id CHAR(22) PRIMARY KEY,
                title TEXT NOT NULL,
                summary TEXT NOT NULL,
                body TEXT NOT NULL DEFAULT '',
                is_private BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                deleted_at TIMESTAMPTZ,
                search_document TSVECTOR GENERATED ALWAYS AS (
                    setweight(to_tsvector('simple', COALESCE(title, '')), 'A') ||
                    setweight(to_tsvector('simple', COALESCE(body, '')), 'B')
                ) STORED
            );

            CREATE INDEX IF NOT EXISTS idx_records_updated ON records(updated_at DESC, id ASC);
            CREATE INDEX IF NOT EXISTS idx_records_created ON records(created_at ASC, id ASC);
            CREATE INDEX IF NOT EXISTS idx_records_active ON records(deleted_at)
                WHERE deleted_at IS NULL;
            CREATE INDEX IF NOT EXISTS idx_records_search ON records USING GIN(search_document);

            CREATE TABLE IF NOT EXISTS record_revisions (
                id SERIAL PRIMARY KEY,
                record_id CHAR(22) NOT NULL REFERENCES records(id),
                body TEXT NOT NULL,
                is_private BOOLEAN NOT NULL,
                revision_number INTEGER NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                UNIQUE(record_id, revision_number)
            );

            CREATE INDEX IF NOT EXISTS idx_revisions_lookup
                ON record_revisions(record_id, revision_number DESC);
            "#,
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Migration failed: {e}")))?;

    Ok(())
}
