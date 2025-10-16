use crate::bot::run_bot;
use crate::config::load_config;
use crate::{BotState, BotStatus};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::Mutex;

pub async fn handle_start_bot(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> (StatusCode, Json<&'static str>) {
    let mut state_lock = bot_state.lock().await;

    match state_lock.status {
        BotStatus::Stopped => {
            state_lock.config = load_config("config.toml");

            if state_lock.config.is_none() {
                return (StatusCode::BAD_REQUEST, Json("Missing configuration."));
            }

            let (shutdown_tx, mut shutdown_rx) = tokio::sync::watch::channel(false);
            let (command_tx, command_rx) = tokio::sync::mpsc::channel(32);
            let bot_state_clone = bot_state.clone();

            let handle = spawn(async move {
                run_bot(bot_state_clone.clone(), &mut shutdown_rx, command_rx).await;
                let mut s = bot_state_clone.lock().await;
                s.status = BotStatus::Stopped;
            });
            state_lock.status = BotStatus::Running {
                handle,
                shutdown: shutdown_tx,
            };
            state_lock.command_tx = command_tx;

            drop(state_lock);
            (StatusCode::OK, Json("Bot is starting"))
        }
        BotStatus::Running { .. } => (StatusCode::CONFLICT, Json("Bot is already running")),
    }
}
