use crate::config::Config;
use crate::db::operations::{
    get_next_message_number, increment_message_number, insert_staff_message,
};
use crate::errors::{ModmailResult, common};
use crate::utils::extract_reply_content::extract_reply_content;
use crate::utils::fetch_thread::fetch_thread;
use std::collections::HashMap;
use serenity::all::{Attachment, Context, CreateAttachment, GuildId, Message, UserId};
use crate::utils::hex_string_to_int::hex_string_to_int;
use crate::utils::message_builder::MessageBuilder;

enum ReplyIntent {
    Text(String),
    Attachments(Vec<CreateAttachment>),
    TextAndAttachments(String, Vec<CreateAttachment>),
}

async fn extract_intent(
    content: Option<String>,
    attachments: &[Attachment],
) -> Option<ReplyIntent> {
    let attachments = download_attachments(attachments).await;

    match (content, attachments.is_empty()) {
        (Some(c), true) => Some(ReplyIntent::Text(c)),
        (None, false) => Some(ReplyIntent::Attachments(attachments)),
        (Some(c), false) => Some(ReplyIntent::TextAndAttachments(c, attachments)),
        (None, true) => None,
    }
}

async fn download_attachments(attachments: &[Attachment]) -> Vec<CreateAttachment> {
    let mut downloaded_attachments = Vec::new();

    for attachment in attachments {
        if let Ok(response) = reqwest::get(&attachment.url).await {
            if let Ok(bytes) = response.bytes().await {
                downloaded_attachments
                    .push(CreateAttachment::bytes(bytes, attachment.filename.clone()));
            } else {
                eprintln!(
                    "Failed to read bytes from attachment: {}",
                    attachment.filename
                );
            }
        } else {
            eprintln!("Failed to download attachment: {}", attachment.filename);
        }
    }

    downloaded_attachments
}

pub async fn reply(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let content = extract_reply_content(&msg.content, &config.command.prefix, &["reply", "r"]);
    let intent = extract_intent(content, &msg.attachments).await;

    let Some(intent) = intent else {
        MessageBuilder::system_message(ctx, config)
            .translated_content(
                "reply.missing_content",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            ).await
            .color(0xFF0000)
            .reply_to(msg.clone())
            .send_and_forget()
            .await;

        return Err(common::validation_failed("Missing content"));
    };

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;
    let username: String = thread.user_name;
    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

    if !user_still_member {
        let mut params = HashMap::new();
        params.insert("username".to_string(), username.clone());

        MessageBuilder::user_message(ctx, config, msg.author.id, msg.author.name.clone())
            .translated_content(
                "user.left_server",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            ).await
            .to_channel(msg.channel_id)
            .send_and_forget()
            .await;

        return Ok(());
    }

    let next_message_number = get_next_message_number(&thread.id, db_pool).await;

    if let Err(e) = increment_message_number(&thread.id, db_pool).await {
        eprintln!(
            "Erreur lors de l'incrémentation du numéro de message: {}",
            e
        );
    }

    let _ = msg.delete(&ctx.http).await;

    match intent {
        ReplyIntent::Text(text) => {
            let thread_response = MessageBuilder::staff_message(ctx, config, msg.author.id, msg.author.name.clone())
                .content(&text)
                .with_message_number(next_message_number)
                .to_channel(msg.channel_id)
                .send()
                .await;

            let dm_response = MessageBuilder::user_message(ctx, config, msg.author.id, msg.author.name.clone())
                .content(&text)
                .with_message_number(next_message_number)
                .to_user(user_id)
                .color(hex_string_to_int(&config.thread.staff_message_color) as u32)
                .send()
                .await;

            let thread_msg = match thread_response {
                Ok(msg) => msg,
                Err(_) => {
                    MessageBuilder::system_message(ctx, config)
                        .translated_content(
                            "reply.send_failed_thread",
                            None,
                            Some(msg.author.id),
                            msg.guild_id.map(|g| g.get()),
                        ).await
                        .color(0xFF0000)
                        .to_channel(msg.channel_id)
                        .send_and_forget()
                        .await;
                    return Err(common::validation_failed("Failed to send to thread"));
                }
            };

            let dm_msg = match dm_response {
                Ok(msg) => Some(msg.id.to_string()),
                Err(_) => {
                    MessageBuilder::system_message(ctx, config)
                        .translated_content(
                            "reply.send_failed_dm",
                            None,
                            Some(msg.author.id),
                            msg.guild_id.map(|g| g.get()),
                        ).await
                        .color(0xFFA500)
                        .to_channel(msg.channel_id)
                        .send_and_forget()
                        .await;
                    None
                }
            };

            if let Err(e) = insert_staff_message(
                &thread_msg,
                dm_msg,
                &thread.id,
                msg.author.id,
                false,
                db_pool,
                config,
                next_message_number,
            ).await {
                eprintln!("Error inserting staff message: {}", e);
            }
        }
        ReplyIntent::Attachments(files) => {
            let thread_response = MessageBuilder::staff_message(ctx, config, msg.author.id, msg.author.name.clone())
                .with_message_number(next_message_number)
                .add_attachments(files.clone())
                .to_channel(msg.channel_id)
                .send()
                .await;

            let dm_response = MessageBuilder::user_message(ctx, config, msg.author.id, msg.author.name.clone())
                .with_message_number(next_message_number)
                .add_attachments(files)
                .color(hex_string_to_int(&config.thread.staff_message_color) as u32)
                .to_user(user_id)
                .send()
                .await;

            let thread_msg = thread_response.map_err(|_| common::validation_failed("Failed to send to thread"))?;
            let dm_msg = dm_response.ok().map(|msg| msg.id.to_string());

            if let Err(e) = insert_staff_message(
                &thread_msg,
                dm_msg,
                &thread.id,
                msg.author.id,
                false,
                db_pool,
                config,
                next_message_number,
            ).await {
                eprintln!("Error inserting staff message: {}", e);
            }
        }
        ReplyIntent::TextAndAttachments(text, files) => {
            let thread_response = MessageBuilder::staff_message(ctx, config, msg.author.id, msg.author.name.clone())
                .content(&text)
                .with_message_number(next_message_number)
                .add_attachments(files.clone())
                .to_channel(msg.channel_id)
                .send()
                .await;

            let dm_response = MessageBuilder::user_message(ctx, config, msg.author.id, msg.author.name.clone())
                .content(&text)
                .with_message_number(next_message_number)
                .add_attachments(files)
                .color(hex_string_to_int(&config.thread.staff_message_color) as u32)
                .to_user(user_id)
                .send()
                .await;

            let thread_msg = thread_response.map_err(|_| common::validation_failed("Failed to send to thread"))?;
            let dm_msg = dm_response.ok().map(|msg| msg.id.to_string());

            if let Err(e) = insert_staff_message(
                &thread_msg,
                dm_msg,
                &thread.id,
                msg.author.id,
                false,
                db_pool,
                config,
                next_message_number,
            ).await {
                eprintln!("Error inserting staff message: {}", e);
            }
        }
    }

    if config.notifications.show_success_on_reply {
        let mut params = HashMap::new();
        params.insert("number".to_string(), next_message_number.to_string());

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("success.message_sent",
                                Some(&params),
                                Some(msg.author.id),
                                msg.guild_id.map(|g| g.get())).await
            .to_channel(msg.channel_id)
            .send()
            .await;
    }

    Ok(())
}
