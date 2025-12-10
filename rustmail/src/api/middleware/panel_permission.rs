use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use rustmail_types::api::panel_permissions::PanelPermission;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn require_panel_permission(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    jar: CookieJar,
    req: Request,
    next: Next,
    required_permission: PanelPermission,
) -> Response {
    let (db_pool, config, guild_id, bot_http) = {
        let state_lock = bot_state.lock().await;
        let db = match &state_lock.db_pool {
            Some(pool) => pool.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database not initialized",
                )
                    .into_response();
            }
        };
        let cfg = match &state_lock.config {
            Some(c) => c.clone(),
            None => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Config not initialized")
                    .into_response();
            }
        };
        let gid = cfg.bot.get_staff_guild_id();
        let http = match &state_lock.bot_http {
            Some(h) => h.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Bot HTTP not initialized",
                )
                    .into_response();
            }
        };
        (db, cfg, gid, http)
    };

    let session_cookie = jar.get("session_id");
    if session_cookie.is_none() {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let session_id = session_cookie.unwrap().value().to_string();
    let user_id = get_user_id_from_session(&session_id, &db_pool).await;

    if !has_panel_permission(
        &user_id,
        required_permission,
        &config,
        guild_id,
        bot_http,
        &db_pool,
    )
    .await
    {
        return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
    }

    next.run(req).await
}
