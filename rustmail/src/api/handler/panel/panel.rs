use crate::BotState;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use std::sync::Arc;
use hyper::StatusCode;
use tokio::sync::Mutex;

pub async fn handle_panel_check(State(bot_state): State<Arc<Mutex<BotState>>>) -> impl IntoResponse {
    axum::response::Json(serde_json::json!({ "authorized": true }))
}
