use crate::db::operations::banned_users::get_all_banned_users;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serenity::all::GuildId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_list_bans(State(bot_state): State<Arc<Mutex<BotState>>>) -> impl IntoResponse {
    let (community_guild_id, staff_guild_id, pool, bot_http) = {
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
        (
            config.bot.get_community_guild_id(),
            config.bot.get_staff_guild_id(),
            pool,
            http,
        )
    };

    let community_guild_obj = GuildId::new(community_guild_id);
    let roles_map = match community_guild_obj.roles(&bot_http).await {
        Ok(r) => r,
        Err(_) => HashMap::new(),
    };

    let staff_guild_obj = GuildId::new(staff_guild_id);
    let staff_members = match staff_guild_obj.members(&bot_http, None, None).await {
        Ok(m) => m,
        Err(_) => Vec::new(),
    };
    let staff_map: HashMap<String, String> = staff_members
        .into_iter()
        .map(|m| (m.user.id.to_string(), m.user.name.clone()))
        .collect();

    match get_all_banned_users(&community_guild_id.to_string(), &pool).await {
        Ok(mut users) => {
            for user in &mut users {
                // Resolve roles
                user.roles = user
                    .roles
                    .iter()
                    .map(|role_id_str| {
                        if let Ok(id) = role_id_str.parse::<u64>() {
                            if let Some(role) = roles_map.get(&serenity::all::RoleId::new(id)) {
                                return role.name.clone();
                            }
                        }
                        role_id_str.clone()
                    })
                    .collect();

                // Resolve banned_by
                if let Some(banned_by_id) = &user.banned_by {
                    if let Some(staff_name) = staff_map.get(banned_by_id) {
                        user.banned_by = Some(staff_name.clone());
                    }
                }
            }
            (StatusCode::OK, Json(users)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to fetch banned users: {}", e)})),
        )
            .into_response(),
    }
}
