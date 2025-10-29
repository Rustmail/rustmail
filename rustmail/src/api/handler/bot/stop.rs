use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_stop_bot(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> (StatusCode, Json<&'static str>) {
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
        (StatusCode::OK, Json("Bot stopped"))
    } else {
        println!("Not Starting bot stop");
        (StatusCode::CONFLICT, Json("Bot is not running"))
    }
}
