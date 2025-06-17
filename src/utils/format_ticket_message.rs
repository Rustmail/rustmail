use std::f32::NAN;

use crate::config::Config;
use crate::utils::hex_string_to_int::hex_string_to_int;
use serenity::all::{Colour, Context, CreateEmbed, CreateEmbedAuthor, Timestamp, UserId};

pub enum Sender {
    User {
        user_id: UserId,
        username: String,
    },
    Staff {
        username: String,
        user_id: UserId,
        role: Option<String>,
        message_number: Option<u64>,
    },
    System {
        user_id: UserId,
        username: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum MessageDestination {
    Thread,
    DirectMessage,
}

pub enum TicketMessage {
    Plain(String),
    Embed(CreateEmbed),
}

async fn get_user_avatar_url(ctx: &Context, user_id: UserId) -> String {
    match user_id.to_user(&ctx.http).await {
        Ok(user) => user
            .avatar_url()
            .unwrap_or_else(|| user.default_avatar_url()),
        Err(err) => {
            eprintln!("Failed to fetch user: {:?}", err);
            String::new()
        }
    }
}

async fn create_embed_message(
    ctx: &Context,
    sender: &Sender,
    content: &str,
    config: &Config,
    destination: MessageDestination,
) -> CreateEmbed {
    let (user_id, username, color, message_number) = match sender {
        Sender::User { user_id, username } => {
            (user_id, username, &config.thread.user_message_color, None)
        }
        Sender::Staff {
            user_id,
            username,
            message_number,
            ..
        } => (
            user_id,
            username,
            &config.thread.staff_message_color,
            *message_number,
        ),
        Sender::System { user_id, username } => {
            (user_id, username, &config.thread.system_message_color, None)
        }
    };
    let avatar_url = get_user_avatar_url(ctx, *user_id).await;
    let mut embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(username).icon_url(avatar_url))
        .description(format!(">>> {}", content))
        .color(Colour::new(hex_string_to_int(color) as u32))
        .timestamp(Timestamp::now());

    if let (Some(msg_num), MessageDestination::Thread) = (message_number, destination) {
        let footer_text = format!(
            "Message #{} • {}edit {} pour modifier",
            msg_num, config.command.prefix, msg_num
        );
        embed = embed.footer(serenity::all::CreateEmbedFooter::new(footer_text));
    }

    embed
}

fn create_classic_message(
    sender: &Sender,
    content: &str,
    config: &Config,
    destination: MessageDestination,
) -> String {
    match sender {
        Sender::User { username, .. } => format!("**{}** : {}", username, content),
        Sender::System { username, .. } => format!("**{}** : {}", username, content),
        Sender::Staff {
            username,
            role,
            message_number,
            ..
        } => {
            let base_message = if let Some(role) = role {
                format!("**{}** [{}] : {}", username, role, content)
            } else {
                format!("**{}** : {}", username, content)
            };

            if let (Some(msg_num), MessageDestination::Thread) = (message_number, destination) {
                format!(
                    "{}\n\n*Message #{} - `{}edit {}` pour modifier*",
                    base_message, msg_num, config.command.prefix, msg_num
                )
            } else {
                base_message
            }
        }
    }
}

pub async fn format_ticket_message(
    ctx: &Context,
    sender: Sender,
    content: &str,
    config: &Config,
) -> TicketMessage {
    format_ticket_message_with_destination(ctx, sender, content, config, MessageDestination::Thread)
        .await
}

pub async fn format_ticket_message_with_destination(
    ctx: &Context,
    sender: Sender,
    content: &str,
    config: &Config,
    destination: MessageDestination,
) -> TicketMessage {
    if !config.thread.embedded_message {
        TicketMessage::Plain(create_classic_message(
            &sender,
            content,
            config,
            destination,
        ))
    } else {
        TicketMessage::Embed(create_embed_message(ctx, &sender, content, config, destination).await)
    }
}
