use crate::config::Config;
use crate::db::operations::{get_thread_id_by_user_id, insert_user_message, is_user_left};
use crate::utils::format_ticket_message::Sender::User;
use crate::utils::format_ticket_message::{TicketMessage, format_ticket_message_with_destination, MessageDestination};
use serenity::all::{ChannelId, Context, CreateMessage, Message, GuildId};
use crate::i18n::get_translated_message;
use std::collections::HashMap;

pub async fn send_to_thread(
    ctx: &Context,
    channel_id: ChannelId,
    msg: &Message,
    config: &Config,
    is_anonymous: bool,
) -> serenity::Result<Message> {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            eprintln!("Database pool is not set in config.");
            return Err(serenity::Error::Other("Database pool not available"));
        }
    };

    if let Ok(user_left) = is_user_left(&channel_id.to_string(), pool).await {
        if user_left {
            let mut params = HashMap::new();
            params.insert("username".to_string(), msg.author.name.clone());
            
            let error_message = get_translated_message(
                config,
                "user.left_server",
                Some(&params),
                Some(msg.author.id),
                None,
                None
            ).await;
            
            let error_ticket = format_ticket_message_with_destination(
                ctx,
                User {
                    username: msg.author.name.clone(),
                    user_id: msg.author.id,
                },
                &error_message,
                config,
                MessageDestination::Thread,
            ).await;

            match error_ticket {
                TicketMessage::Plain(content) => {
                    return channel_id.say(&ctx.http, content).await;
                }
                TicketMessage::Embed(embed) => {
                    return channel_id
                        .send_message(&ctx.http, CreateMessage::new().embed(embed))
                        .await;
                }
            }
        }
    }

    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    if let Err(_) = community_guild_id.member(&ctx.http, msg.author.id).await {
        let mut params = HashMap::new();
        params.insert("username".to_string(), msg.author.name.clone());
        
        let error_message = get_translated_message(
            config,
            "user.left_server",
            Some(&params),
            Some(msg.author.id),
            None,
            None
        ).await;
        
        let error_ticket = format_ticket_message_with_destination(
            ctx,
            User {
                username: msg.author.name.clone(),
                user_id: msg.author.id,
            },
            &error_message,
            config,
            MessageDestination::Thread,
        ).await;

        match error_ticket {
            TicketMessage::Plain(content) => {
                return channel_id.say(&ctx.http, content).await;
            }
            TicketMessage::Embed(embed) => {
                return channel_id
                    .send_message(&ctx.http, CreateMessage::new().embed(embed))
                    .await;
            }
        }
    }

    let ticket_msg = format_ticket_message_with_destination(
        ctx,
        User {
            username: msg.author.name.clone(),
            user_id: msg.author.id,
        },
        &msg.content,
        config,
        MessageDestination::DirectMessage,
    )
    .await;

    let sent_msg = match ticket_msg {
        TicketMessage::Plain(content) => channel_id.say(&ctx.http, content).await,
        TicketMessage::Embed(embed) => {
            channel_id
                .send_message(&ctx.http, CreateMessage::new().embed(embed))
                .await
        }
    };

    let sent_msg = match sent_msg {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("Failed to send message: {}", err);
            return Err(err);
        }
    };

    let thread_id = match get_thread_id_by_user_id(msg.author.id, pool).await {
        Some(thread_id) => thread_id,
        None => {
            eprintln!("Failed to get thread ID");
            return Ok(sent_msg);
        }
    };

    if let Err(e) = insert_user_message(
        &sent_msg,
        &thread_id,
        is_anonymous,
        pool,
        config,
    ).await {
        eprintln!("Error inserting user message: {}", e);
    }

    Ok(sent_msg)
}
