use crate::config::Config;
use crate::db::operations::{
    allocate_next_message_number,
};
use crate::errors::{ModmailResult, common, ModmailError, ThreadError};
use crate::utils::extract_reply_content::extract_reply_content;
use crate::utils::fetch_thread::fetch_thread;
use std::collections::HashMap;
use serenity::all::{Attachment, Context, CreateAttachment, GuildId, Message, UserId};
use crate::errors::MessageError::MessageEmpty;
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
        return Err(ModmailError::Message(MessageEmpty));
    };

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;
    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

    if !user_still_member {
        return Err(ModmailError::Thread(ThreadError::UserNotInTheServer));
    }

    let next_message_number = allocate_next_message_number(&thread.id, db_pool).await
        .map_err(|_| common::validation_failed("Failed to allocate message number"))?;

    let _ = msg.delete(&ctx.http).await;

    let mut sr = MessageBuilder::begin_staff_reply(
        ctx,
        config,
        thread.id.clone(),
        msg.author.id,
        msg.author.name.clone(),
        next_message_number,
    )
    .to_thread(msg.channel_id)
    .to_user(user_id);

    match intent {
        ReplyIntent::Text(text) => {
            sr = sr.content(text);
        }
        ReplyIntent::Attachments(files) => {
            sr = sr.add_attachments(files);
        }
        ReplyIntent::TextAndAttachments(text, files) => {
            sr = sr.content(text).add_attachments(files);
        }
    }

    let (thread_msg, dm_msg_opt) = match sr.send_and_record(db_pool).await {
        Ok(tuple) => tuple,
        Err(_) => {
            MessageBuilder::system_message(ctx, config)
                .translated_content(
                    "reply.send_failed_thread",
                    None,
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                ).await
                .to_channel(msg.channel_id)
                .send_and_forget()
                .await;
            return Err(common::validation_failed("Failed to send to thread"));
        }
    };

    if dm_msg_opt.is_none() {
        MessageBuilder::system_message(ctx, config)
            .translated_content(
                "reply.send_failed_dm",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            ).await
            .to_channel(thread_msg.channel_id)
            .send_and_forget()
            .await;
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
