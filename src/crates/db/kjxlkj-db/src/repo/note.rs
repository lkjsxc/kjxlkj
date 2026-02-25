//! Note repository - in-memory implementation

use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use kjxlkj_domain::{
    NoteStream, NoteId, ConcurrencyError,
    DomainEvent, EventSeq, Actor, NoteEvent,
};
use crate::error::{DbError, Result};

/// In-memory note repository
#[derive(Debug, Clone)]
pub struct NoteRepo {
    notes: Arc<RwLock<HashMap<NoteId, NoteStream>>>,
    events: Arc<RwLock<HashMap<NoteId, Vec<DomainEvent>>>>,
    event_seq: Arc<RwLock<u64>>,
}

impl NoteRepo {
    pub fn new() -> Self {
        Self {
            notes: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(HashMap::new())),
            event_seq: Arc::new(RwLock::new(0)),
        }
    }

    /// Create a new note
    pub async fn create(&self, mut note: NoteStream, actor: &Actor) -> Result<NoteStream> {
        let mut notes = self.notes.write().await;
        let mut events = self.events.write().await;
        let mut event_seq = self.event_seq.write().await;

        *event_seq += 1;
        let seq = *event_seq;

        // Create event
        let event = DomainEvent::new(
            note.note_id,
            NoteEvent::Created {
                title: note.title.clone(),
                markdown: note.markdown.clone(),
                workspace_id: note.workspace_id,
                project_id: note.project_id,
                note_kind: format!("{:?}", note.note_kind),
            },
            seq,
            note.version,
            actor.clone(),
        );

        notes.insert(note.note_id, note.clone());
        events.insert(note.note_id, vec![event]);

        Ok(note)
    }

    /// Get note by ID
    pub async fn get(&self, note_id: NoteId) -> Result<Option<NoteStream>> {
        let notes = self.notes.read().await;
        Ok(notes.get(&note_id).cloned())
    }

    /// Get note by ID with version check
    pub async fn get_with_version(
        &self,
        note_id: NoteId,
        expected_version: u64,
    ) -> Result<NoteStream> {
        let notes = self.notes.read().await;
        let note = notes
            .get(&note_id)
            .ok_or_else(|| DbError::NotFound(format!("Note {} not found", note_id)))?;

        if note.version != expected_version {
            return Err(DbError::ConcurrencyConflict {
                expected: expected_version,
                current: note.version,
            });
        }

        Ok(note.clone())
    }

    /// Update note with optimistic concurrency
    pub async fn update(
        &self,
        note_id: NoteId,
        expected_version: u64,
        mut update_fn: impl FnMut(&mut NoteStream) -> Option<NoteEvent>,
        actor: &Actor,
    ) -> Result<NoteStream> {
        let mut notes = self.notes.write().await;
        let mut events = self.events.write().await;
        let mut event_seq = self.event_seq.write().await;

        let note = notes
            .get_mut(&note_id)
            .ok_or_else(|| DbError::NotFound(format!("Note {} not found", note_id)))?;

        if note.version != expected_version {
            return Err(DbError::ConcurrencyConflict {
                expected: expected_version,
                current: note.version,
            });
        }

        if let Some(event_type) = update_fn(note) {
            *event_seq += 1;
            let seq = *event_seq;
            note.version += 1;
            note.updated_at = Utc::now();

            let event = DomainEvent::new(note_id, event_type, seq, note.version, actor.clone());
            events.entry(note_id).or_insert_with(Vec::new).push(event);
        }

        Ok(note.clone())
    }

    /// Delete note (soft delete)
    pub async fn delete(&self, note_id: NoteId, expected_version: u64, actor: &Actor) -> Result<()> {
        self.update(note_id, expected_version, |note| {
            note.delete();
            Some(NoteEvent::Deleted { deleted_at: Utc::now() })
        }, actor).await?;
        Ok(())
    }

    /// List notes for workspace
    pub async fn list_by_workspace(
        &self,
        workspace_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<NoteStream>> {
        let notes = self.notes.read().await;
        let mut filtered: Vec<_> = notes
            .values()
            .filter(|n| n.workspace_id == workspace_id && n.is_active())
            .cloned()
            .collect();

        // Sort by updated_at desc
        filtered.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        // Apply pagination
        let start = offset.min(filtered.len());
        let end = (offset + limit).min(filtered.len());

        Ok(filtered[start..end].to_vec())
    }

    /// Get event history for note
    pub async fn get_events(&self, note_id: NoteId) -> Result<Vec<DomainEvent>> {
        let events = self.events.read().await;
        Ok(events.get(&note_id).cloned().unwrap_or_default())
    }

    /// Get current event sequence
    pub async fn get_event_seq(&self) -> Result<u64> {
        let seq = self.event_seq.read().await;
        Ok(*seq)
    }
}

impl Default for NoteRepo {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;
