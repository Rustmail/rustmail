use crate::bot::ShardManagerKey;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serenity::all::ShardId;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_health(State(bot_state): State<Arc<Mutex<BotState>>>) -> impl IntoResponse {
    let state_lock = bot_state.lock().await;

    let mut db_ok = false;
    if let Some(pool) = &state_lock.db_pool
        && sqlx::query("SELECT 1").execute(pool).await.is_ok()
    {
        db_ok = true;
    }

    let bot_running = matches!(state_lock.status, BotStatus::Running { .. });

    let ctx_guard = state_lock.bot_context.read().await;
    let bot_connected = ctx_guard.is_some();

    let mut bot_latency = None;
    if let Some(ctx) = ctx_guard.as_ref() {
        let data = ctx.data.read().await;
        if let Some(shard_manager) = data.get::<ShardManagerKey>() {
            let runners = shard_manager.runners.lock().await;
            if let Some(runner) = runners.get(&ShardId(0)) {
                bot_latency = runner.latency.map(|d| d.as_millis());
            }
        }
    }

    let mut status_code = StatusCode::OK;
    if !db_ok || (bot_running && !bot_connected) {
        status_code = StatusCode::SERVICE_UNAVAILABLE;
    }

    (
        status_code,
        Json(serde_json::json!({
            "status": if db_ok && (!bot_running || bot_connected) { "healthy" } else { "unhealthy" },
            "database": if db_ok { "connected" } else { "disconnected" },
            "bot": {
                "running": bot_running,
                "connected": bot_connected,
                "latency_ms": bot_latency,
            }
        })),
    )
}
