//! Apply-patch handler split from protocol.rs.
//! Per /docs/spec/api/websocket.md §ApplyPatch.

use kjxlkj_db::{repo_idempotency, repo_note};
use kjxlkj_domain::ids::{EventId, NoteId, UserId};
use sqlx::PgPool;
use uuid::Uuid;

use crate::messages::ServerMessage;
use crate::subscriptions::SubscriptionState;

/// Handle the ApplyPatch message from a WS client.
pub async fn handle_apply_patch(
    note_id: Uuid,
    base_version: i64,
    patch_ops: Vec<serde_json::Value>,
    idempotency_key: String,
    user_id: Uuid,
    pool: &PgPool,
    subs: &mut SubscriptionState,
) -> Vec<ServerMessage> {
    let nid = NoteId(note_id);
    let uid = UserId(user_id);
    let rid = Uuid::now_v7().to_string();

    // Idempotency check per /docs/spec/api/websocket.md.
    // Duplicate keys MUST replay-safe-return existing commit identity.
    if let Ok(Some(existing)) = repo_idempotency::find_idempotency(
        pool, nid, &idempotency_key,
    ).await {
        return vec![ServerMessage::PatchCommitted {
            note_id,
            version: existing.version,
            event_seq: existing.event_seq,
            idempotency_key,
        }];
    }

    // Fetch current stream
    let stream = match repo_note::find_note_stream(pool, nid).await {
        Ok(Some(s)) => s,
        _ => return vec![ServerMessage::Error {
            code: "NOTE_NOT_FOUND".into(),
            message: "note not found".into(),
            details: None,
            request_id: rid,
        }],
    };

    if stream.deleted_at.is_some() {
        return vec![ServerMessage::Error {
            code: "NOTE_NOT_FOUND".into(),
            message: "note deleted".into(),
            details: None,
            request_id: rid,
        }];
    }

    // Version conflict check
    if base_version != stream.current_version {
        return vec![ServerMessage::PatchRejected {
            note_id,
            expected_version: base_version,
            current_version: stream.current_version,
            reason: "VERSION_CONFLICT".into(),
        }];
    }

    let new_version = stream.current_version + 1;
    let event_id = EventId(Uuid::now_v7());
    let payload = serde_json::json!({ "ops": patch_ops });

    // Append event
    if repo_note::append_note_event(
        pool, event_id, nid, new_version, "patch", &payload, uid,
    ).await.is_err() {
        return vec![ServerMessage::Error {
            code: "INTERNAL_ERROR".into(),
            message: "failed to append event".into(),
            details: None,
            request_id: rid,
        }];
    }

    // Update projection
    if let Ok(Some(proj)) = repo_note::find_note_projection(pool, nid).await {
        let new_md = crate::apply_patch_ops(&proj.markdown, &patch_ops);
        let _ = repo_note::update_note_projection(
            pool, nid, &proj.title, new_version,
            &new_md, "", &proj.metadata_json,
        ).await;

        // Snapshot every 100 events
        if new_version % 100 == 0 {
            let _ = repo_note::store_snapshot(
                pool, nid, new_version, &new_md, &proj.metadata_json,
            ).await;
        }

        // Sync backlinks
        let links = kjxlkj_domain::backlink::extract_wiki_links(&new_md);
        let _ = kjxlkj_db::repo_backlink::sync_backlinks(
            pool, nid, &links,
        ).await;
    }

    // Store idempotency key
    let _ = repo_idempotency::store_idempotency(
        pool, nid, &idempotency_key,
        event_id.0, new_version, new_version,
    ).await;

    // Update subscription cursor
    if subs.is_subscribed_note(&note_id) {
        subs.subscribe_note(note_id, new_version);
    }

    vec![ServerMessage::PatchCommitted {
        note_id,
        version: new_version,
        event_seq: new_version,
        idempotency_key,
    }]
}
