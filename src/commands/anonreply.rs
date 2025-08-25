use crate::config::Config;
use crate::db::operations::allocate_next_message_number;
use crate::errors::{ModmailResult, common};
use crate::utils::command::extract_reply_content::extract_reply_content;
use crate::utils::message::message_builder::MessageBuilder;
use crate::utils::message::reply_intent::{ReplyIntent, extract_intent};
use crate::utils::thread::fetch_thread::fetch_thread;
use serenity::all::{Context, GuildId, Message, UserId};
use std::collections::HashMap;

pub async fn anonreply(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let content = extract_reply_content(&msg.content, &config.command.prefix, &["anonreply", "ar"]);
    let intent = extract_intent(content, &msg.attachments).await;

    let Some(intent) = intent else {
        MessageBuilder::system_message(ctx, config)
            .translated_content(
                "reply.missing_content",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .color(0xFF0000)
            .reply_to(msg.clone())
            .send_and_forget()
            .await;

        return Err(common::validation_failed("Missing content"));
    };

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;
    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

    if !user_still_member {
        let mut params = HashMap::new();
        params.insert("username".to_string(), thread.user_name.clone());

        MessageBuilder::user_message(ctx, config, msg.author.id, msg.author.name.clone())
            .translated_content(
                "user.left_server",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .to_channel(msg.channel_id)
            .send_and_forget()
            .await;

        return Ok(());
    }

    let next_message_number = allocate_next_message_number(&thread.id, db_pool)
        .await
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
    .anonymous(true)
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
                )
                .await
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
            )
            .await
            .to_channel(thread_msg.channel_id)
            .send_and_forget()
            .await;
    }

    if config.notifications.show_success_on_reply
        && let Some(error_handler) = &config.error_handler {
            let mut params = HashMap::new();
            params.insert("number".to_string(), next_message_number.to_string());
            let _ = error_handler
                .send_success_message(
                    ctx,
                    msg.channel_id,
                    "success.message_sent",
                    Some(params),
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                )
                .await;
        }

    Ok(())
}
