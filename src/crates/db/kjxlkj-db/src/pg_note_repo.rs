/// PostgreSQL note repository per /docs/spec/domain/notes.md
///
/// Uses sqlx::PgPool for all operations. Row types in pg_rows.rs.
use crate::pg_rows::{pg_err, PgNoteEventRow, PgNoteProjection, PgNoteStreamRow};
use kjxlkj_domain::event::NoteEvent;
use kjxlkj_domain::note::*;
use kjxlkj_domain::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

/// PostgreSQL-backed note repository.
pub struct PgNoteRepo {
    pool: PgPool,
}

impl PgNoteRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_note(
        &self,
        stream: &NoteStream,
        proj: &NoteProjection,
        event: &NoteEvent,
    ) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await.map_err(pg_err)?;
        sqlx::query(
            "INSERT INTO note_streams (id, workspace_id, project_id, title, note_kind, access_scope, state, current_version, created_at, updated_at)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)")
            .bind(stream.id).bind(stream.workspace_id).bind(stream.project_id)
            .bind(&stream.title).bind(stream.note_kind.as_str())
            .bind(stream.access_scope.as_str()).bind(stream.state.as_str())
            .bind(stream.current_version).bind(stream.created_at).bind(stream.updated_at)
            .execute(&mut *tx).await.map_err(pg_err)?;
        sqlx::query(
            "INSERT INTO note_projections (note_id, title, version, markdown, metadata_json, updated_at)
             VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(proj.note_id).bind(&proj.title).bind(proj.version)
            .bind(&proj.markdown).bind(&proj.metadata_json).bind(proj.updated_at)
            .execute(&mut *tx).await.map_err(pg_err)?;
        insert_event(&mut tx, event).await?;
        tx.commit().await.map_err(pg_err)?;
        Ok(())
    }

    pub async fn get_note_projection(
        &self,
        id: Uuid,
    ) -> Result<Option<NoteProjection>, DomainError> {
        let row = sqlx::query_as::<_, PgNoteProjection>(
            "SELECT note_id, title, version, markdown, metadata_json, updated_at
             FROM note_projections WHERE note_id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await.map_err(pg_err)?;
        Ok(row.map(|r| r.into()))
    }

    pub async fn list_notes(
        &self,
        workspace_id: Uuid,
        include_deleted: bool,
    ) -> Result<Vec<PgNoteStreamRow>, DomainError> {
        let q = if include_deleted {
            "SELECT id, workspace_id, project_id, title, note_kind, access_scope, state, current_version, created_at, updated_at
             FROM note_streams WHERE workspace_id = $1 ORDER BY updated_at DESC"
        } else {
            "SELECT id, workspace_id, project_id, title, note_kind, access_scope, state, current_version, created_at, updated_at
             FROM note_streams WHERE workspace_id = $1 AND state = 'active' ORDER BY updated_at DESC"
        };
        let rows = sqlx::query_as::<_, PgNoteStreamRow>(q)
            .bind(workspace_id)
            .fetch_all(&self.pool).await.map_err(pg_err)?;
        Ok(rows)
    }

    pub async fn update_note(
        &self, id: Uuid, base_version: i64, markdown: Option<&str>,
        title: Option<&str>, event: &NoteEvent,
    ) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await.map_err(pg_err)?;
        let row = sqlx::query_scalar::<_, i64>(
            "SELECT current_version FROM note_streams WHERE id = $1 FOR UPDATE")
            .bind(id).fetch_optional(&mut *tx).await.map_err(pg_err)?;
        let current = row.ok_or(DomainError::NoteNotFound)?;
        if current != base_version {
            return Err(DomainError::VersionConflict { expected: base_version, actual: current });
        }
        let nv = base_version + 1;
        sqlx::query("UPDATE note_streams SET current_version=$1, updated_at=now() WHERE id=$2")
            .bind(nv).bind(id).execute(&mut *tx).await.map_err(pg_err)?;
        if let Some(md) = markdown {
            sqlx::query("UPDATE note_projections SET markdown=$1, version=$2, updated_at=now() WHERE note_id=$3")
                .bind(md).bind(nv).bind(id).execute(&mut *tx).await.map_err(pg_err)?;
        }
        if let Some(t) = title {
            sqlx::query("UPDATE note_projections SET title=$1, version=$2, updated_at=now() WHERE note_id=$3")
                .bind(t).bind(nv).bind(id).execute(&mut *tx).await.map_err(pg_err)?;
            sqlx::query("UPDATE note_streams SET title=$1 WHERE id=$2")
                .bind(t).bind(id).execute(&mut *tx).await.map_err(pg_err)?;
        }
        insert_event(&mut tx, event).await?;
        tx.commit().await.map_err(pg_err)?;
        Ok(())
    }

    pub async fn soft_delete_note(&self, id: Uuid, event: &NoteEvent) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await.map_err(pg_err)?;
        sqlx::query("UPDATE note_streams SET state='soft_deleted', updated_at=now() WHERE id=$1")
            .bind(id).execute(&mut *tx).await.map_err(pg_err)?;
        insert_event(&mut tx, event).await?;
        tx.commit().await.map_err(pg_err)?;
        Ok(())
    }

    pub async fn get_note_history(&self, id: Uuid) -> Result<Vec<PgNoteEventRow>, DomainError> {
        let rows = sqlx::query_as::<_, PgNoteEventRow>(
            "SELECT id, note_id, seq, event_type, actor_type, actor_id, payload, created_at
             FROM note_events WHERE note_id = $1 ORDER BY seq")
            .bind(id).fetch_all(&self.pool).await.map_err(pg_err)?;
        Ok(rows)
    }
}

/// Insert a note event within a transaction.
async fn insert_event(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, event: &NoteEvent,
) -> Result<(), DomainError> {
    sqlx::query(
        "INSERT INTO note_events (id, note_id, seq, event_type, actor_type, actor_id, payload, created_at)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8)")
        .bind(event.id).bind(event.note_id).bind(event.seq)
        .bind(event.event_type.as_str()).bind(event.actor_type.as_str())
        .bind(event.actor_id).bind(&event.payload).bind(event.created_at)
        .execute(&mut **tx).await.map_err(pg_err)?;
    Ok(())
}
