use crate::db::operations::banned_users::get_all_banned_users;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serenity::all::{GuildId, RoleId, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Serialize)]
struct PanelBannedUser {
    user_id: String,
    username: String,
    global_name: Option<String>,
    nickname: Option<String>,
    avatar_url: Option<String>,
    roles: Vec<String>,
    joined_at: Option<i64>,
    banned_at: i64,
    banned_by_id: Option<String>,
    banned_by_name: Option<String>,
    ban_reason: Option<String>,
    roles_unknown: bool,
}

pub async fn handle_list_bans(State(bot_state): State<Arc<Mutex<BotState>>>) -> impl IntoResponse {
    let (community_guild_id, pool, bot_http) = {
        let state_lock = bot_state.lock().await;
        let config = match &state_lock.config {
            Some(c) => c,
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Config not loaded"})),
                )
                    .into_response();
            }
        };
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
        (config.bot.get_community_guild_id(), pool, http)
    };

    let users = match get_all_banned_users(&community_guild_id.to_string(), &pool).await {
        Ok(users) => users,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to fetch banned users: {}", e)})),
            )
                .into_response();
        }
    };

    let community_guild_obj = GuildId::new(community_guild_id);
    let roles_map = match community_guild_obj.roles(&bot_http).await {
        Ok(r) => r,
        Err(_) => HashMap::new(),
    };

    let mut banned_by_map = HashMap::new();
    for id_str in users.iter().filter_map(|u| u.banned_by.as_deref()) {
        if banned_by_map.contains_key(id_str) {
            continue;
        }
        if let Ok(id) = id_str.parse::<u64>()
            && let Ok(user) = bot_http.get_user(UserId::new(id)).await
        {
            banned_by_map.insert(id_str.to_string(), user.name);
        }
    }

    let panel_users: Vec<PanelBannedUser> = users
        .into_iter()
        .map(|user| {
            let resolved_roles = user
                .roles
                .iter()
                .map(|role_id_str| {
                    if let Ok(id) = role_id_str.parse::<u64>()
                        && let Some(role) = roles_map.get(&RoleId::new(id))
                    {
                        return role.name.clone();
                    }
                    role_id_str.clone()
                })
                .collect();

            let banned_by_id = user.banned_by.clone();
            let banned_by_name = banned_by_id
                .as_ref()
                .and_then(|id| banned_by_map.get(id).cloned());

            PanelBannedUser {
                user_id: user.user_id,
                username: user.username,
                global_name: user.global_name,
                nickname: user.nickname,
                avatar_url: user.avatar_url,
                roles: resolved_roles,
                joined_at: user.joined_at,
                banned_at: user.banned_at,
                banned_by_id,
                banned_by_name,
                ban_reason: user.ban_reason,
                roles_unknown: user.roles_unknown,
            }
        })
        .collect();

    (StatusCode::OK, Json(panel_users)).into_response()
}
