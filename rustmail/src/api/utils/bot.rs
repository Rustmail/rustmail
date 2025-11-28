use std::sync::Arc;
use axum::http::StatusCode;
use axum::Json;
use tokio::spawn;
use tokio::sync::Mutex;
use crate::bot::run_bot;
use crate::config::load_config;
use crate::types::{BotState, BotStatus};

pub enum StartBotResponse {
    Success(StatusCode, Json<&'static str>),
    Conflict(StatusCode, Json<&'static str>),
}

pub enum StopBotResponse {
    Success(StatusCode, Json<&'static str>),
    Conflict(StatusCode, Json<&'static str>),
}

pub async fn start_bot(bot_state: Arc<Mutex<BotState>>) -> StartBotResponse
{
    let mut state_lock = bot_state.lock().await;
    match state_lock.status {
        BotStatus::Stopped => {
            state_lock.config = load_config("config.toml");

            if state_lock.config.is_none() {
                return StartBotResponse::Conflict(StatusCode::BAD_REQUEST, Json("Missing configuration."));
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
            StartBotResponse::Success(StatusCode::OK, Json("Bot is starting"))
        }
        BotStatus::Running { .. } => StartBotResponse::Conflict(StatusCode::BAD_REQUEST, Json("Bot is already running")),
    }
}

pub async fn stop_bot(bot_state: Arc<Mutex<BotState>>) -> StopBotResponse {
    let handle_and_shutdown = {
        let mut state_lock = bot_state.lock().await;

        match std::mem::replace(&mut state_lock.status, BotStatus::Stopped) {
            BotStatus::Running { handle, shutdown } => Some((handle, shutdown)),
            BotStatus::Stopped => None,
        }
    };

    if let Some((handle, shutdown_tx)) = handle_and_shutdown {
        let _ = shutdown_tx.send(true);
        handle.await.unwrap();
        StopBotResponse::Success(StatusCode::OK, Json("Bot stopped"))
    } else {
        println!("Not Starting bot stop");
        StopBotResponse::Conflict(StatusCode::CONFLICT, Json("Bot is not running"))
    }
}