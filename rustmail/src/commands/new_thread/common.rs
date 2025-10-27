use crate::config::Config;
use crate::errors::ModmailResult;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, GuildChannel, Message, User, UserId};
use std::collections::HashMap;

pub async fn extract_user_id(msg: &Message, config: &Config) -> Option<UserId> {
    let content = msg.content.trim();
    let prefix = &config.command.prefix;

    let command_names = ["new_thread", "nt"];
    let mut found_command = None;

    for command_name in &command_names {
        if content.starts_with(&format!("{}{}", prefix, command_name)) {
            found_command = Some(command_name);
            break;
        }
    }

    if let Some(command_name) = found_command {
        let start = prefix.len() + command_name.len();
        let args = content[start..].trim();

        if args.is_empty() {
            return None;
        }

        if let Ok(user_id) = args.parse::<u64>() {
            return Some(UserId::new(user_id));
        }

        if args.starts_with("<@") && args.ends_with(">") {
            let id_str = &args[2..args.len() - 1];
            let clean_id = id_str.trim_start_matches('!');
            if let Ok(user_id) = clean_id.parse::<u64>() {
                return Some(UserId::new(user_id));
            }
        }
    }

    None
}

pub async fn send_welcome_message(
    ctx: &Context,
    channel: &GuildChannel,
    config: &Config,
    user: &User,
) {
    let mut params = HashMap::new();
    params.insert("user".to_string(), user.name.clone());

    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content(
            "new_thread.welcome_message",
            Some(&params),
            None,
            Some(channel.guild_id.get()),
        )
        .await
        .to_channel(channel.id)
        .send(true)
        .await;

    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content(
            "new_thread.welcome_message",
            Some(&params),
            None,
            Some(channel.guild_id.get()),
        )
        .await
        .to_user(user.id)
        .send(true)
        .await;
}

pub async fn send_dm_to_user(ctx: &Context, user: &User, config: &Config) -> ModmailResult<()> {
    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content("new_thread.dm_notification", None, Some(user.id), None)
        .await
        .to_user(user.id)
        .send(true)
        .await;

    Ok(())
}

pub async fn send_error_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    error_key: &str,
    params: Option<&HashMap<String, String>>,
) {
    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content(
            error_key,
            params,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await
        .to_channel(msg.channel_id)
        .send(true)
        .await;
}

pub async fn send_success_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    user: &User,
    channel: &GuildChannel,
    dm_sent: bool,
) {
    let mut params = HashMap::new();
    params.insert("user".to_string(), user.name.clone());
    params.insert("channel_id".to_string(), channel.to_string());
    params.insert("staff".to_string(), msg.author.name.clone());

    let success_key = if dm_sent {
        "new_thread.success_with_dm"
    } else {
        "new_thread.success_without_dm"
    };

    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content(
            success_key,
            Some(&params),
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await
        .to_channel(msg.channel_id)
        .send(true)
        .await;
}
