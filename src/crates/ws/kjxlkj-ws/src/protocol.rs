/// WS protocol dispatch per /docs/spec/api/websocket.md.
/// Handles subscribe/unsubscribe, ack, presence, and delegates
/// patch application to protocol_patch module.
use kjxlkj_db::repo_note;
use kjxlkj_domain::ids::NoteId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::automation_events;
use crate::messages::{ClientMessage, ServerMessage};
use crate::subscriptions::SubscriptionState;

/// Handle a parsed client message and return server response(s).
pub async fn handle_message(
    msg: ClientMessage,
    user_id: Uuid,
    pool: &PgPool,
    subs: &mut SubscriptionState,
    _replay_batch: i64,
) -> Vec<ServerMessage> {
    match msg {
        ClientMessage::SubscribeNote { note_id } => {
            handle_subscribe_note(note_id, pool, subs).await
        }
        ClientMessage::UnsubscribeNote { note_id } => {
            subs.unsubscribe_note(&note_id);
            vec![]
        }
        ClientMessage::SubscribeWorkspace { workspace_id } => {
            handle_subscribe_workspace(workspace_id, pool, subs).await
        }
        ClientMessage::UnsubscribeWorkspace { workspace_id } => {
            subs.unsubscribe_workspace(&workspace_id);
            vec![]
        }
        ClientMessage::ApplyPatch {
            note_id, base_version, patch_ops,
            idempotency_key, ..
        } => {
            crate::protocol_patch::handle_apply_patch(
                note_id, base_version, patch_ops,
                idempotency_key, user_id, pool, subs,
            ).await
        }
        ClientMessage::Ack { stream_id, event_seq } => {
            handle_ack(stream_id, event_seq, subs)
        }
        ClientMessage::PresencePing { .. } => {
            // Presence events MAY be lossy per spec — no-op.
            vec![]
        }
    }
}

/// Subscribe to a note stream with cursor-based replay.
/// Per /docs/spec/api/websocket.md §Ordering and Replay Rules.
async fn handle_subscribe_note(
    note_id: Uuid,
    pool: &PgPool,
    subs: &mut SubscriptionState,
) -> Vec<ServerMessage> {
    let nid = NoteId(note_id);
    let stream = match repo_note::find_note_stream(pool, nid).await {
        Ok(Some(s)) => s,
        _ => return vec![ServerMessage::Error {
            code: "NOTE_NOT_FOUND".into(),
            message: "note not found".into(),
            details: None,
            request_id: Uuid::now_v7().to_string(),
        }],
    };

    let current_version = stream.current_version;
    let cursor = subs.note_cursor(&note_id);
    subs.subscribe_note(note_id, cursor);

    let stream_id = format!("note:{note_id}");
    let mut msgs = vec![ServerMessage::Subscribed {
        stream_id,
        current_version,
        replay_cursor: cursor,
    }];

    // Replay missed events from cursor per event sourcing contract.
    // Replays MUST follow commit order (monotonic seq).
    if let Ok(events) = repo_note::list_note_events_from(
        pool, nid, cursor, 100,
    ).await {
        for ev in events {
            msgs.push(ServerMessage::NoteEvent {
                note_id: ev.note_id,
                event_seq: ev.seq,
                version: ev.seq,
                event_type: ev.event_type,
                payload: ev.payload_json,
            });
        }
    }

    msgs
}

/// Subscribe to a workspace stream with cursor-based replay.
/// Detects automation-prefixed events and emits them as
/// AutomationEvent messages per /docs/spec/api/websocket.md.
async fn handle_subscribe_workspace(
    workspace_id: Uuid,
    pool: &PgPool,
    subs: &mut SubscriptionState,
) -> Vec<ServerMessage> {
    let cursor = subs.workspace_cursor(&workspace_id);
    subs.subscribe_workspace(workspace_id, cursor);

    let stream_id = format!("workspace:{workspace_id}");
    let mut msgs = vec![ServerMessage::Subscribed {
        stream_id,
        current_version: 0,
        replay_cursor: cursor,
    }];

    // Replay workspace events with automation event detection.
    // Per spec: ordered interleave of note and automation events
    // within workspace stream, all sharing monotonic event_seq.
    let ws_id = kjxlkj_domain::ids::WorkspaceId(workspace_id);
    if let Ok(events) =
        kjxlkj_db::repo_workspace_event::list_workspace_events_from(
            pool, ws_id, cursor, 100,
        ).await
    {
        for ev in events {
            if let Some(auto_msg) = automation_events::classify_event(&ev) {
                msgs.push(auto_msg);
            } else {
                msgs.push(ServerMessage::WorkspaceEvent {
                    workspace_id: ev.workspace_id,
                    event_seq: ev.seq,
                    event_type: ev.event_type,
                    payload: ev.payload_json,
                });
            }
        }
    }

    msgs
}

/// Handle ack cursor advancement with stale-cursor detection.
/// Per /docs/spec/api/websocket.md: stale `ack` cursor regressions
/// MUST return error with code=STALE_CURSOR and include stream_id,
/// attempted event_seq, and current_cursor.
fn handle_ack(
    stream_id: String,
    event_seq: i64,
    subs: &mut SubscriptionState,
) -> Vec<ServerMessage> {
    match subs.ack(&stream_id, event_seq) {
        Ok(()) => vec![],
        Err((sid, attempted, current)) => {
            vec![ServerMessage::Error {
                code: "STALE_CURSOR".into(),
                message: format!(
                    "stale cursor: stream {sid}, attempted {attempted}, \
                     current {current}"
                ),
                details: Some(serde_json::json!({
                    "stream_id": sid,
                    "attempted_seq": attempted,
                    "current_cursor": current,
                })),
                request_id: Uuid::now_v7().to_string(),
            }]
        }
    }
}
