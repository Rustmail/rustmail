use crate::commands::status::BotStatus as PresenceStatus;
use crate::i18n::get_translated_message;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serenity::all::ActivityData;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::Mutex;

pub async fn handle_status_bot(State(bot_state): State<Arc<Mutex<BotState>>>) -> impl IntoResponse {
    let state_lock = bot_state.lock().await;
    let presence_status = state_lock.presence_status.read().await.clone();

    match state_lock.status {
        BotStatus::Running { .. } => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "running",
                "presence": presence_status
            })),
        ),
        BotStatus::Stopped => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "stopped",
                "presence": presence_status
            })),
        ),
    }
}

#[derive(serde::Deserialize)]
pub struct PresenceStatusRequest {
    pub status: String,
}

pub async fn handle_set_presence(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(payload): Json<PresenceStatusRequest>,
) -> impl IntoResponse {
    let presence = match PresenceStatus::from_str(&payload.status) {
        Ok(s) => s,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Invalid status value"})),
            );
        }
    };

    let state_lock = bot_state.lock().await;

    let ctx_guard = state_lock.bot_context.read().await;
    let ctx = match ctx_guard.as_ref() {
        Some(c) => c,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "Bot is not running"})),
            );
        }
    };

    let config = match &state_lock.config {
        Some(c) => c.clone(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Config not loaded"})),
            );
        }
    };

    match presence {
        PresenceStatus::Online => {
            state_lock.maintenance_mode.store(false, Ordering::Relaxed);
            ctx.set_activity(Some(ActivityData::playing(&config.bot.status)));
            ctx.online();
        }
        PresenceStatus::Idle => {
            ctx.idle();
        }
        PresenceStatus::Dnd => {
            ctx.dnd();
        }
        PresenceStatus::Invisible => {
            ctx.invisible();
        }
        PresenceStatus::Maintenance => {
            state_lock.maintenance_mode.store(true, Ordering::Relaxed);
            let maintenance_status = futures::executor::block_on(get_translated_message(
                &config,
                "status.maintenance_activity",
                None,
                None,
                None,
                None,
            ));
            ctx.set_activity(Some(ActivityData::playing(&maintenance_status)));
            ctx.dnd();
        }
    }

    let mut presence_guard = state_lock.presence_status.write().await;
    *presence_guard = payload.status.clone();

    (
        StatusCode::OK,
        Json(serde_json::json!({"status": payload.status})),
    )
}
