use std::collections::{HashMap, HashSet};

use axum::{
    extract::{ws::Message, ws::WebSocket, ws::WebSocketUpgrade, State},
    http::HeaderMap,
    response::IntoResponse,
};
use futures_util::StreamExt;
use serde_json::{json, Value};
use tokio::time::{interval, Duration};

use crate::{
    auth::{request_id, require_auth},
    error::ApiError,
    state::{AppState, WsEnvelope},
};

pub async fn ws_upgrade(
    State(state): State<AppState>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid).await?;
    Ok(ws.on_upgrade(move |socket| serve_socket(state, socket, identity.user_id)))
}

async fn serve_socket(state: AppState, mut socket: WebSocket, user_id: String) {
    let mut subscriptions = HashSet::<String>::new();
    let mut cursors = HashMap::<String, u64>::new();
    let mut ws_rx = state.ws_tx.subscribe();
    let mut heartbeat = interval(Duration::from_secs(20));

    loop {
        tokio::select! {
            incoming = socket.next() => {
                match incoming {
                    Some(Ok(Message::Text(payload))) => {
                        if handle_client_message(&state, &user_id, payload.to_string(), &mut subscriptions, &mut cursors, &mut socket).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            Ok(WsEnvelope { stream_id, payload }) = ws_rx.recv() => {
                if subscriptions.contains(&stream_id) {
                    if socket.send(Message::Text(payload.to_string().into())).await.is_err() {
                        break;
                    }
                }
            }
            _ = heartbeat.tick() => {
                let payload = json!({"type":"heartbeat","server_ts":request_id(&HeaderMap::new())});
                if socket.send(Message::Text(payload.to_string().into())).await.is_err() {
                    break;
                }
            }
        }
    }
}

async fn handle_client_message(
    state: &AppState,
    user_id: &str,
    raw: String,
    subscriptions: &mut HashSet<String>,
    cursors: &mut HashMap<String, u64>,
    socket: &mut WebSocket,
) -> Result<(), ()> {
    let value: Value = match serde_json::from_str(&raw) {
        Ok(value) => value,
        Err(_) => {
            send_socket(socket, json!({"type":"error","code":"BAD_REQUEST","message":"invalid json","request_id":"ws"})).await?;
            return Ok(());
        }
    };

    match value.get("type").and_then(Value::as_str).unwrap_or_default() {
        "subscribe_note" => {
            let note_id = value.get("note_id").and_then(Value::as_str).unwrap_or_default().to_string();
            let stream_id = format!("note:{note_id}");
            let replay_cursor = value
                .get("ack_cursor")
                .and_then(Value::as_u64)
                .unwrap_or_else(|| *cursors.get(&stream_id).unwrap_or(&0));
            subscriptions.insert(stream_id.clone());
            cursors.insert(stream_id.clone(), replay_cursor);
            let store = state.store.read().await;
            let current_version = store.notes.get(&note_id).map(|note| note.current_version).unwrap_or(0);
            send_socket(socket, json!({"type":"subscribed","stream_id":stream_id,"current_version":current_version,"replay_cursor":replay_cursor})).await?;
            for replay in store.replay_after(&format!("note:{note_id}"), replay_cursor) {
                send_socket(socket, replay).await?;
            }
        }
        "subscribe_workspace" => {
            let workspace_id = value.get("workspace_id").and_then(Value::as_str).unwrap_or_default().to_string();
            let stream_id = format!("workspace:{workspace_id}");
            let replay_cursor = value
                .get("ack_cursor")
                .and_then(Value::as_u64)
                .unwrap_or_else(|| *cursors.get(&stream_id).unwrap_or(&0));
            subscriptions.insert(stream_id.clone());
            cursors.insert(stream_id.clone(), replay_cursor);
            send_socket(socket, json!({"type":"subscribed","stream_id":stream_id,"current_version":0,"replay_cursor":replay_cursor})).await?;
        }
        "ack" => {
            let stream_id = value.get("stream_id").and_then(Value::as_str).unwrap_or_default().to_string();
            let attempted = value.get("event_seq").and_then(Value::as_u64).unwrap_or_default();
            let current = *cursors.get(&stream_id).unwrap_or(&0);
            if attempted < current {
                send_socket(socket, json!({"type":"error","code":"STALE_CURSOR","message":"cursor regression","request_id":"ws","stream_id":stream_id,"event_seq":attempted,"current_cursor":current})).await?;
            } else {
                cursors.insert(stream_id, attempted);
            }
        }
        "apply_patch" => {
            apply_patch_ws(state, value, user_id.to_string(), socket).await?;
        }
        "presence_ping" => {
            let workspace_id = value.get("workspace_id").and_then(Value::as_str).unwrap_or_default();
            let note_id = value.get("note_id").and_then(Value::as_str).unwrap_or_default();
            let payload = json!({"type":"presence_event","workspace_id":workspace_id,"note_id":note_id,"user_id":user_id,"state":"active","server_ts":"now"});
            let _ = state.ws_tx.send(WsEnvelope { stream_id: format!("workspace:{workspace_id}"), payload });
        }
        _ => {
            send_socket(socket, json!({"type":"error","code":"BAD_REQUEST","message":"unknown message type","request_id":"ws"})).await?;
        }
    }
    Ok(())
}

async fn apply_patch_ws(state: &AppState, value: Value, user_id: String, socket: &mut WebSocket) -> Result<(), ()> {
    let note_id = value.get("note_id").and_then(Value::as_str).unwrap_or_default().to_string();
    let base_version = value.get("base_version").and_then(Value::as_u64).unwrap_or_default();
    let idempotency_key = value.get("idempotency_key").and_then(Value::as_str).unwrap_or_default().to_string();

    let mut store = state.store.write().await;
    if let Some((version, event_seq, current_version)) = {
        store
            .notes
            .get(&note_id)
            .and_then(|note| {
                note.idempotency
                    .get(&idempotency_key)
                    .map(|(version, event_seq)| (*version, *event_seq, note.current_version))
            })
    } {
        let _ = current_version;
        return send_socket(socket, json!({"type":"patch_committed","note_id":note_id,"version":version,"event_seq":event_seq,"idempotency_key":idempotency_key})).await;
    }

    let version = {
        let note = match store.notes.get_mut(&note_id) {
            Some(note) => note,
            None => return send_socket(socket, json!({"type":"error","code":"NOTE_NOT_FOUND","message":"note not found","request_id":"ws"})).await,
        };

        if base_version != note.current_version {
            return send_socket(socket, json!({"type":"patch_rejected","note_id":note_id,"expected_version":base_version,"current_version":note.current_version,"reason":"VERSION_CONFLICT"})).await;
        }

        note.current_version += 1;
        note.current_version
    };

    let stream_id = format!("note:{note_id}");
    let event_seq = store.next_stream_seq(&stream_id);
    if let Some(note) = store.notes.get_mut(&note_id) {
        note.idempotency
            .insert(idempotency_key.clone(), (version, event_seq));
        note.history.push(crate::model::NoteEvent {
            event_seq,
            version,
            event_type: "note_patched".to_string(),
            payload: json!({"actor_id":user_id}),
        });
    }

    send_socket(socket, json!({"type":"patch_committed","note_id":note_id,"version":version,"event_seq":event_seq,"idempotency_key":idempotency_key})).await?;
    let note_event = json!({"type":"note_event","note_id":note_id,"event_seq":event_seq,"version":version,"event_type":"note_patched","payload":{}});
    store.append_stream_event(&stream_id, note_event.clone());
    let _ = state.ws_tx.send(WsEnvelope { stream_id, payload: note_event });
    Ok(())
}

async fn send_socket(socket: &mut WebSocket, payload: Value) -> Result<(), ()> {
    socket
        .send(Message::Text(payload.to_string().into()))
        .await
        .map_err(|_| ())
}
