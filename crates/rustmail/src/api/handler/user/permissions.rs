use crate::api::utils::panel_permissions::get_user_panel_permissions;
use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_get_user_permissions(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let (db_pool, config, guild_id, bot_http, user_id) = {
        let state_lock = bot_state.lock().await;

        let pool = match &state_lock.db_pool {
            Some(p) => p.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Database not initialized"})),
                )
                    .into_response();
            }
        };

        let cfg = match &state_lock.config {
            Some(c) => c.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Config not loaded"})),
                )
                    .into_response();
            }
        };

        let http = match &state_lock.bot_http {
            Some(h) => h.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Bot not initialized"})),
                )
                    .into_response();
            }
        };

        let session_cookie = jar.get("session_id");
        if session_cookie.is_none() {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Unauthorized"})),
            )
                .into_response();
        }

        let session_id = session_cookie.unwrap().value().to_string();
        let uid = get_user_id_from_session(&session_id, &pool).await;
        let gid = cfg.bot.get_staff_guild_id();

        (pool, cfg, gid, http, uid)
    };

    let permissions =
        get_user_panel_permissions(&user_id, &config, guild_id, bot_http, &db_pool).await;

    (StatusCode::OK, Json(permissions)).into_response()
}
