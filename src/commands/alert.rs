use crate::config::Config;
use crate::errors::{ModmailResult, common};
use crate::db::operations::{get_user_id_from_channel_id, set_alert_for_staff, cancel_alert_for_staff};
use crate::i18n::get_translated_message;
use crate::utils::format_ticket_message::{Sender, format_ticket_message_with_destination, MessageDestination};
use crate::utils::build_message_from_ticket::build_message_from_ticket;
use serenity::all::{Context, Message, CreateMessage};
use std::collections::HashMap;

pub async fn alert(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let user_id = get_thread_user_id(ctx, msg, config, pool).await?;
    let is_cancel = extract_alert_action(msg, config).await;
    
    if is_cancel {
        handle_cancel_alert(ctx, msg, config, user_id, pool).await
    } else {
        handle_set_alert(ctx, msg, config, user_id, pool).await
    }
}

async fn get_thread_user_id(
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

async fn handle_cancel_alert(
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

async fn handle_set_alert(
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

async fn send_alert_message(
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

    let message_content = get_translated_message(
        config,
        message_key,
        params,
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;
    
    let ticket_message = format_ticket_message_with_destination(
        ctx,
        Sender::System {
            user_id: bot_user_id,
            username: bot_user.name.clone(),
        },
        &message_content,
        config,
        MessageDestination::Thread,
    )
    .await;

    let mut message_builder = CreateMessage::default();
    message_builder = build_message_from_ticket(ticket_message, message_builder);
    
    let _ = msg.channel_id.send_message(&ctx.http, message_builder).await;
}

async fn extract_alert_action(msg: &Message, config: &Config) -> bool {
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