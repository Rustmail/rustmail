use crate::config::Config;
use crate::errors::{ModmailResult, common};
use crate::db::operations::{thread_exists, create_thread_for_user, get_thread_channel_by_user_id};
use crate::i18n::get_translated_message;
use serenity::all::{ChannelId, Context, GuildId, Message, UserId};
use std::collections::HashMap;
use crate::utils::message_builder::MessageBuilder;

pub async fn new_thread(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let user_id = extract_user_id(msg, config).await;
    if user_id.is_none() {
        send_error_message(ctx, msg, config, "new_thread.missing_user", None).await;
        return Ok(());
    }

    let user_id = user_id.unwrap();

    let user = match ctx.http.get_user(user_id).await {
        Ok(user) => user,
        Err(_) => {
            send_error_message(ctx, msg, config, "new_thread.user_not_found", None).await;
            return Ok(());
        }
    };

    if thread_exists(user_id, pool).await {
        if let Some(channel_id_str) = get_thread_channel_by_user_id(user_id, pool).await {
            let mut params = HashMap::new();
            params.insert("user".to_string(), user.name.clone());
            params.insert("channel_id".to_string(), channel_id_str.clone());
            
            send_error_message(ctx, msg, config, "new_thread.user_has_thread_with_link", Some(&params)).await;
        } else {
            send_error_message(ctx, msg, config, "new_thread.user_has_thread", None).await;
        }
        return Ok(());
    }

    let inbox_category_id = ChannelId::new(config.thread.inbox_category_id);
    let channel_name = format!("{}", user.name.to_lowercase().replace(" ", "-"));
    let mut channel_builder = serenity::all::CreateChannel::new(&channel_name);
    channel_builder = channel_builder
        .kind(serenity::model::channel::ChannelType::Text)
        .category(inbox_category_id)
        .topic(format!("Support thread for {}", user.name));

    let staff_guild_id = GuildId::new(config.bot.get_staff_guild_id());
    let guild_channel = match staff_guild_id.create_channel(&ctx.http, channel_builder).await {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Failed to create channel: {}", e);
            send_error_message(ctx, msg, config, "new_thread.channel_creation_failed", None).await;
            return Ok(());
        }
    };

    let _ = match create_thread_for_user(&guild_channel, user_id.get() as i64, &user.name, pool).await {
        Ok(thread_id) => thread_id,
        Err(e) => {
            eprintln!("Failed to create thread in database: {}", e);
            let _ = guild_channel.delete(&ctx.http).await;
            send_error_message(ctx, msg, config, "new_thread.database_error", None).await;
            return Ok(());
        }
    };

    send_welcome_message(ctx, &guild_channel, config, &user).await;

    match send_dm_to_user(ctx, &user, config, &guild_channel).await {
        Ok(_) => {
            send_success_message(ctx, msg, config, &user, &guild_channel, true).await;
        }
        Err(dm_error) => {
            eprintln!("Failed to send DM to user: {}", dm_error);
            send_success_message(ctx, msg, config, &user, &guild_channel, false).await;
        }
    }

    Ok(())
}

async fn extract_user_id(msg: &Message, config: &Config) -> Option<UserId> {
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
            let id_str = &args[2..args.len()-1];
            let clean_id = id_str.trim_start_matches('!');
            if let Ok(user_id) = clean_id.parse::<u64>() {
                return Some(UserId::new(user_id));
            }
        }
    }
    
    None
}

async fn send_welcome_message(ctx: &Context, channel: &serenity::model::channel::GuildChannel, config: &Config, user: &serenity::model::user::User) {
    let mut params = HashMap::new();
    params.insert("user".to_string(), user.name.clone());
    
    let welcome_msg = get_translated_message(
        config,
        "new_thread.welcome_message",
        Some(&params),
        None,
        Some(channel.guild_id.get()),
        None,
    )
    .await;

    let _ = MessageBuilder::system_message(&ctx, config)
        .content(welcome_msg)
        .to_channel(channel.id)
        .send()
        .await;
}

async fn send_dm_to_user(
    ctx: &Context, 
    user: &serenity::model::user::User, 
    config: &Config, 
    _channel: &serenity::model::channel::GuildChannel
) -> Result<(), serenity::Error> {
    let dm_msg = get_translated_message(
        config,
        "new_thread.dm_notification",
        None,
        Some(user.id),
        None,
        None,
    )
    .await;

    let _ = MessageBuilder::system_message(&ctx, config)
        .content(dm_msg)
        .to_user(user.id)
        .send()
        .await;

    Ok(())
}

async fn send_error_message(
    ctx: &Context, 
    msg: &Message, 
    config: &Config, 
    error_key: &str, 
    params: Option<&HashMap<String, String>>
) {
    let error_msg = get_translated_message(
        config,
        error_key,
        params,
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let _ = MessageBuilder::system_message(&ctx, config)
        .content(error_msg)
        .to_channel(msg.channel_id)
        .send()
        .await;
}

async fn send_success_message(
    ctx: &Context, 
    msg: &Message, 
    config: &Config, 
    user: &serenity::model::user::User, 
    channel: &serenity::model::channel::GuildChannel,
    dm_sent: bool
) {
    let mut params = HashMap::new();
    params.insert("user".to_string(), user.name.clone());
    params.insert("channel_id".to_string(), channel.id.to_string());
    params.insert("staff".to_string(), msg.author.name.clone());
    
    let success_key = if dm_sent {
        "new_thread.success_with_dm"
    } else {
        "new_thread.success_without_dm"
    };
    
    let confirmation_msg = get_translated_message(
        config,
        success_key,
        Some(&params),
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let _ = MessageBuilder::system_message(&ctx, config)
        .content(confirmation_msg)
        .to_channel(msg.channel_id)
        .send()
        .await;
} 