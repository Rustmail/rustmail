use crate::config::Config;
use crate::db::operations::{
    get_next_message_number, increment_message_number, insert_staff_message,
};
use crate::errors::{ModmailResult, common};
use crate::i18n::get_translated_message;
use crate::utils::build_message_from_ticket::build_message_from_ticket;
use crate::utils::extract_reply_content::extract_reply_content;
use crate::utils::fetch_thread::fetch_thread;
use crate::utils::format_ticket_message::{
    MessageDestination, Sender::User, TicketMessage, format_ticket_message_with_destination,
};
use std::collections::HashMap;

use crate::utils::reply_intent::{ReplyIntent, build_reply_message, extract_intent};
use serenity::all::{Context, CreateMessage, GuildId, Message, UserId};

pub async fn anonreply(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let content = extract_reply_content(&msg.content, &config.command.prefix, &["anonreply", "ar"]);
    let intent = extract_intent(content, &msg.attachments).await;

    let Some(intent) = intent else {
        let error_msg = get_translated_message(
            config,
            "reply.missing_content",
            None,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
            None,
        )
        .await;
        return Err(common::validation_failed(&error_msg));
    };

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;
    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

    if !user_still_member {
        let mut params = HashMap::new();
        params.insert("username".to_string(), thread.user_name.clone());
        let error_message = get_translated_message(
            config,
            "user.left_server",
            Some(&params),
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
            None,
        )
        .await;
        let error_response = format_ticket_message_with_destination(
            ctx,
            User {
                username: msg.author.name.clone(),
                user_id: msg.author.id,
            },
            &error_message,
            config,
            MessageDestination::Thread,
        )
        .await;
        let error_message_builder = match error_response {
            TicketMessage::Plain(content) => CreateMessage::new().content(content),
            TicketMessage::Embed(embed) => CreateMessage::new().embed(embed),
        };
        let _ = msg
            .channel_id
            .send_message(&ctx.http, error_message_builder)
            .await;
        return Ok(());
    }

    let dm_channel = match user_id.create_dm_channel(&ctx.http).await {
        Ok(channel) => channel,
        Err(_) => {
            let err_msg = get_translated_message(
                config,
                "reply.user_not_found",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;
            return Err(common::user_not_found());
        }
    };

    let next_message_number = get_next_message_number(&thread.id, db_pool).await;
    if let Err(e) = increment_message_number(&thread.id, db_pool).await {
        eprintln!(
            "Erreur lors de l'incrémentation du numéro de message: {}",
            e
        );
    }

    let mut thread_msg_builder = CreateMessage::default();
    let mut dm_msg_builder = CreateMessage::default();

    match intent {
        ReplyIntent::Text(text) => {
            let thread_tmsg = build_reply_message(
                ctx,
                &msg.author.name,
                msg.author.id,
                &text,
                Some(next_message_number),
                config,
                MessageDestination::Thread,
                true,
            )
            .await;
            thread_msg_builder = build_message_from_ticket(thread_tmsg, thread_msg_builder);

            let dm_tmsg = build_reply_message(
                ctx,
                &msg.author.name,
                msg.author.id,
                &text,
                Some(next_message_number),
                config,
                MessageDestination::DirectMessage,
                true,
            )
            .await;
            dm_msg_builder = build_message_from_ticket(dm_tmsg, dm_msg_builder);
        }
        ReplyIntent::Attachments(files) => {
            for file in files.clone() {
                thread_msg_builder = thread_msg_builder.add_file(file);
            }
            for file in files {
                dm_msg_builder = dm_msg_builder.add_file(file);
            }
        }
        ReplyIntent::TextAndAttachments(text, files) => {
            let thread_tmsg = build_reply_message(
                ctx,
                &msg.author.name,
                msg.author.id,
                &text,
                Some(next_message_number),
                config,
                MessageDestination::Thread,
                true,
            )
            .await;
            thread_msg_builder = build_message_from_ticket(thread_tmsg, thread_msg_builder);

            let dm_tmsg = build_reply_message(
                ctx,
                &msg.author.name,
                msg.author.id,
                &text,
                Some(next_message_number),
                config,
                MessageDestination::DirectMessage,
                true,
            )
            .await;
            dm_msg_builder = build_message_from_ticket(dm_tmsg, dm_msg_builder);

            for file in files.clone() {
                thread_msg_builder = thread_msg_builder.add_file(file);
            }
            for file in files {
                dm_msg_builder = dm_msg_builder.add_file(file);
            }
        }
    }

    let _ = msg.delete(&ctx.http).await;

    let thread_response = match msg
        .channel_id
        .send_message(&ctx.http, thread_msg_builder)
        .await
    {
        Ok(msg) => msg,
        Err(_) => {
            let err_msg = get_translated_message(
                config,
                "reply.send_failed_thread",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;
            return Err(common::validation_failed(&err_msg));
        }
    };

    let dm_response = match dm_channel.send_message(&ctx.http, dm_msg_builder).await {
        Ok(msg) => Some(msg),
        Err(_) => {
            let err_msg = get_translated_message(
                config,
                "reply.send_failed_dm",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;
            return Err(common::validation_failed(&err_msg));
        }
    };

    if let Err(e) = insert_staff_message(
        &thread_response,
        dm_response.map(|response| response.id.to_string()),
        &thread.id,
        msg.author.id,
        false,
        db_pool,
        config,
        next_message_number,
    )
    .await
    {
        eprintln!("Error inserting staff message: {}", e);
    }

    if config.notifications.show_success_on_reply {
        if let Some(error_handler) = &config.error_handler {
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
    }

    Ok(())
}
