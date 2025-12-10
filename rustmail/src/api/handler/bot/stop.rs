use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_stop_bot(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> (StatusCode, Json<&'static str>) {
    match stop_bot(bot_state).await {
        StopBotResponse::Success(status, message) => (status, message),
        StopBotResponse::Conflict(status, message) => (status, message),
    }
}
