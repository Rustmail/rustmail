use std::sync::Arc;
use std::time::Duration;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::__rt::sleep;
use tokio::sync::Mutex;
use crate::{BotState, BotStatus};
use crate::api::utils::ping_internal::ping_internal;

pub async fn handle_restart_bot(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> (StatusCode, Json<&'static str>) {
    let token = {
        let state_lock = bot_state.lock().await;
        state_lock.internal_token.clone()
    };

    let state_lock = bot_state.lock().await;

    match state_lock.status {
        BotStatus::Stopped => {
            drop(state_lock);

            let result = ping_internal("/api/bot/start", &token)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to restart bot")));

            println!("Restart result: {:?}", result);

            (StatusCode::OK, Json("Bot is starting"))
        }
        BotStatus::Running { .. } => {
            drop(state_lock);

            let _ = ping_internal("/api/bot/stop", &token)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to stop bot")));

            sleep(Duration::from_secs(2)).await;

            let _ = ping_internal("/api/bot/start", &token)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to stop bot")));

            (StatusCode::OK, Json("Bot is restarting"))
        },
    }
}