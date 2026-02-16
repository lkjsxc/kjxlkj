/// Note event construction helpers.
///
/// Extracted from routes_note.rs to keep modules under 200 lines.
/// Spec: /docs/spec/domain/events.md
use kjxlkj_domain::event::{ActorType, NoteEvent, NoteEventType};
use uuid::Uuid;

/// Build a NoteEvent with common defaults.
pub fn build_note_event(
    note_id: Uuid,
    seq: i64,
    event_type: NoteEventType,
    payload: serde_json::Value,
) -> NoteEvent {
    NoteEvent {
        id: Uuid::new_v4(),
        note_id,
        seq,
        event_type,
        actor_type: ActorType::User,
        actor_id: Uuid::nil(),
        payload,
        created_at: chrono::Utc::now().naive_utc(),
    }
}
