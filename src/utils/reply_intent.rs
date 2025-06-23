use crate::config::Config;
use crate::utils::format_ticket_message::{
    MessageDestination, Sender, TicketMessage, format_ticket_message_with_destination,
};
use serenity::all::{Attachment, Context, CreateAttachment, UserId};

pub enum ReplyIntent {
    Text(String),
    Attachments(Vec<CreateAttachment>),
    TextAndAttachments(String, Vec<CreateAttachment>),
}

pub async fn extract_intent(
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

pub async fn download_attachments(attachments: &[Attachment]) -> Vec<CreateAttachment> {
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

pub async fn build_reply_message(
    ctx: &Context,
    username: &str,
    user_id: UserId,
    content: &str,
    message_number: Option<u64>,
    config: &Config,
    destination: MessageDestination,
    is_anonymous: bool,
) -> TicketMessage {
    let anonymous_username = match ctx.http.get_current_user().await {
        Ok(user) => user.name.clone(),
        Err(_) => "System".to_string(),
    };
    let bot_id = match ctx.http.get_current_user().await {
        Ok(user) => user.id,
        Err(_) => user_id,
    };
    let sender = if is_anonymous {
        Sender::StaffAnonymous {
            username: anonymous_username,
            user_id: bot_id,
            role: None,
            message_number,
        }
    } else {
        Sender::Staff {
            username: username.to_string(),
            user_id,
            role: None,
            message_number,
        }
    };
    format_ticket_message_with_destination(ctx, sender, content, config, destination).await
}
