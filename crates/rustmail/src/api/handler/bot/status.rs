use crate::commands::status::{PresenceMode, apply_presence};
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_status_bot(State(bot_state): State<Arc<Mutex<BotState>>>) -> impl IntoResponse {
    let state_lock = bot_state.lock().await;
    let presence_status = state_lock.presence_status.read().await.clone();

    match state_lock.status {
        BotStatus::Running { .. } => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "running",
                "presence": presence_status
            })),
        ),
        BotStatus::Stopped => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "stopped",
                "presence": presence_status
            })),
        ),
    }
}

#[derive(serde::Deserialize)]
pub struct PresenceStatusRequest {
    pub status: String,
}

pub async fn handle_set_presence(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(payload): Json<PresenceStatusRequest>,
) -> impl IntoResponse {
    let presence = match PresenceMode::from_str(&payload.status) {
        Ok(s) => s,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Invalid status value"})),
            );
        }
    };

    let state_lock = bot_state.lock().await;

    let ctx_guard = state_lock.bot_context.read().await;
    let ctx = match ctx_guard.as_ref() {
        Some(c) => c,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "Bot is not running"})),
            );
        }
    };

    let config = match &state_lock.config {
        Some(c) => c.clone(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Config not loaded"})),
            );
        }
    };

    apply_presence(ctx, &config, presence, &state_lock.maintenance_mode).await;

    let mut presence_guard = state_lock.presence_status.write().await;
    *presence_guard = payload.status.clone();

    (
        StatusCode::OK,
        Json(serde_json::json!({"status": payload.status})),
    )
}
