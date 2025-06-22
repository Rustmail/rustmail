use crate::config::Config;
use crate::errors::{ModmailResult, common};
use crate::db::operations::{get_user_id_from_channel_id, set_alert_for_staff};
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

    let bot_user = match ctx.http.get_current_user().await {
        Ok(user) => user,
        Err(_) => {
            return Err(common::user_not_found());
        }
    };

    let bot_user_id = ctx.cache.current_user().id;

    let channel_id = msg.channel_id.to_string();
    
    let user_id = match get_user_id_from_channel_id(&channel_id, pool).await {
        Some(uid) => uid,
        None => {
            let error_msg = get_translated_message(
                config,
                "alert.not_in_thread",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;
            
            let error_ticket = format_ticket_message_with_destination(
                ctx,
                Sender::System {
                    user_id: bot_user_id,
                    username: bot_user.name.clone(),
                },
                &error_msg,
                config,
                MessageDestination::Thread,
            )
            .await;

            let mut message_builder = CreateMessage::default();
            message_builder = build_message_from_ticket(error_ticket, message_builder);
            
            let _ = msg.channel_id.send_message(&ctx.http, message_builder).await;
            return Ok(());
        }
    };

    if let Err(e) = set_alert_for_staff(msg.author.id, user_id, pool).await {
        eprintln!("Failed to set alert: {}", e);
        let error_msg = get_translated_message(
            config,
            "alert.set_failed",
            None,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
            None,
        )
        .await;
        
        let error_ticket = format_ticket_message_with_destination(
            ctx,
            Sender::System {
                user_id: bot_user_id,
                username: bot_user.name.clone(),
            },
            &error_msg,
            config,
            MessageDestination::Thread,
        )
        .await;

        let mut message_builder = CreateMessage::default();
        message_builder = build_message_from_ticket(error_ticket, message_builder);
        
        let _ = msg.channel_id.send_message(&ctx.http, message_builder).await;
        return Ok(());
    }

    let mut params = HashMap::new();
    params.insert("user".to_string(), format!("<@{}>", user_id));
    
    let confirmation_msg = get_translated_message(
        config,
        "alert.confirmation",
        Some(&params),
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let confirmation_ticket = format_ticket_message_with_destination(
        ctx,
        Sender::System {
            user_id: bot_user_id,
            username: bot_user.name.clone(),
        },
        &confirmation_msg,
        config,
        MessageDestination::Thread,
    )
    .await;

    let mut message_builder = CreateMessage::default();
    message_builder = build_message_from_ticket(confirmation_ticket, message_builder);

    let _ = msg.channel_id.send_message(&ctx.http, message_builder).await;

    Ok(())
} 