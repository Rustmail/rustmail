use crate::config::Config;
use crate::db::operations::{get_next_message_number, insert_staff_message};
use crate::db::repr::Thread;
use crate::errors::{ModmailResult, common};
use crate::utils::extract_reply_content::extract_reply_content;
use crate::utils::fetch_thread::fetch_thread;
use crate::utils::format_ticket_message::Sender::Staff;
use crate::utils::format_ticket_message::{
    MessageDestination, TicketMessage, format_ticket_message_with_destination,
};

use serenity::all::{Attachment, Context, CreateAttachment, CreateMessage, Message, UserId};

enum ReplyIntent {
    Text(String),
    Attachments(Vec<CreateAttachment>),
    TextAndAttachments(String, Vec<CreateAttachment>),
}

async fn build_reply_message(
    ctx: &Context,
    thread: &Thread,
    content: &str,
    user_id: UserId,
    message_number: Option<u64>,
    config: &Config,
    destination: MessageDestination,
) -> TicketMessage {
    format_ticket_message_with_destination(
        ctx,
        Staff {
            username: thread.user_name.clone(),
            user_id,
            role: None,
            message_number,
        },
        content,
        config,
        destination,
    )
    .await
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

fn build_message_from_ticket(tmsg: TicketMessage, mut msg_builder: CreateMessage) -> CreateMessage {
    match tmsg {
        TicketMessage::Plain(txt) => {
            msg_builder = msg_builder.content(txt);
        }
        TicketMessage::Embed(embed) => {
            msg_builder = msg_builder.embed(embed);
        }
    }

    msg_builder
}

pub async fn reply(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let content = extract_reply_content(&msg.content, &config.command.prefix, &["reply", "r"]);
    let intent = extract_intent(content, &msg.attachments).await;

    let Some(intent) = intent else {
        return Err(common::validation_failed(
            "Please provide a message to send to the user.",
        ));
    };

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;

    let user_id = UserId::new(thread.user_id as u64);
    let dm_channel = user_id
        .create_dm_channel(&ctx.http)
        .await
        .map_err(|_| common::user_not_found())?;
    let next_message_number = get_next_message_number(&thread.id, db_pool).await;

    let mut thread_msg_builder = CreateMessage::default();
    let mut dm_msg_builder = CreateMessage::default();

    match intent {
        ReplyIntent::Text(text) => {
            let thread_tmsg = build_reply_message(
                ctx,
                &thread,
                &text,
                msg.author.id,
                Some(next_message_number),
                config,
                MessageDestination::Thread,
            )
            .await;
            thread_msg_builder = build_message_from_ticket(thread_tmsg, thread_msg_builder);

            let dm_tmsg = build_reply_message(
                ctx,
                &thread,
                &text,
                msg.author.id,
                Some(next_message_number),
                config,
                MessageDestination::DirectMessage,
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
                &thread,
                &text,
                msg.author.id,
                Some(next_message_number),
                config,
                MessageDestination::Thread,
            )
            .await;
            thread_msg_builder = build_message_from_ticket(thread_tmsg, thread_msg_builder);

            let dm_tmsg = build_reply_message(
                ctx,
                &thread,
                &text,
                msg.author.id,
                Some(next_message_number),
                config,
                MessageDestination::DirectMessage,
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

    let thread_response = msg
        .channel_id
        .send_message(&ctx.http, thread_msg_builder)
        .await
        .map_err(|_| common::validation_failed("Failed to send the message to the channel."))?;

    let dm_response = dm_channel
        .send_message(&ctx.http, dm_msg_builder)
        .await
        .map_err(|_| common::validation_failed("Failed to send the message to the user in DM."))?;

    if let Err(e) = insert_staff_message(
        &thread_response,
        Some(dm_response.id.to_string()),
        &thread.id,
        msg.author.id,
        false,
        db_pool,
        config,
    )
    .await
    {
        eprintln!("Error inserting staff message: {}", e);
    }

    Ok(())
}
