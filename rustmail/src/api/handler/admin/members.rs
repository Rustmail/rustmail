use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serenity::all::GuildId;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberInfo {
    pub user_id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
}

pub async fn handle_list_members(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> impl IntoResponse {
    let (guild_id, bot_http) = {
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
        (config.bot.get_staff_guild_id(), http)
    };

    let guild_id_obj = GuildId::new(guild_id);

    let members = match guild_id_obj.members(bot_http, None, None).await {
        Ok(m) => m,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to fetch members: {}", e)})),
            )
                .into_response();
        }
    };

    let member_infos: Vec<MemberInfo> = members
        .iter()
        .map(|m| MemberInfo {
            user_id: m.user.id.to_string(),
            username: m.user.name.clone(),
            discriminator: m
                .user
                .discriminator
                .map(|d| d.to_string())
                .unwrap_or_else(|| "0".to_string()),
            avatar: m.user.avatar.as_ref().map(|a| a.to_string()),
            roles: m.roles.iter().map(|r| r.to_string()).collect(),
        })
        .collect();

    (StatusCode::OK, Json(member_infos)).into_response()
}
