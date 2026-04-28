//! Live broadcast page and signaling handlers

use crate::error::AppError;
use crate::web::db;
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::live::LiveRole;
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::{HeaderMap, Uri};
use axum::response::Response;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

pub async fn live_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    if !db::is_setup(pool).await? {
        return Ok(http::redirect("/setup"));
    }
    let is_admin = session::check_session(&headers, pool).await?;
    let settings = db::get_settings(pool).await?;
    let site = SiteContext::from_settings(&settings);
    Ok(http::html(templates::live_page(
        &site,
        is_admin,
        &session::login_url(&uri),
        &settings,
    )))
}

pub async fn live_ws(
    State(state): State<AppState>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Result<Response, AppError> {
    let is_admin = session::check_session(&headers, &state.pool).await?;
    Ok(ws.on_upgrade(move |socket| handle_socket(state, is_admin, socket)))
}

async fn handle_socket(state: AppState, is_admin: bool, socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                break;
            }
        }
    });
    let Some(role) = register_role(&state, is_admin, &tx, &mut receiver).await else {
        send_task.abort();
        return;
    };
    tracing::info!(role = role_name(&role), "live websocket registered");
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            forward_message(&state, &role, &text).await;
        }
    }
    tracing::info!(role = role_name(&role), "live websocket closed");
    state.live_hub.unregister(&role).await;
    send_task.abort();
}

async fn register_role(
    state: &AppState,
    is_admin: bool,
    tx: &mpsc::UnboundedSender<Message>,
    receiver: &mut futures_util::stream::SplitStream<WebSocket>,
) -> Option<LiveRole> {
    let hello = next_json(receiver).await?;
    let role = hello.get("role")?.as_str()?.to_string();
    match role.as_str() {
        "broadcaster" if is_admin => state
            .live_hub
            .register_broadcaster(tx.clone())
            .await
            .map_err(|message| send_error(tx, &message))
            .ok(),
        "broadcaster" => {
            send_error(tx, "Admin session required.");
            None
        }
        "viewer" => Some(state.live_hub.register_viewer(tx.clone()).await),
        _ => {
            send_error(tx, "Unknown live role.");
            None
        }
    }
}

async fn next_json(receiver: &mut futures_util::stream::SplitStream<WebSocket>) -> Option<Value> {
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            return serde_json::from_str(&text).ok();
        }
    }
    None
}

async fn forward_message(state: &AppState, role: &LiveRole, text: &str) {
    let Ok(value) = serde_json::from_str::<Value>(text) else {
        tracing::warn!(role = role_name(role), "invalid live websocket JSON");
        return;
    };
    match value.get("type").and_then(Value::as_str) {
        Some("publish_offer") if matches!(role, LiveRole::Broadcaster) => {
            if let Some(sdp) = session_description(&value) {
                tracing::info!(role = role_name(role), "live publish offer received");
                state.live_hub.publish_offer(sdp).await;
            }
        }
        Some("view_offer") => {
            if let (LiveRole::Viewer(id), Some(sdp)) = (role, session_description(&value)) {
                tracing::info!(viewer_id = id, "live view offer received");
                state.live_hub.view_offer(id, sdp).await;
            }
        }
        Some("ice") => {
            if let Some(candidate) = ice_candidate(&value) {
                tracing::debug!(role = role_name(role), "live ICE candidate received");
                state.live_hub.add_ice(role, candidate).await;
            }
        }
        _ => tracing::debug!(role = role_name(role), "ignored live websocket message"),
    }
}

fn session_description(value: &Value) -> Option<RTCSessionDescription> {
    serde_json::from_value(value.get("sdp")?.clone()).ok()
}

fn ice_candidate(value: &Value) -> Option<RTCIceCandidateInit> {
    serde_json::from_value(value.get("candidate")?.clone()).ok()
}

fn send_error(tx: &mpsc::UnboundedSender<Message>, message: &str) {
    let _ = tx.send(Message::Text(
        json!({ "type": "error", "message": message })
            .to_string()
            .into(),
    ));
}

fn role_name(role: &LiveRole) -> &'static str {
    match role {
        LiveRole::Broadcaster => "broadcaster",
        LiveRole::Viewer(_) => "viewer",
    }
}
