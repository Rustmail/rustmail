use crate::config::Config;
use crate::db::operations::{get_message_ids_by_number, get_user_id_from_channel_id};
use crate::utils::format_ticket_message::{Sender, TicketMessage, format_ticket_message_with_destination, MessageDestination};
use serenity::all::{Context, EditMessage, Message, MessageId, UserId};

#[derive(Debug)]
pub enum EditResult {
    Success,
    PartialSuccess(String),
    Failure(String),
}

impl EditResult {
    pub async fn send_feedback(&self, ctx: &Context, msg: &Message) {
        let message = match self {
            EditResult::Success => "✅ Message modifié avec succès dans le thread et en DM.",
            EditResult::PartialSuccess(warning) => warning,
            EditResult::Failure(error) => error,
        };

        let _ = msg.reply(ctx, message).await;
    }
}

pub async fn get_message_ids(
    message_number: i64,
    user_id: UserId,
    pool: &sqlx::SqlitePool,
) -> Result<(Option<String>, Option<String>), String> {
    match get_message_ids_by_number(message_number, user_id, pool).await {
        Some(message_ids) => Ok((message_ids.dm_message_id, message_ids.inbox_message_id)),
        None => Err("❌ Aucun message trouvé avec ce numéro.".to_string()),
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
) -> Result<(), String> {
    let message_id = inbox_msg_id
        .parse::<u64>()
        .map_err(|_| "❌ ID de message invalide pour le thread.".to_string())?;

    let message_id = MessageId::new(message_id);

    let edit_message = match thread_message {
        TicketMessage::Plain(text) => EditMessage::new().content(text),
        TicketMessage::Embed(embed) => EditMessage::new().embed(embed.clone()),
    };

    channel_id
        .edit_message(&ctx.http, message_id, edit_message)
        .await
        .map_err(|e| {
            eprintln!("Failed to edit inbox message: {:?}", e);
            "❌ Erreur lors de la modification du message dans le thread.".to_string()
        })?;

    Ok(())
}

pub async fn edit_dm_message(
    ctx: &Context,
    dm_msg_id: &str,
    channel_id: serenity::all::ChannelId,
    dm_message: &TicketMessage,
    pool: &sqlx::SqlitePool,
) -> Result<(), String> {
    let message_id = dm_msg_id
        .parse::<u64>()
        .map_err(|_| "❌ ID de message DM invalide.".to_string())?;

    let message_id = MessageId::new(message_id);

    let thread_user_id = get_user_id_from_channel_id(&channel_id.to_string(), pool)
        .await
        .ok_or_else(|| "❌ Thread introuvable pour ce canal.".to_string())?;

    let user_id = UserId::new(thread_user_id as u64);

    let dm_channel = user_id.create_dm_channel(&ctx.http).await.map_err(|e| {
        eprintln!("Failed to create DM channel: {:?}", e);
        "⚠️ Impossible d'accéder au DM de l'utilisateur.".to_string()
    })?;

    let edit_message = match dm_message {
        TicketMessage::Plain(text) => EditMessage::new().content(text),
        TicketMessage::Embed(embed) => EditMessage::new().embed(embed.clone()),
    };

    dm_channel
        .edit_message(&ctx.http, message_id, edit_message)
        .await
        .map_err(|e| {
            eprintln!("Failed to edit DM message: {:?}", e);
            "⚠️ Échec de la modification en DM.".to_string()
        })?;

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
) -> EditResult {
    let mut inbox_success = false;
    let mut dm_success = false;
    let mut warnings = Vec::new();

    if let Some(inbox_id) = inbox_msg_id {
        match edit_inbox_message(ctx, channel_id, &inbox_id, thread_message).await {
            Ok(()) => inbox_success = true,
            Err(error) => {
                warnings.push(error);
            }
        }
    }

    if let Some(dm_id) = dm_msg_id {
        match edit_dm_message(ctx, &dm_id, channel_id, dm_message, pool).await {
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
