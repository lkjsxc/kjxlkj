use crate::app_state::AppState;
use crate::authn::{optional_identity, SessionIdentity};
use crate::handlers::automation::evaluate_workspace_event;
use actix::{Actor, ActorContext, ActorFutureExt, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use kjxlkj_db::repos;
use kjxlkj_domain::Role;
use kjxlkj_rbac::{ensure_note_write, ensure_workspace_member_read};
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "subscribe_note")]
    SubscribeNote { note_id: Uuid },
    #[serde(rename = "unsubscribe_note")]
    UnsubscribeNote { note_id: Uuid },
    #[serde(rename = "subscribe_workspace")]
    SubscribeWorkspace { workspace_id: Uuid },
    #[serde(rename = "unsubscribe_workspace")]
    UnsubscribeWorkspace { workspace_id: Uuid },
    #[serde(rename = "apply_patch")]
    ApplyPatch {
        note_id: Uuid,
        base_version: i32,
        patch_ops: Vec<kjxlkj_db::repos::notes_patch::PatchOp>,
        idempotency_key: String,
        client_ts: Option<String>,
    },
    #[serde(rename = "ack")]
    Ack { stream_id: String, event_seq: i32 },
    #[serde(rename = "presence_ping")]
    PresencePing {
        workspace_id: Uuid,
        note_id: Option<Uuid>,
        cursor: Option<i32>,
    },
}

pub fn configure_root(cfg: &mut web::ServiceConfig) {
    cfg.route("/ws", web::get().to(ws_endpoint));
}

async fn ws_endpoint(
    req: HttpRequest,
    payload: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let identity = optional_identity(&req, &state)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let Some(identity) = identity else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    ws::start(WsSession::new(state.get_ref().clone(), identity), &req, payload)
}

struct WsSession {
    state: AppState,
    identity: SessionIdentity,
}

impl WsSession {
    fn new(state: AppState, identity: SessionIdentity) -> Self {
        Self { state, identity }
    }

    fn send_json(ctx: &mut ws::WebsocketContext<Self>, payload: serde_json::Value) {
        if let Ok(text) = serde_json::to_string(&payload) {
            ctx.text(text);
        }
    }

    fn send_protocol_error(ctx: &mut ws::WebsocketContext<Self>, code: &str, message: &str) {
        Self::send_json(
            ctx,
            json!({
                "type": "error",
                "code": code,
                "message": message,
                "request_id": Uuid::now_v7(),
            }),
        );
    }

    fn on_client_message(&mut self, message: ClientMessage, ctx: &mut ws::WebsocketContext<Self>) {
        match message {
            ClientMessage::SubscribeNote { note_id } => {
                let state = self.state.clone();
                let actor_id = self.identity.user_id;
                let stream_id = format!("note:{note_id}");
                ctx.spawn(
                    actix::fut::wrap_future(async move {
                        let Some((stream, _)) = repos::notes::get_note(&state.pool, note_id).await? else {
                            return Ok::<_, sqlx::Error>(json!({"kind": "not_found"}));
                        };

                        let role = repos::workspaces::actor_workspace_role(&state.pool, stream.workspace_id, actor_id)
                            .await?
                            .and_then(|value| Role::from_str(&value).ok());

                        let Some(role) = role else {
                            return Ok(json!({"kind": "forbidden"}));
                        };

                        if ensure_workspace_member_read(role).is_err() {
                            return Ok(json!({"kind": "forbidden"}));
                        }

                        let replay_cursor = state.ws_ack_cursor(actor_id, &stream_id);
                        let events = repos::notes::note_events_after(&state.pool, note_id, replay_cursor).await?;

                        Ok(json!({
                            "kind": "note_subscribed",
                            "stream_id": stream_id,
                            "note_id": note_id,
                            "current_version": stream.current_version,
                            "replay_cursor": replay_cursor,
                            "events": events.into_iter().map(|event| {
                                json!({
                                    "note_id": event.note_id,
                                    "event_seq": event.seq,
                                    "version": event.payload_json.get("version").and_then(|v| v.as_i64()).unwrap_or(event.seq as i64),
                                    "event_type": event.event_type,
                                    "payload": event.payload_json,
                                })
                            }).collect::<Vec<_>>(),
                        }))
                    })
                    .map(|result, _, ctx| match result {
                        Ok(payload) => {
                            let kind = payload.get("kind").and_then(|v| v.as_str()).unwrap_or_default();
                            if kind == "not_found" {
                                Self::send_protocol_error(ctx, "NOTE_NOT_FOUND", "note not found");
                                return;
                            }
                            if kind == "forbidden" {
                                Self::send_protocol_error(ctx, "ROLE_FORBIDDEN", "forbidden");
                                return;
                            }

                            Self::send_json(
                                ctx,
                                json!({
                                    "type": "subscribed",
                                    "stream_id": payload["stream_id"],
                                    "current_version": payload["current_version"],
                                    "replay_cursor": payload["replay_cursor"],
                                }),
                            );

                            if let Some(events) = payload.get("events").and_then(|value| value.as_array()) {
                                for event in events {
                                    Self::send_json(
                                        ctx,
                                        json!({
                                            "type": "note_event",
                                            "note_id": event["note_id"],
                                            "event_seq": event["event_seq"],
                                            "version": event["version"],
                                            "event_type": event["event_type"],
                                            "payload": event["payload"],
                                        }),
                                    );
                                }
                            }
                        }
                        Err(_) => Self::send_protocol_error(ctx, "INTERNAL_ERROR", "internal error"),
                    }),
                );
            }
            ClientMessage::SubscribeWorkspace { workspace_id } => {
                let state = self.state.clone();
                let actor_id = self.identity.user_id;
                let stream_id = format!("workspace:{workspace_id}");
                ctx.spawn(
                    actix::fut::wrap_future(async move {
                        let role = repos::workspaces::actor_workspace_role(&state.pool, workspace_id, actor_id)
                            .await?
                            .and_then(|value| Role::from_str(&value).ok());

                        let Some(role) = role else {
                            return Ok::<_, sqlx::Error>(json!({"kind": "forbidden"}));
                        };

                        if ensure_workspace_member_read(role).is_err() {
                            return Ok(json!({"kind": "forbidden"}));
                        }

                        let replay_cursor = state.ws_ack_cursor(actor_id, &stream_id);
                        let events = repos::notes::workspace_events_after(&state.pool, workspace_id, replay_cursor).await?;
                        let current_version = repos::notes::workspace_latest_seq(&state.pool, workspace_id).await?;

                        Ok(json!({
                            "kind": "workspace_subscribed",
                            "stream_id": stream_id,
                            "current_version": current_version,
                            "replay_cursor": replay_cursor,
                            "events": events.into_iter().map(|event| {
                                json!({
                                    "workspace_id": event.workspace_id,
                                    "event_seq": event.seq,
                                    "event_type": event.event_type,
                                    "payload": event.payload_json,
                                })
                            }).collect::<Vec<_>>(),
                        }))
                    })
                    .map(|result, _, ctx| match result {
                        Ok(payload) => {
                            if payload.get("kind") == Some(&json!("forbidden")) {
                                Self::send_protocol_error(ctx, "ROLE_FORBIDDEN", "forbidden");
                                return;
                            }

                            Self::send_json(
                                ctx,
                                json!({
                                    "type": "subscribed",
                                    "stream_id": payload["stream_id"],
                                    "current_version": payload["current_version"],
                                    "replay_cursor": payload["replay_cursor"],
                                }),
                            );

                            if let Some(events) = payload.get("events").and_then(|value| value.as_array()) {
                                for event in events {
                                    Self::send_json(
                                        ctx,
                                        json!({
                                            "type": "workspace_event",
                                            "workspace_id": event["workspace_id"],
                                            "event_seq": event["event_seq"],
                                            "event_type": event["event_type"],
                                            "payload": event["payload"],
                                        }),
                                    );
                                }
                            }
                        }
                        Err(_) => Self::send_protocol_error(ctx, "INTERNAL_ERROR", "internal error"),
                    }),
                );
            }
            ClientMessage::ApplyPatch {
                note_id,
                base_version,
                patch_ops,
                idempotency_key,
                client_ts,
            } => {
                let _ = client_ts;
                let state = self.state.clone();
                let actor_id = self.identity.user_id;
                ctx.spawn(
                    actix::fut::wrap_future(async move {
                        let Some((stream, _)) = repos::notes::get_note(&state.pool, note_id).await? else {
                            return Ok::<_, sqlx::Error>(json!({"kind": "not_found"}));
                        };

                        let role = repos::workspaces::actor_workspace_role(&state.pool, stream.workspace_id, actor_id)
                            .await?
                            .and_then(|value| Role::from_str(&value).ok());

                        let Some(role) = role else {
                            return Ok(json!({"kind": "forbidden"}));
                        };

                        if ensure_note_write(role).is_err() {
                            return Ok(json!({"kind": "forbidden"}));
                        }

                        let result = repos::notes::apply_note_patch(
                            &state.pool,
                            actor_id,
                            note_id,
                            base_version,
                            &patch_ops,
                            &idempotency_key,
                        )
                        .await;

                        match result {
                            Ok(result) => {
                                let _ = evaluate_workspace_event(
                                    &state,
                                    &Uuid::now_v7().to_string(),
                                    actor_id,
                                    stream.workspace_id,
                                    "note_patched",
                                    &format!("note:{}:{}", note_id, result.event_seq),
                                    &json!({
                                        "note_id": note_id,
                                        "version": result.version,
                                        "event_seq": result.event_seq,
                                    }),
                                )
                                .await;

                                let events = repos::notes::note_events_after(&state.pool, note_id, result.event_seq - 1)
                                    .await?;
                                let workspace_latest = repos::notes::workspace_latest_seq(&state.pool, stream.workspace_id)
                                    .await?;
                                let workspace_events = repos::notes::workspace_events_after(
                                    &state.pool,
                                    stream.workspace_id,
                                    workspace_latest.saturating_sub(1),
                                )
                                .await?;

                                Ok(json!({
                                    "kind": "patched",
                                    "note_id": note_id,
                                    "workspace_id": stream.workspace_id,
                                    "version": result.version,
                                    "event_seq": result.event_seq,
                                    "idempotency_key": idempotency_key,
                                    "note_events": events.into_iter().map(|event| {
                                        json!({
                                            "note_id": event.note_id,
                                            "seq": event.seq,
                                            "event_type": event.event_type,
                                            "payload_json": event.payload_json,
                                        })
                                    }).collect::<Vec<_>>(),
                                    "workspace_events": workspace_events.into_iter().map(|event| {
                                        json!({
                                            "workspace_id": event.workspace_id,
                                            "seq": event.seq,
                                            "event_type": event.event_type,
                                            "payload_json": event.payload_json,
                                        })
                                    }).collect::<Vec<_>>(),
                                }))
                            }
                            Err(error) => match error {
                                repos::notes::NoteMutationError::Conflict { current_version } => {
                                    Ok(json!({
                                        "kind": "conflict",
                                        "note_id": note_id,
                                        "expected_version": base_version,
                                        "current_version": current_version,
                                    }))
                                }
                                repos::notes::NoteMutationError::NotFound => Ok(json!({"kind": "not_found"})),
                                repos::notes::NoteMutationError::InvalidPatch => Ok(json!({"kind": "invalid_patch"})),
                                repos::notes::NoteMutationError::Database(_) => {
                                    Ok(json!({"kind": "db_error"}))
                                }
                            },
                        }
                    })
                    .map(|result, _, ctx| match result {
                        Ok(payload) => {
                            match payload.get("kind").and_then(|value| value.as_str()).unwrap_or_default() {
                                "not_found" => {
                                    Self::send_protocol_error(ctx, "NOTE_NOT_FOUND", "note not found")
                                }
                                "forbidden" => {
                                    Self::send_protocol_error(ctx, "ROLE_FORBIDDEN", "forbidden")
                                }
                                "invalid_patch" => {
                                    Self::send_protocol_error(ctx, "INVALID_PATCH", "invalid patch")
                                }
                                "db_error" => {
                                    Self::send_protocol_error(ctx, "INTERNAL_ERROR", "internal error")
                                }
                                "conflict" => Self::send_json(
                                    ctx,
                                    json!({
                                        "type": "patch_rejected",
                                        "note_id": payload["note_id"],
                                        "expected_version": payload["expected_version"],
                                        "current_version": payload["current_version"],
                                        "reason": "version_conflict",
                                    }),
                                ),
                                "patched" => {
                                    Self::send_json(
                                        ctx,
                                        json!({
                                            "type": "patch_committed",
                                            "note_id": payload["note_id"],
                                            "version": payload["version"],
                                            "event_seq": payload["event_seq"],
                                            "idempotency_key": payload["idempotency_key"],
                                        }),
                                    );

                                    if let Some(note_events) = payload.get("note_events").and_then(|value| value.as_array()) {
                                        for event in note_events {
                                            Self::send_json(
                                                ctx,
                                                json!({
                                                    "type": "note_event",
                                                    "note_id": event["note_id"],
                                                    "event_seq": event["seq"],
                                                    "version": event["payload_json"]["version"],
                                                    "event_type": event["event_type"],
                                                    "payload": event["payload_json"],
                                                }),
                                            );
                                        }
                                    }

                                    if let Some(workspace_events) = payload
                                        .get("workspace_events")
                                        .and_then(|value| value.as_array())
                                    {
                                        for event in workspace_events {
                                            Self::send_json(
                                                ctx,
                                                json!({
                                                    "type": "workspace_event",
                                                    "workspace_id": event["workspace_id"],
                                                    "event_seq": event["seq"],
                                                    "event_type": event["event_type"],
                                                    "payload": event["payload_json"],
                                                }),
                                            );
                                        }
                                    }
                                }
                                _ => Self::send_protocol_error(ctx, "INTERNAL_ERROR", "internal error"),
                            }
                        }
                        Err(_) => Self::send_protocol_error(ctx, "INTERNAL_ERROR", "internal error"),
                    }),
                );
            }
            ClientMessage::Ack { stream_id, event_seq } => {
                self.state
                    .set_ws_ack_cursor(self.identity.user_id, &stream_id, event_seq);
            }
            ClientMessage::UnsubscribeNote { note_id } => {
                let _ = note_id;
            }
            ClientMessage::UnsubscribeWorkspace { workspace_id } => {
                let _ = workspace_id;
            }
            ClientMessage::PresencePing {
                workspace_id,
                note_id,
                cursor,
            } => {
                Self::send_json(
                    ctx,
                    json!({
                        "type": "presence_event",
                        "workspace_id": workspace_id,
                        "note_id": note_id,
                        "user_id": self.identity.user_id,
                        "state": "active",
                        "cursor": cursor,
                    }),
                );
            }
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, message: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match message {
            Ok(ws::Message::Text(text)) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(parsed) => self.on_client_message(parsed, ctx),
                Err(_) => Self::send_protocol_error(ctx, "BAD_REQUEST", "invalid websocket payload"),
            },
            Ok(ws::Message::Ping(bytes)) => ctx.pong(&bytes),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Binary(_)) => {
                Self::send_protocol_error(ctx, "BAD_REQUEST", "binary payloads are unsupported")
            }
            _ => {}
        }
    }
}
