use crate::types::bot::{BotState, BotStatus};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_status_bot(State(bot_state): State<Arc<Mutex<BotState>>>) -> impl IntoResponse {
    let state_lock = bot_state.lock().await;

    match state_lock.status {
        BotStatus::Running { .. } => (
            StatusCode::OK,
            Json(serde_json::json!({"status": "running"})),
        ),
        BotStatus::Stopped => (
            StatusCode::OK,
            Json(serde_json::json!({"status": "stopped"})),
        ),
    }
}
