use crate::config::Config;
use crate::db::operations::{get_thread_id_by_user_id, is_user_left, get_staff_alerts_for_user, mark_alert_as_used};
use crate::utils::format_ticket_message::Sender::User;
use crate::utils::format_ticket_message::{TicketMessage, format_ticket_message_with_destination, MessageDestination};
use serenity::all::{ChannelId, Context, CreateMessage, Message, GuildId, CreateAttachment};
use crate::i18n::get_translated_message;
use std::collections::HashMap;
use crate::utils::message_builder::{MessageBuilder};

fn extract_message_content_with_media(msg: &Message) -> (String, Vec<String>) {
    let content = msg.content.clone();
    let mut attachment_urls = Vec::new();

    for attachment in &msg.attachments {
        attachment_urls.push(attachment.url.clone());
    }

    for embed in &msg.embeds {
        if let Some(image) = &embed.image {
            let url = &image.url;
            if url.contains(".gif") || url.contains("tenor.com") || url.contains("giphy.com") {
                attachment_urls.push(url.clone());
            }
        }
        
        if let Some(thumbnail) = &embed.thumbnail {
            let url = &thumbnail.url;
            if url.contains(".gif") || url.contains("tenor.com") || url.contains("giphy.com") {
                attachment_urls.push(url.clone());
            }
        }
    }

    (content, attachment_urls)
}

async fn download_attachment(url: &str) -> Option<CreateAttachment> {
    match reqwest::get(url).await {
        Ok(response) => {
            match response.bytes().await {
                Ok(bytes) => {
                    let filename = if let Some(last_slash) = url.rfind('/') {
                        let name_part = &url[last_slash + 1..];
                        if let Some(question_mark) = name_part.find('?') {
                            &name_part[..question_mark]
                        } else {
                            name_part
                        }
                    } else {
                        "attachment"
                    };
                    
                    let clean_filename = if filename.is_empty() || filename == "attachment" {
                        "attachment"
                    } else {
                        filename
                    };
                    
                    Some(CreateAttachment::bytes(bytes, clean_filename))
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

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

    let (content, attachment_urls) = extract_message_content_with_media(msg);

    let mut attachments: Vec<CreateAttachment> = Vec::new();
    for url in &attachment_urls {
        if let Some(a) = download_attachment(url).await { attachments.push(a); }
    }

    let thread_id = match get_thread_id_by_user_id(msg.author.id, pool).await {
        Some(thread_id) => thread_id,
        None => {
            eprintln!("Failed to get thread ID");
            return Err(serenity::Error::Other("Failed to get thread ID"));
        }
    };

    let builder = MessageBuilder::begin_user_incoming(ctx, config, thread_id.clone(), msg)
        .to_thread(channel_id)
        .content(content)
        .add_attachments(attachments)
        .anonymous(is_anonymous);

    let sent_msg = match builder.send_and_record(pool).await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            return Err(e);
        }
    };

    let user_id = msg.author.id.get() as i64;
    if let Ok(alerts) = get_staff_alerts_for_user(user_id, pool).await {
        if !alerts.is_empty() {
            let mut ping_mentions = String::new();
            for staff_id in &alerts {
                ping_mentions.push_str(&format!("<@{}> ", staff_id));
            }

            let mut params = HashMap::new();
            params.insert("user".to_string(), msg.author.name.clone());
            let alert_content = get_translated_message(
                config,
                "alert.ping_message",
                Some(&params),
                None,
                None,
                None,
            ).await;

            if config.thread.embedded_message {
                use crate::utils::format_ticket_message::{Sender, MessageDestination, format_ticket_message_with_destination};
                let bot_user_id = ctx.cache.current_user().id;
                let bot_username = ctx.cache.current_user().name.clone();
                let ticket_msg = format_ticket_message_with_destination(
                    ctx,
                    Sender::System {
                        user_id: bot_user_id,
                        username: bot_username,
                    },
                    &alert_content,
                    config,
                    MessageDestination::Thread,
                ).await;
                match ticket_msg {
                    TicketMessage::Plain(content) => {
                        let full_content = format!("{}{}", ping_mentions, content);
                        let _ = channel_id.send_message(&ctx.http, CreateMessage::new().content(full_content)).await;
                    }
                    TicketMessage::Embed(embed) => {
                        let _ = channel_id.send_message(&ctx.http, CreateMessage::new()
                            .content(ping_mentions)
                            .embed(embed)).await;
                    }
                }
            } else {
                let full_content = format!("{}{}", ping_mentions, alert_content);
                let _ = channel_id.send_message(&ctx.http, CreateMessage::new().content(&full_content)).await;
            }

            for staff_id in &alerts {
                let _ = mark_alert_as_used(*staff_id, user_id, pool).await;
            }
        }
    }

    Ok(sent_msg)
}
