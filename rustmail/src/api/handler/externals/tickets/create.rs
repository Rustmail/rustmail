use crate::db::repr::{ApiKey, Permission};
use crate::prelude::api::*;
use crate::prelude::db::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use crate::types::{BotCommand, BotState};
use axum::Json;
use axum::extract::{Extension, State};
use axum::http::StatusCode;
use rustmail_types::CreateTicket;
use serenity::all::{ChannelId, CreateChannel, GuildId, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_external_ticket_create(
    Extension(api_key): Extension<ApiKey>,
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(update): Json<CreateTicket>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    check_permission(&api_key, Permission::CreateTicket)
        .map_err(|e| (StatusCode::FORBIDDEN, format!("{:?}", e)))?;

    let user_id_u64 = update.discord_id.parse::<u64>().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Invalid Discord ID format".to_string(),
        )
    })?;

    let user_id = UserId::new(user_id_u64);

    let (mut config, db_pool, bot_http, command_tx) = {
        let state = bot_state.lock().await;
        let config = state
            .config
            .as_ref()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Configuration not loaded".to_string(),
            ))?
            .clone();
        let db_pool = state
            .db_pool
            .as_ref()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database not available".to_string(),
            ))?
            .clone();
        let bot_http = state
            .bot_http
            .as_ref()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Bot HTTP client not available".to_string(),
            ))?
            .clone();
        let command_tx = state.command_tx.clone();

        (config, db_pool, bot_http, command_tx)
    };

    config.db_pool = Some(db_pool.clone());

    println!(
        "API Key #{} creating ticket for Discord ID: {}",
        api_key.id, user_id_u64
    );

    let user = bot_http.get_user(user_id).await.map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            format!("Discord user not found: {}", e),
        )
    })?;

    if user.bot {
        return Err((
            StatusCode::BAD_REQUEST,
            "Cannot create ticket for bot users".to_string(),
        ));
    }

    let (tx, rx) = tokio::sync::oneshot::channel();
    command_tx
        .send(BotCommand::CheckUserIsMember {
            user_id: user_id_u64,
            resp: tx,
        })
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to communicate with bot".to_string(),
            )
        })?;

    let is_member = rx.await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to check guild membership".to_string(),
        )
    })?;

    if !is_member {
        return Err((
            StatusCode::FORBIDDEN,
            "User is not a member of the community guild".to_string(),
        ));
    }

    if thread_exists(user_id, &db_pool).await {
        return if let Some(channel_id_str) = get_thread_channel_by_user_id(user_id, &db_pool).await
        {
            Err((
                StatusCode::CONFLICT,
                format!("User already has an active ticket: <#{}>", channel_id_str),
            ))
        } else {
            Err((
                StatusCode::CONFLICT,
                "User already has an active ticket".to_string(),
            ))
        };
    }

    let username = user.name.clone();
    let thread_name = format!("🔴・{}・0m", username);
    let staff_guild_id = GuildId::new(config.bot.get_staff_guild_id());
    let inbox_category_id = ChannelId::new(config.thread.inbox_category_id);

    let channel_builder = CreateChannel::new(&thread_name).category(inbox_category_id);

    let channel = staff_guild_id
        .create_channel(&bot_http, channel_builder)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create Discord channel: {}", e),
            )
        })?;

    create_thread_for_user(&channel, user_id_u64 as i64, &username, &db_pool)
        .await
        .map_err(|e| {
            let http_clone = bot_http.clone();
            let channel_id = channel.id;
            tokio::spawn(async move {
                let _ = http_clone.delete_channel(channel_id, None).await;
            });
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create thread record: {}", e),
            )
        })?;

    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let member_join_date = community_guild_id
        .member(&bot_http, user_id)
        .await
        .ok()
        .and_then(|m| m.joined_at)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let logs_count = match get_logs_from_user_id(&user_id.to_string(), &db_pool).await {
        Ok(logs) => logs.len(),
        Err(_) => 0,
    };

    let params = {
        let mut p = HashMap::new();
        p.insert("logs_count".to_string(), logs_count.to_string());
        p.insert("prefix".to_string(), config.command.prefix.clone());
        p
    };

    let logs_info = get_translated_message(
        &config,
        "new_thread.show_logs",
        Some(&params),
        None,
        None,
        None,
    )
    .await;

    let open_thread_message = get_user_recap(user_id, &username, &member_join_date, &logs_info);

    let ctx = {
        let state = bot_state.lock().await;
        let ctx_lock = state.bot_context.read().await;
        ctx_lock
            .as_ref()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Bot context not available".to_string(),
            ))?
            .clone()
    };

    if let Err(e) = MessageBuilder::system_message(&ctx, &config)
        .to_channel(channel.id)
        .content(open_thread_message)
        .send(true)
        .await
    {
        eprintln!("Failed to send message to channel via MessageBuilder: {:?}", e);
    }

    if let Err(e) = MessageBuilder::system_message(&ctx, &config)
        .content(&config.bot.welcome_message)
        .to_user(user_id)
        .send(true)
        .await
    {
        eprintln!("Failed to send DM via MessageBuilder: {:?}", e);
    }

    println!(
        "API Key #{} successfully created ticket for user {} (channel: {})",
        api_key.id, username, channel.id
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "channel_id": channel.id.to_string(),
        "user_id": user_id_u64.to_string(),
        "username": username,
        "message": "Ticket created successfully"
    })))
}
