/// In-memory NoteRepo implementation for testing and development.
///
/// Spec: /docs/spec/domain/notes.md (write rules, version conflict)
/// Spec: /docs/spec/domain/events.md (append-only events)
use crate::repo::NoteRepo;
use kjxlkj_domain::event::NoteEvent;
use kjxlkj_domain::note::*;
use kjxlkj_domain::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// Thread-safe in-memory note store.
pub struct InMemoryNoteRepo {
    streams: RwLock<HashMap<Uuid, NoteStream>>,
    projections: RwLock<HashMap<Uuid, NoteProjection>>,
    events: RwLock<HashMap<Uuid, Vec<NoteEvent>>>,
}

impl InMemoryNoteRepo {
    pub fn new() -> Self {
        Self {
            streams: RwLock::new(HashMap::new()),
            projections: RwLock::new(HashMap::new()),
            events: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryNoteRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl NoteRepo for InMemoryNoteRepo {
    fn create_note(
        &self,
        stream: &NoteStream,
        projection: &NoteProjection,
        event: &NoteEvent,
    ) -> Result<(), DomainError> {
        let mut streams = self.streams.write().unwrap();
        let mut projs = self.projections.write().unwrap();
        let mut evts = self.events.write().unwrap();
        streams.insert(stream.id, stream.clone());
        projs.insert(projection.note_id, projection.clone());
        evts.entry(stream.id).or_default().push(event.clone());
        Ok(())
    }

    fn get_note_stream(&self, id: Uuid) -> Result<Option<NoteStream>, DomainError> {
        let streams = self.streams.read().unwrap();
        Ok(streams.get(&id).cloned())
    }

    fn get_note_projection(&self, id: Uuid) -> Result<Option<NoteProjection>, DomainError> {
        let projs = self.projections.read().unwrap();
        Ok(projs.get(&id).cloned())
    }

    fn list_notes(
        &self,
        workspace_id: Uuid,
        include_deleted: bool,
    ) -> Result<Vec<NoteStream>, DomainError> {
        let streams = self.streams.read().unwrap();
        let results: Vec<NoteStream> = streams
            .values()
            .filter(|s| {
                s.workspace_id == workspace_id
                    && (include_deleted || s.state == NoteState::Active)
            })
            .cloned()
            .collect();
        Ok(results)
    }

    fn update_note(
        &self,
        id: Uuid,
        base_version: i64,
        markdown: Option<&str>,
        title: Option<&str>,
        event: &NoteEvent,
    ) -> Result<NoteProjection, DomainError> {
        let mut streams = self.streams.write().unwrap();
        let mut projs = self.projections.write().unwrap();
        let mut evts = self.events.write().unwrap();

        let stream = streams
            .get_mut(&id)
            .ok_or(DomainError::NoteNotFound)?;
        if stream.current_version != base_version {
            return Err(DomainError::VersionConflict {
                expected: base_version,
                actual: stream.current_version,
            });
        }
        stream.current_version += 1;
        stream.updated_at = chrono::Utc::now().naive_utc();
        if let Some(t) = title {
            stream.title = t.to_string();
        }

        let proj = projs
            .get_mut(&id)
            .ok_or(DomainError::NoteNotFound)?;
        proj.version = stream.current_version;
        proj.updated_at = stream.updated_at;
        if let Some(md) = markdown {
            proj.markdown = md.to_string();
        }
        if let Some(t) = title {
            proj.title = t.to_string();
        }

        evts.entry(id).or_default().push(event.clone());
        Ok(proj.clone())
    }

    fn soft_delete_note(&self, id: Uuid, event: &NoteEvent) -> Result<(), DomainError> {
        let mut streams = self.streams.write().unwrap();
        let mut evts = self.events.write().unwrap();
        let stream = streams
            .get_mut(&id)
            .ok_or(DomainError::NoteNotFound)?;
        stream.state = NoteState::SoftDeleted;
        stream.updated_at = chrono::Utc::now().naive_utc();
        evts.entry(id).or_default().push(event.clone());
        Ok(())
    }

    fn get_note_history(&self, id: Uuid) -> Result<Vec<NoteEvent>, DomainError> {
        let evts = self.events.read().unwrap();
        Ok(evts.get(&id).cloned().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_domain::event::{ActorType, NoteEventType};

    fn evt(note_id: Uuid, seq: i64, t: NoteEventType) -> NoteEvent {
        NoteEvent {
            id: Uuid::new_v4(), note_id, seq, event_type: t,
            actor_type: ActorType::User, actor_id: Uuid::new_v4(),
            payload: serde_json::json!({}), created_at: chrono::Utc::now().naive_utc(),
        }
    }
    fn stream(id: Uuid, ws: Uuid, title: &str) -> NoteStream {
        let now = chrono::Utc::now().naive_utc();
        NoteStream {
            id, workspace_id: ws, project_id: None, title: title.into(),
            note_kind: NoteKind::Markdown, access_scope: AccessScope::Workspace,
            state: NoteState::Active, current_version: 1, created_at: now, updated_at: now,
        }
    }
    fn proj(id: Uuid, title: &str, md: &str) -> NoteProjection {
        NoteProjection {
            note_id: id, title: title.into(), version: 1, markdown: md.into(),
            metadata_json: serde_json::json!({}), updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    #[test]
    fn test_create_and_get() {
        let repo = InMemoryNoteRepo::new();
        let (id, ws) = (Uuid::new_v4(), Uuid::new_v4());
        repo.create_note(&stream(id, ws, "Test"), &proj(id, "Test", "# Hello"),
            &evt(id, 1, NoteEventType::Created)).unwrap();
        assert_eq!(repo.get_note_stream(id).unwrap().unwrap().title, "Test");
        assert_eq!(repo.get_note_projection(id).unwrap().unwrap().markdown, "# Hello");
    }

    #[test]
    fn test_version_conflict() {
        let repo = InMemoryNoteRepo::new();
        let id = Uuid::new_v4();
        repo.create_note(&stream(id, Uuid::new_v4(), "T"), &proj(id, "T", ""),
            &evt(id, 1, NoteEventType::Created)).unwrap();
        let result = repo.update_note(id, 99, Some("new"), None,
            &evt(id, 2, NoteEventType::BodyUpdated));
        assert!(matches!(result, Err(DomainError::VersionConflict { .. })));
    }
}
