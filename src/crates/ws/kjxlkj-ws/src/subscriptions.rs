/// Subscription state management per /docs/spec/api/websocket.md.
use std::collections::HashMap;
use uuid::Uuid;

/// Tracks per-session subscription state and ack cursors.
#[derive(Debug, Default)]
pub struct SubscriptionState {
    /// note_id → ack cursor (last confirmed event_seq)
    pub note_cursors: HashMap<Uuid, i64>,
    /// workspace_id → ack cursor
    pub workspace_cursors: HashMap<Uuid, i64>,
}

impl SubscriptionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe_note(&mut self, note_id: Uuid, initial_cursor: i64) {
        self.note_cursors.insert(note_id, initial_cursor);
    }

    pub fn unsubscribe_note(&mut self, note_id: &Uuid) {
        self.note_cursors.remove(note_id);
    }

    pub fn is_subscribed_note(&self, note_id: &Uuid) -> bool {
        self.note_cursors.contains_key(note_id)
    }

    pub fn subscribe_workspace(&mut self, ws_id: Uuid, initial_cursor: i64) {
        self.workspace_cursors.insert(ws_id, initial_cursor);
    }

    pub fn unsubscribe_workspace(&mut self, ws_id: &Uuid) {
        self.workspace_cursors.remove(ws_id);
    }

    /// Update ack cursor for a stream. Returns Ok(()) or Err with
    /// info for STALE_CURSOR error.
    pub fn ack(
        &mut self,
        stream_id: &str,
        event_seq: i64,
    ) -> Result<(), (String, i64, i64)> {
        if let Some(stripped) = stream_id.strip_prefix("note:") {
            if let Ok(id) = stripped.parse::<Uuid>() {
                if let Some(cursor) = self.note_cursors.get_mut(&id) {
                    if event_seq < *cursor {
                        return Err((
                            stream_id.to_string(),
                            event_seq,
                            *cursor,
                        ));
                    }
                    *cursor = event_seq;
                    return Ok(());
                }
            }
        }
        if let Some(stripped) = stream_id.strip_prefix("workspace:") {
            if let Ok(id) = stripped.parse::<Uuid>() {
                if let Some(cursor) = self.workspace_cursors.get_mut(&id) {
                    if event_seq < *cursor {
                        return Err((
                            stream_id.to_string(),
                            event_seq,
                            *cursor,
                        ));
                    }
                    *cursor = event_seq;
                    return Ok(());
                }
            }
        }
        Ok(()) // Unknown stream — no-op
    }

    pub fn note_cursor(&self, note_id: &Uuid) -> i64 {
        self.note_cursors.get(note_id).copied().unwrap_or(0)
    }

    pub fn workspace_cursor(&self, ws_id: &Uuid) -> i64 {
        self.workspace_cursors.get(ws_id).copied().unwrap_or(0)
    }
}
