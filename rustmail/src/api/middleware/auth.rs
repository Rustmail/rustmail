use crate::db::operations::{get_api_key_by_hash, hash_api_key, update_last_used};
use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::extract::State;
use axum::extract::{ConnectInfo, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use hyper::StatusCode;
use serenity::all::{GuildId, UserId};
use sqlx::{Row, query};
use std::net::SocketAddr;
use std::sync::Arc;
use subtle::ConstantTimeEq;
use tokio::sync::Mutex;

async fn check_user_with_bot(bot_state: Arc<Mutex<BotState>>, user_id: &str) -> bool {
    let user_id_num = match user_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => return false,
    };

    let cmd_tx = {
        let state_lock = bot_state.lock().await;
        state_lock.command_tx.clone()
    };

    let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();

    if cmd_tx
        .send(BotCommand::CheckUserIsMember {
            user_id: user_id_num,
            resp: resp_tx,
        })
        .await
        .is_err()
    {
        return false;
    }

    resp_rx.await.unwrap()
}

async fn check_user_with_api(
    user_id: &str,
    guild_id: u64,
    bot_http: Arc<serenity::http::Http>,
) -> bool {
    let guild_id = GuildId::new(guild_id);
    let user_id = match user_id.parse::<u64>() {
        Ok(id) => UserId::new(id),
        Err(_) => return false,
    };

    match guild_id.member(bot_http, user_id).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn verify_user(user_id: &str, guild_id: u64, bot_state: Arc<Mutex<BotState>>) -> bool {
    let state_lock = bot_state.lock().await;

    let is_bot_on = match state_lock.status {
        BotStatus::Running { .. } => true,
        BotStatus::Stopped => false,
    };

    drop(state_lock);

    let http = {
        let state_lock = bot_state.lock().await;
        match &state_lock.bot_http {
            Some(bot_http) => bot_http.clone(),
            None => return false,
        }
    };

    if is_bot_on {
        return check_user_with_bot(bot_state, user_id).await;
    }
    check_user_with_api(user_id, guild_id, http).await
}

pub async fn auth_middleware(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Response {
    if addr.ip().is_loopback() {
        if let Some(h) = req.headers().get("x-internal-call") {
            if let Ok(s) = h.to_str() {
                let state_lock = bot_state.lock().await;
                let expected = state_lock.internal_token.as_bytes();

                if expected.ct_eq(s.as_bytes()).unwrap_u8() == 1 {
                    drop(state_lock);
                    return next.run(req).await;
                }
            }
        }
    }

    let db_pool = {
        let state_lock = bot_state.lock().await;
        match &state_lock.db_pool {
            Some(pool) => pool.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database not initialized",
                )
                    .into_response();
            }
        }
    };

    if let Some(api_key_header) = req.headers().get("x-api-key") {
        if let Ok(api_key_str) = api_key_header.to_str() {
            let key_hash = hash_api_key(api_key_str);

            match get_api_key_by_hash(&db_pool, &key_hash).await {
                Ok(Some(api_key)) => {
                    if api_key.is_valid() {
                        let pool_clone = db_pool.clone();
                        let key_id = api_key.id;
                        tokio::spawn(async move {
                            let _ = update_last_used(&pool_clone, key_id).await;
                        });

                        req.extensions_mut().insert(api_key);
                        return next.run(req).await;
                    } else {
                        return (StatusCode::UNAUTHORIZED, "API key expired or inactive")
                            .into_response();
                    }
                }
                Ok(None) => {
                    return (StatusCode::UNAUTHORIZED, "Invalid API key").into_response();
                }
                Err(e) => {
                    eprintln!("Error fetching API key: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                        .into_response();
                }
            }
        }
    }

    let session_cookie = jar.get("session_id");

    if session_cookie.is_none() {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let session_id = session_cookie.unwrap().value().to_string();
    let user_id = get_user_id_from_session(&session_id, &db_pool).await;

    let guild_id = {
        let state_lock = bot_state.lock().await;
        match &state_lock.config {
            Some(config) => config.bot.get_staff_guild_id(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database not initialized",
                )
                    .into_response();
            }
        }
    };

    let result =
        query("SELECT expires_at FROM sessions_panel WHERE session_id = ? AND user_id = ?")
            .bind(&session_id)
            .bind(&user_id)
            .fetch_one(&db_pool)
            .await;

    match result {
        Ok(row) => {
            let expires_at = row.get::<i64, _>("expires_at");
            let now = Utc::now().timestamp();

            if expires_at < now {
                let _ = query("DELETE FROM sessions_panel WHERE session_id = ?")
                    .bind(&session_id)
                    .execute(&db_pool)
                    .await;
                return (StatusCode::UNAUTHORIZED, "Session expired").into_response();
            }

            if !verify_user(&user_id, guild_id, bot_state.clone()).await {
                let _ = query("DELETE FROM sessions_panel WHERE session_id = ?")
                    .bind(&session_id)
                    .execute(&db_pool)
                    .await;
                return (StatusCode::UNAUTHORIZED, "Invalid session").into_response();
            }

            next.run(req).await
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    }
}
