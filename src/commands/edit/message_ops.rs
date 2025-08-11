use crate::config::Config;
use crate::db::operations::{
    get_message_ids_by_number, get_thread_by_channel_id, get_user_id_from_channel_id,
};
use crate::i18n::get_translated_message;
use crate::utils::format_ticket_message::{
    MessageDestination, Sender, TicketMessage, format_ticket_message_with_destination,
};
use serenity::all::{Context, EditMessage, Message, MessageId, UserId};
use std::collections::HashMap;

#[derive(Debug)]
pub enum EditResult {
    Success,
    PartialSuccess(String),
    Failure(String),
}

impl EditResult {
    pub async fn _send_feedback(&self, ctx: &Context, msg: &Message, config: &Config) {
        let (key, params) = match self {
            EditResult::Success => ("edit.success", None),
            EditResult::PartialSuccess(warning) => ("edit.partial_success", Some(warning)),
            EditResult::Failure(error) => ("edit.failure", Some(error)),
        };
        let mut param_map = HashMap::new();
        if let Some(text) = params {
            param_map.insert("details".to_string(), text.clone());
        }
        let message = get_translated_message(
            config,
            key,
            if params.is_some() {
                Some(&param_map)
            } else {
                None
            },
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
            None,
        )
        .await;
        let _ = msg.reply(ctx, message).await;
    }
}

pub async fn get_message_ids(
    message_number: i64,
    user_id: UserId,
    pool: &sqlx::SqlitePool,
    config: &Config,
    _ctx: &Context,
    msg: &Message,
) -> Result<(Option<String>, Option<String>), String> {
    let thread = match get_thread_by_channel_id(&msg.channel_id.to_string(), pool).await {
        Some(thread) => thread,
        None => {
            let error = get_translated_message(
                config,
                "thread.not_found",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;
            return Err(error);
        }
    };

    match get_message_ids_by_number(message_number, user_id, &thread.id, pool).await {
        Some(message_ids) => Ok((message_ids.dm_message_id, message_ids.inbox_message_id)),
        None => {
            let error = get_translated_message(
                config,
                "edit.not_found",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;
            Err(error)
        }
    }
}

pub async fn format_new_message(
    ctx: &Context,
    staff_username: &str,
    staff_user_id: UserId,
    content: &str,
    message_number: Option<u64>,
    config: &Config,
) -> (TicketMessage, TicketMessage) {
    let thread_message = format_ticket_message_with_destination(
        ctx,
        Sender::Staff {
            username: staff_username.to_string(),
            user_id: staff_user_id,
            role: None,
            message_number,
        },
        content,
        config,
        MessageDestination::Thread,
    )
    .await;

    let dm_message = format_ticket_message_with_destination(
        ctx,
        Sender::Staff {
            username: staff_username.to_string(),
            user_id: staff_user_id,
            role: None,
            message_number,
        },
        content,
        config,
        MessageDestination::DirectMessage,
    )
    .await;

    (thread_message, dm_message)
}

pub async fn edit_inbox_message(
    ctx: &Context,
    channel_id: serenity::all::ChannelId,
    inbox_msg_id: &str,
    thread_message: &TicketMessage,
    config: &Config,
    msg: &Message,
) -> Result<(), String> {
    let message_id = match inbox_msg_id.parse::<u64>() {
        Ok(id) => MessageId::new(id),
        Err(_) => {
            return Err(get_translated_message(
                config,
                "edit.invalid_id_thread",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await);
        }
    };

    let edit_message = match thread_message {
        TicketMessage::Plain(text) => EditMessage::new().content(text),
        TicketMessage::Embed(embed) => EditMessage::new().embed(embed.clone()),
    };

    if channel_id
        .edit_message(&ctx.http, message_id, edit_message)
        .await
        .is_err()
    {
        return Err(get_translated_message(
            config,
            "edit.edit_failed_thread",
            None,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
            None,
        )
        .await);
    }

    Ok(())
}

pub async fn edit_dm_message(
    ctx: &Context,
    dm_msg_id: &str,
    channel_id: serenity::all::ChannelId,
    dm_message: &TicketMessage,
    pool: &sqlx::SqlitePool,
    config: &Config,
    msg: &Message,
) -> Result<(), String> {
    let message_id = match dm_msg_id.parse::<u64>() {
        Ok(id) => MessageId::new(id),
        Err(_) => {
            return Err(get_translated_message(
                config,
                "edit.invalid_id_dm",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await);
        }
    };

    let thread_user_id = match get_user_id_from_channel_id(&channel_id.to_string(), pool).await {
        Some(id) => id,
        None => {
            return Err(get_translated_message(
                config,
                "thread.not_found",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await);
        }
    };

    let user_id = UserId::new(thread_user_id as u64);

    let dm_channel = match user_id.create_dm_channel(&ctx.http).await {
        Ok(channel) => channel,
        Err(_) => {
            return Err(get_translated_message(
                config,
                "edit.dm_access_failed",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await);
        }
    };

    let edit_message = match dm_message {
        TicketMessage::Plain(text) => EditMessage::new().content(text),
        TicketMessage::Embed(embed) => EditMessage::new().embed(embed.clone()),
    };

    if dm_channel
        .edit_message(&ctx.http, message_id, edit_message)
        .await
        .is_err()
    {
        return Err(get_translated_message(
            config,
            "edit.edit_failed_dm",
            None,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
            None,
        )
        .await);
    }

    Ok(())
}

pub async fn edit_messages(
    ctx: &Context,
    channel_id: serenity::all::ChannelId,
    dm_msg_id: Option<String>,
    inbox_msg_id: Option<String>,
    thread_message: &TicketMessage,
    dm_message: &TicketMessage,
    pool: &sqlx::SqlitePool,
    config: &Config,
    msg: &Message,
) -> EditResult {
    let mut inbox_success = false;
    let mut dm_success = false;
    let mut warnings = Vec::new();

    if let Some(inbox_id) = inbox_msg_id {
        match edit_inbox_message(ctx, channel_id, &inbox_id, thread_message, config, msg).await {
            Ok(()) => inbox_success = true,
            Err(error) => {
                warnings.push(error);
            }
        }
    }

    if let Some(dm_id) = dm_msg_id {
        match edit_dm_message(ctx, &dm_id, channel_id, dm_message, pool, config, msg).await {
            Ok(()) => dm_success = true,
            Err(error) => {
                warnings.push(error);
            }
        }
    }

    match (inbox_success, dm_success, warnings.is_empty()) {
        (true, true, true) => EditResult::Success,
        (true, true, false) => EditResult::Success,
        (true, false, _) => EditResult::PartialSuccess(
            "⚠️ Message modifié dans le thread mais échec de la modification en DM.".to_string(),
        ),
        (false, true, _) => EditResult::PartialSuccess(
            "⚠️ Message modifié en DM mais échec de la modification dans le thread.".to_string(),
        ),
        (false, false, _) => {
            let error_msg = if warnings.is_empty() {
                "❌ Échec de la modification des messages.".to_string()
            } else {
                warnings.join(" ")
            };
            EditResult::Failure(error_msg)
        }
    }
}

pub async fn cleanup_command_message(ctx: &Context, msg: &Message) {
    if let Err(e) = msg.delete(&ctx.http).await {
        eprintln!("Failed to delete command message: {:?}", e);
    }
}
