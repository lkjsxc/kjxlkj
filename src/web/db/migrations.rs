//! Database migrations

use super::DbPool;
use crate::error::AppError;

/// Run database migrations
pub async fn run_migrations(pool: &DbPool) -> Result<(), AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(format!("Connection failed: {e}")))?;

    client
        .batch_execute(
            r#"
            -- Admin user table
            CREATE TABLE IF NOT EXISTS admin_user (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                username VARCHAR(255) NOT NULL UNIQUE,
                password_hash VARCHAR(255) NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            -- Sessions table
            CREATE TABLE IF NOT EXISTS sessions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL REFERENCES admin_user(id) ON DELETE CASCADE,
                expires_at TIMESTAMPTZ NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);
            CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);

            -- Records table (notes)
            CREATE TABLE IF NOT EXISTS records (
                slug VARCHAR(64) PRIMARY KEY,
                body TEXT NOT NULL DEFAULT '',
                is_private BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                deleted_at TIMESTAMPTZ
            );

            CREATE INDEX IF NOT EXISTS idx_records_updated ON records(updated_at DESC);
            CREATE INDEX IF NOT EXISTS idx_records_created ON records(created_at ASC, slug ASC);
            CREATE INDEX IF NOT EXISTS idx_records_active ON records(deleted_at)
                WHERE deleted_at IS NULL;

            -- Record revisions table
            CREATE TABLE IF NOT EXISTS record_revisions (
                id SERIAL PRIMARY KEY,
                record_slug VARCHAR(64) NOT NULL,
                body TEXT NOT NULL,
                is_private BOOLEAN NOT NULL,
                revision_number INTEGER NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                UNIQUE(record_slug, revision_number)
            );

            CREATE INDEX IF NOT EXISTS idx_revisions_slug
                ON record_revisions(record_slug, revision_number DESC);
            "#,
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Migration failed: {e}")))?;

    Ok(())
}
