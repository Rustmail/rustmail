use crate::config::Config;
use crate::errors::{ModmailResult, common};
use crate::modules::message_recovery::recover_missing_messages;
use crate::i18n::get_translated_message;
use crate::utils::format_ticket_message::{Sender, format_ticket_message_with_destination, MessageDestination};
use crate::utils::build_message_from_ticket::build_message_from_ticket;
use serenity::all::{Context, Message, CreateMessage};
use std::collections::HashMap;

pub async fn recover(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let _ = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let mut params = HashMap::new();
    params.insert("user".to_string(), msg.author.name.clone());
    
    let confirmation_message = get_translated_message(
        config,
        "recovery.started",
        Some(&params),
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let bot_user_id = ctx.cache.current_user().id;
    let bot_username = ctx.cache.current_user().name.clone();
    
    let confirmation_ticket = format_ticket_message_with_destination(
        ctx,
        Sender::System {
            user_id: bot_user_id,
            username: bot_username.clone(),
        },
        &confirmation_message,
        config,
        MessageDestination::Thread,
    )
    .await;

    let mut message_builder = CreateMessage::new();
    message_builder = build_message_from_ticket(confirmation_ticket, message_builder);
    
    let _ = msg.channel_id.send_message(&ctx.http, message_builder).await;

    let ctx_clone = ctx.clone();
    let config_clone = config.clone();
    let channel_id = msg.channel_id;

    tokio::spawn(async move {
        let recovery_results = recover_missing_messages(&ctx_clone, &config_clone).await;
        
        let total_recovered: u32 = recovery_results.iter().map(|r| r.recovered_count).sum();
        let successful_threads = recovery_results.iter().filter(|r| r.success).count();
        let failed_threads = recovery_results.len() - successful_threads;

        let mut params = HashMap::new();
        params.insert("total".to_string(), total_recovered.to_string());
        params.insert("threads".to_string(), successful_threads.to_string());
        params.insert("failed".to_string(), failed_threads.to_string());

        let summary_message = get_translated_message(
            &config_clone,
            "recovery.summary",
            Some(&params),
            None,
            None,
            None,
        )
        .await;

        let summary_ticket = format_ticket_message_with_destination(
            &ctx_clone,
            Sender::System {
                user_id: bot_user_id,
                username: bot_username,
            },
            &summary_message,
            &config_clone,
            MessageDestination::Thread,
        )
        .await;

        let mut summary_builder = CreateMessage::new();
        summary_builder = build_message_from_ticket(summary_ticket, summary_builder);
        
        let _ = channel_id.send_message(&ctx_clone.http, summary_builder).await;
    });

    Ok(())
} 