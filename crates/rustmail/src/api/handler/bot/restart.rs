use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use sqlx::__rt::sleep;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub async fn handle_restart_bot(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> (StatusCode, Json<&'static str>) {
    let state_lock = bot_state.lock().await;

    match state_lock.status {
        BotStatus::Stopped => {
            drop(state_lock);

            match start_bot(bot_state).await {
                StartBotResponse::Success(status, message) => (status, message),
                StartBotResponse::Conflict(status, message) => (status, message),
            }
        }
        BotStatus::Running { .. } => {
            drop(state_lock);

            match stop_bot(bot_state.clone()).await {
                StopBotResponse::Success(..) => {}
                StopBotResponse::Conflict(status, message) => return (status, message),
            }

            sleep(Duration::from_secs(2)).await;

            match start_bot(bot_state).await {
                StartBotResponse::Success(..) => {}
                StartBotResponse::Conflict(status, message) => return (status, message),
            }

            (StatusCode::OK, Json("Bot is restarting"))
        }
    }
}
