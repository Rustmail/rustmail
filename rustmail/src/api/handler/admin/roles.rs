use crate::prelude::types::*;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serenity::all::GuildId;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleInfo {
    pub role_id: String,
    pub name: String,
    pub color: u32,
    pub position: u16,
}

pub async fn handle_list_roles(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> impl IntoResponse {
    let (guild_id, bot_http) = {
        let state_lock = bot_state.lock().await;
        let config = match &state_lock.config {
            Some(c) => c,
            None => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Config not loaded"}))).into_response(),
        };
        let http = match &state_lock.bot_http {
            Some(h) => h.clone(),
            None => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Bot not initialized"}))).into_response(),
        };
        (config.bot.get_staff_guild_id(), http)
    };

    let guild_id_obj = GuildId::new(guild_id);

    let roles = match guild_id_obj.roles(bot_http).await {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to fetch roles: {}", e)}))).into_response(),
    };

    let mut role_infos: Vec<RoleInfo> = roles.iter().map(|(id, role)| RoleInfo {
        role_id: id.to_string(),
        name: role.name.clone(),
        color: role.colour.0,
        position: role.position,
    }).collect();

    role_infos.sort_by(|a, b| b.position.cmp(&a.position));

    (StatusCode::OK, Json(role_infos)).into_response()
}
