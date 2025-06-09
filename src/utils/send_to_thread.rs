use crate::config::Config;
use crate::db::operations::{get_thread_id_by_user_id, insert_user_message};
use crate::utils::format_ticket_message::Sender::User;
use crate::utils::format_ticket_message::{TicketMessage, format_ticket_message_with_destination, MessageDestination};
use serenity::all::{ChannelId, Context, CreateMessage, Message};

pub async fn send_to_thread(
    ctx: &Context,
    channel_id: ChannelId,
    msg: &Message,
    config: &Config,
    is_anonymous: bool,
) -> serenity::Result<Message> {
    let ticket_msg = format_ticket_message_with_destination(
        ctx,
        User {
            username: msg.author.name.clone(),
            user_id: msg.author.id,
        },
        &msg.content,
        config,
        MessageDestination::DirectMessage,
    )
    .await;

    let sent_msg = match ticket_msg {
        TicketMessage::Plain(content) => channel_id.say(&ctx.http, content).await,
        TicketMessage::Embed(embed) => {
            channel_id
                .send_message(&ctx.http, CreateMessage::new().embed(embed))
                .await
        }
    };

    let sent_msg = match sent_msg {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("Failed to send message: {}", err);
            return Err(err);
        }
    };

    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            eprintln!("Database pool is not set in config.");
            return Ok(sent_msg);
        }
    };

    let thread_id = match get_thread_id_by_user_id(msg.author.id, pool).await {
        Some(thread_id) => thread_id,
        None => {
            eprintln!("Failed to get thread ID");
            return Ok(sent_msg);
        }
    };

    if let Err(e) = insert_user_message(
        &sent_msg,
        &thread_id,
        is_anonymous,
        pool,
        config,
    ).await {
        eprintln!("Error inserting user message: {}", e);
    }

    Ok(sent_msg)
}
