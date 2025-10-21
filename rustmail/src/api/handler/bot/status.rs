use crate::BotState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_status_bot(State(bot_state): State<Arc<Mutex<BotState>>>) -> impl IntoResponse {
    let state_lock = bot_state.lock().await;

    match state_lock.status {
        crate::BotStatus::Running { .. } => (
            StatusCode::OK,
            Json(serde_json::json!({"status": "running"})),
        ),
        crate::BotStatus::Stopped => (
            StatusCode::OK,
            Json(serde_json::json!({"status": "stopped"})),
        ),
    }
}
