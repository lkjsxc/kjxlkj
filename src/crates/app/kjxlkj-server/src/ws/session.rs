use std::{collections::HashMap, time::Duration};

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Message};
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use chrono::Utc;
use serde_json::json;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    auth::{auth_session, AuthSession},
    db_notes,
    error::AppError,
    ws::{ClientMessage, ServerEvent},
};

pub async fn ws_notes(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ActixError> {
    let session = auth_session(&req, &state)
        .await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;
    ws::start(
        WsSession::new(state.get_ref().clone(), session),
        &req,
        stream,
    )
}

pub struct WsSession {
    state: AppState,
    auth: AuthSession,
    subscriptions: HashMap<String, JoinHandle<()>>,
    ack_cursor: HashMap<String, i64>,
}

impl WsSession {
    fn new(state: AppState, auth: AuthSession) -> Self {
        Self {
            state,
            auth,
            subscriptions: HashMap::new(),
            ack_cursor: HashMap::new(),
        }
    }

    fn send_json(ctx: &mut ws::WebsocketContext<Self>, value: serde_json::Value) {
        ctx.text(value.to_string());
    }

    fn subscribe_stream(&mut self, stream_id: &str, ctx: &mut ws::WebsocketContext<Self>) {
        if self.subscriptions.contains_key(stream_id) {
            return;
        }
        let addr: Addr<Self> = ctx.address();
        let state = self.state.clone();
        let stream_id_owned = stream_id.to_string();
        let handle = tokio::spawn(async move {
            let sender = state.topic_sender(&stream_id_owned).await;
            let mut rx = sender.subscribe();
            loop {
                match rx.recv().await {
                    Ok(event) => addr.do_send(HubEvent(event)),
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });
        self.subscriptions.insert(stream_id.to_string(), handle);
    }

    fn subscribe_note(&mut self, note_id: Uuid, ctx: &mut ws::WebsocketContext<Self>) {
        let stream_id = AppState::note_stream_id(note_id);
        self.subscribe_stream(&stream_id, ctx);

        let addr = ctx.address();
        let pool = self.state.pool.clone();
        tokio::spawn(async move {
            let current_version: i64 =
                sqlx::query_scalar("select current_version from note_streams where id = $1")
                    .bind(note_id)
                    .fetch_optional(&pool)
                    .await
                    .ok()
                    .flatten()
                    .unwrap_or(0);
            addr.do_send(SendText(
                json!({
                    "type": "subscribed",
                    "stream_id": stream_id,
                    "current_version": current_version,
                    "replay_cursor": current_version,
                })
                .to_string(),
            ));
        });
    }

    fn subscribe_workspace(&mut self, workspace_id: Uuid, ctx: &mut ws::WebsocketContext<Self>) {
        let stream_id = AppState::workspace_stream_id(workspace_id);
        self.subscribe_stream(&stream_id, ctx);

        let addr = ctx.address();
        let pool = self.state.pool.clone();
        tokio::spawn(async move {
            let current_version: i64 = sqlx::query_scalar(
                "select coalesce(max(seq), 0) from workspace_events where workspace_id = $1",
            )
            .bind(workspace_id)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);
            addr.do_send(SendText(
                json!({
                    "type": "subscribed",
                    "stream_id": stream_id,
                    "current_version": current_version,
                    "replay_cursor": current_version,
                })
                .to_string(),
            ));
        });
    }

    fn unsubscribe(&mut self, stream_id: &str) {
        if let Some(handle) = self.subscriptions.remove(stream_id) {
            handle.abort();
        }
    }

    fn handle_ack(
        &mut self,
        stream_id: String,
        event_seq: i64,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        self.ack_cursor.insert(stream_id.clone(), event_seq);
        let addr = ctx.address();
        let pool = self.state.pool.clone();
        tokio::spawn(async move {
            if let Some(raw_id) = stream_id.strip_prefix("note:") {
                if let Ok(note_id) = Uuid::parse_str(raw_id) {
                    let events = sqlx::query_as::<_, (i64, String, serde_json::Value)>(
                        "select seq, event_type, payload_json
                         from note_events where note_id = $1 and seq > $2
                         order by seq asc",
                    )
                    .bind(note_id)
                    .bind(event_seq)
                    .fetch_all(&pool)
                    .await
                    .unwrap_or_default();

                    for (seq, event_type, payload_json) in events {
                        addr.do_send(SendText(
                            json!({
                                "type": "note_event",
                                "note_id": note_id,
                                "event_seq": seq,
                                "version": seq,
                                "event_type": event_type,
                                "payload": payload_json,
                            })
                            .to_string(),
                        ));
                    }
                }
            } else if let Some(raw_id) = stream_id.strip_prefix("workspace:") {
                if let Ok(workspace_id) = Uuid::parse_str(raw_id) {
                    let events = sqlx::query_as::<_, (i64, String, serde_json::Value)>(
                        "select seq, event_type, payload_json
                         from workspace_events where workspace_id = $1 and seq > $2
                         order by seq asc",
                    )
                    .bind(workspace_id)
                    .bind(event_seq)
                    .fetch_all(&pool)
                    .await
                    .unwrap_or_default();

                    for (seq, event_type, payload_json) in events {
                        addr.do_send(SendText(
                            json!({
                                "type": "workspace_event",
                                "workspace_id": workspace_id,
                                "event_seq": seq,
                                "event_type": event_type,
                                "payload": payload_json,
                            })
                            .to_string(),
                        ));
                    }
                }
            }
        });
    }

    fn handle_client_msg(&mut self, msg: ClientMessage, ctx: &mut ws::WebsocketContext<Self>) {
        match msg {
            ClientMessage::SubscribeNote { note_id } => self.subscribe_note(note_id, ctx),
            ClientMessage::UnsubscribeNote { note_id } => {
                self.unsubscribe(&AppState::note_stream_id(note_id))
            }
            ClientMessage::SubscribeWorkspace { workspace_id } => {
                self.subscribe_workspace(workspace_id, ctx)
            }
            ClientMessage::UnsubscribeWorkspace { workspace_id } => {
                self.unsubscribe(&AppState::workspace_stream_id(workspace_id))
            }
            ClientMessage::Ack {
                stream_id,
                event_seq,
            } => self.handle_ack(stream_id, event_seq, ctx),
            ClientMessage::PresencePing {
                workspace_id,
                note_id,
                cursor: _,
            } => {
                let state = self.state.clone();
                let user_id = self.auth.user_id;
                tokio::spawn(async move {
                    state
                        .publish_workspace(
                            workspace_id,
                            ServerEvent::PresenceEvent {
                                workspace_id,
                                note_id,
                                user_id,
                                state: "active".to_string(),
                                server_ts: Utc::now().to_rfc3339(),
                            },
                        )
                        .await;
                });
            }
            ClientMessage::ApplyPatch {
                note_id,
                base_version,
                patch_ops,
                idempotency_key,
                client_ts,
            } => {
                let _ = client_ts;
                let addr = ctx.address();
                let state = self.state.clone();
                let actor_id = self.auth.user_id;
                tokio::spawn(async move {
                    let result = db_notes::apply_note_patch(
                        &state.pool,
                        actor_id,
                        note_id,
                        base_version,
                        &patch_ops,
                        &idempotency_key,
                    )
                    .await;
                    match result {
                        Ok((version, seq, projection)) => {
                            state
                                .publish(
                                    note_id,
                                    ServerEvent::note_event(
                                        note_id,
                                        seq,
                                        version,
                                        "patch",
                                        serde_json::to_value(projection).unwrap_or_default(),
                                    ),
                                )
                                .await;
                            addr.do_send(SendText(
                                json!({
                                    "type": "patch_committed",
                                    "note_id": note_id,
                                    "version": version,
                                    "event_seq": seq,
                                    "idempotency_key": idempotency_key,
                                })
                                .to_string(),
                            ));
                        }
                        Err(AppError::VersionConflict {
                            expected_version,
                            current_version,
                        }) => {
                            addr.do_send(SendText(
                                json!({
                                    "type": "patch_rejected",
                                    "note_id": note_id,
                                    "expected_version": expected_version,
                                    "current_version": current_version,
                                    "replay_cursor": current_version,
                                    "reason": "base version does not match current version",
                                })
                                .to_string(),
                            ));
                        }
                        Err(err) => {
                            addr.do_send(SendText(
                                json!({
                                    "type": "error",
                                    "code": "PATCH_ERROR",
                                    "message": err.to_string(),
                                    "request_id": Uuid::now_v7(),
                                })
                                .to_string(),
                            ));
                        }
                    }
                });
            }
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(10), |_, ctx| {
            Self::send_json(
                ctx,
                json!({ "type": "heartbeat", "server_ts": Utc::now().to_rfc3339() }),
            );
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        for (_, handle) in self.subscriptions.drain() {
            handle.abort();
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct SendText(String);

impl Handler<SendText> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: SendText, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct HubEvent(ServerEvent);

impl Handler<HubEvent> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: HubEvent, ctx: &mut Self::Context) -> Self::Result {
        Self::send_json(
            ctx,
            serde_json::to_value(msg.0).unwrap_or_else(|_| json!({"type": "error"})),
        );
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Text(text)) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(msg) => self.handle_client_msg(msg, ctx),
                Err(_) => Self::send_json(
                    ctx,
                    json!({
                        "type": "error",
                        "code": "BAD_MESSAGE",
                        "message": "invalid websocket message",
                        "request_id": Uuid::now_v7(),
                    }),
                ),
            },
            Ok(ws::Message::Ping(bytes)) => ctx.pong(&bytes),
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Close(_)) => ctx.stop(),
            Ok(ws::Message::Binary(_)) => Self::send_json(
                ctx,
                json!({
                    "type": "error",
                    "code": "BAD_MESSAGE",
                    "message": "binary not supported",
                    "request_id": Uuid::now_v7(),
                }),
            ),
            Ok(ws::Message::Continuation(_)) => {}
            Ok(ws::Message::Nop) => {}
            Err(_) => ctx.stop(),
        }
    }
}
