use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::types::*;
use crate::prelude::utils::*;
use chrono::Utc;
use serenity::all::{Context, GuildId, Message, UserId};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn reply(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let mut content = extract_reply_content(&msg.content, &config.command.prefix, &["reply", "r"]);

    if let Some(text) = &content {
        if let Some(stripped) = text.strip_prefix("{{").and_then(|s| s.strip_suffix("}}")) {
            let snippet_key = stripped.trim();
            match get_snippet_by_key(snippet_key, db_pool).await? {
                Some(snippet) => {
                    content = Some(snippet.content);
                }
                None => {
                    return Err(ModmailError::Command(CommandError::SnippetNotFound(
                        snippet_key.to_string(),
                    )));
                }
            }
        }
    }

    let intent = extract_intent(content, &msg.attachments).await;

    let Some(intent) = intent else {
        return Err(ModmailError::Message(MessageError::MessageEmpty));
    };

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;
    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

    if !user_still_member {
        return Err(ModmailError::Thread(ThreadError::UserNotInTheServer));
    }

    let next_message_number = allocate_next_message_number(&thread.id, db_pool)
        .await
        .map_err(|_| validation_failed("Failed to allocate message number"))?;

    let mut ticket_status = match get_thread_status(&thread.id, db_pool).await {
        Some(status) => status,
        None => {
            return Err(validation_failed("Failed to get thread status"));
        }
    };

    ticket_status.last_message_by = TicketAuthor::Staff;
    ticket_status.last_message_at = Utc::now().timestamp();
    update_thread_status_db(&thread.id, &ticket_status, db_pool).await?;

    let _ = msg.delete(&ctx.http).await;

    let mut sr = MessageBuilder::begin_staff_reply(
        &ctx,
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

    let (_, dm_msg_opt) = match sr.send_msg_and_record(db_pool).await {
        Ok(tuple) => tuple,
        Err(_) => {
            return Err(validation_failed("Failed to send to thread"));
        }
    };

    if dm_msg_opt.is_none() {
        return Err(ModmailError::Command(CommandError::SendDmFailed));
    }

    if config.notifications.show_success_on_reply {
        let mut params = HashMap::new();
        params.insert("number".to_string(), next_message_number.to_string());

        let _ = MessageBuilder::system_message(&ctx, config)
            .translated_content(
                "success.message_sent",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .to_channel(msg.channel_id)
            .send(true)
            .await;
    }

    Ok(())
}
