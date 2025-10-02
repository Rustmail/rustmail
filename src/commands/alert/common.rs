use crate::config::Config;
use crate::db::{cancel_alert_for_staff, get_user_id_from_channel_id, set_alert_for_staff};
use crate::errors::DatabaseError::QueryFailed;
use crate::errors::DiscordError::ApiError;
use crate::errors::{ModmailError, ModmailResult, common};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::colours::branding::GREEN;
use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, Message};
use std::collections::HashMap;

pub async fn get_thread_user_id_from_msg(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<i64> {
    let channel_id = msg.channel_id.to_string();

    match get_user_id_from_channel_id(&channel_id, pool).await {
        Some(uid) => Ok(uid),
        None => {
            send_alert_message(ctx, msg, config, "alert.not_in_thread", None).await;
            Err(common::validation_failed("Not in a thread"))
        }
    }
}

pub async fn get_thread_user_id_from_command(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<i64> {
    let channel_id = command.channel_id.to_string();

    match get_user_id_from_channel_id(&channel_id, pool).await {
        Some(uid) => Ok(uid),
        None => {
            let bot_user = match ctx.http.get_current_user().await {
                Ok(user) => user,
                Err(e) => return Err(ModmailError::Discord(ApiError(e.to_string()))),
            };

            let bot_user_id = ctx.cache.current_user().id;

            let response =
                MessageBuilder::staff_message(ctx, config, bot_user_id, bot_user.name.clone())
                    .translated_content(
                        "alert.not_in_thread",
                        None,
                        Some(command.user.id),
                        command.guild_id.map(|g| g.get()),
                    )
                    .await
                    .to_channel(command.channel_id)
                    .color(GREEN.0)
                    .build_interaction_message()
                    .await;

            let _ = command
                .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                .await;

            Err(common::validation_failed("Not in a thread"))
        }
    }
}

pub async fn handle_cancel_alert_from_msg(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    user_id: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    if let Err(e) = cancel_alert_for_staff(msg.author.id, user_id, pool).await {
        eprintln!("Failed to cancel alert: {}", e);
        send_alert_message(ctx, msg, config, "alert.cancel_failed", None).await;
        return Ok(());
    }

    let mut params = HashMap::new();
    params.insert("user".to_string(), format!("<@{}>", user_id));

    send_alert_message(ctx, msg, config, "alert.cancel_confirmation", Some(&params)).await;
    Ok(())
}

pub async fn handle_cancel_alert_from_command(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
    user_id: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    if let Err(e) = cancel_alert_for_staff(command.user.id, user_id, pool).await {
        Err(ModmailError::Database(QueryFailed(e.to_string())))
    } else {
        let mut params = HashMap::new();
        params.insert("user".to_string(), format!("<@{}>", user_id));

        let response = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "alert.cancel_confirmation",
                Some(&params),
                Some(command.user.id),
                command.guild_id.map(|g| g.get()),
            )
            .await
            .to_channel(command.channel_id)
            .build_interaction_message()
            .await;

        let _ = command
            .create_response(&ctx.http, CreateInteractionResponse::Message(response))
            .await;

        Ok(())
    }
}

pub async fn handle_set_alert_from_msg(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    user_id: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    if let Err(e) = set_alert_for_staff(msg.author.id, user_id, pool).await {
        eprintln!("Failed to set alert: {}", e);
        send_alert_message(ctx, msg, config, "alert.set_failed", None).await;
        return Ok(());
    }

    let mut params = HashMap::new();
    params.insert("user".to_string(), format!("<@{}>", user_id));

    send_alert_message(ctx, msg, config, "alert.confirmation", Some(&params)).await;
    Ok(())
}

pub async fn handle_set_alert_from_command(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
    user_id: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    if let Err(e) = set_alert_for_staff(command.user.id, user_id, pool).await {
        Err(ModmailError::Database(QueryFailed(e.to_string())))
    } else {
        let mut params = HashMap::new();
        params.insert("user".to_string(), format!("<@{}>", user_id));

        let response = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "alert.confirmation",
                Some(&params),
                Some(command.user.id),
                command.guild_id.map(|g| g.get()),
            )
            .await
            .to_channel(command.channel_id)
            .build_interaction_message()
            .await;

        let _ = command
            .create_response(&ctx.http, CreateInteractionResponse::Message(response))
            .await;

        Ok(())
    }
}

pub async fn send_alert_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    message_key: &str,
    params: Option<&HashMap<String, String>>,
) {
    let bot_user = match ctx.http.get_current_user().await {
        Ok(user) => user,
        Err(_) => return,
    };

    let bot_user_id = ctx.cache.current_user().id;

    let _ = MessageBuilder::staff_message(ctx, config, bot_user_id, bot_user.name.clone())
        .translated_content(
            message_key,
            params,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await
        .to_channel(msg.channel_id)
        .color(GREEN.0)
        .send()
        .await;
}

pub async fn extract_alert_action(msg: &Message, config: &Config) -> bool {
    let content = msg.content.trim();
    let prefix = &config.command.prefix;
    let command_name = "alert";

    if content.starts_with(&format!("{}{}", prefix, command_name)) {
        let start = prefix.len() + command_name.len();
        let args = content[start..].trim();

        args.to_lowercase() == "cancel"
    } else {
        false
    }
}
