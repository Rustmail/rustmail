use crate::commands::new_thread::common::{
    extract_user_id, send_dm_to_user, send_error_message, send_success_message,
    send_welcome_message,
};
use crate::config::Config;
use crate::db::{create_thread_for_user, get_thread_channel_by_user_id, thread_exists};
use crate::errors::{common, DiscordError, ModmailError, ModmailResult};
use serenity::all::{ChannelId, Context, GuildId, Message};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub async fn new_thread(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    _shutdown: Arc<Receiver<bool>>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let user_id = extract_user_id(msg, config).await;
    if user_id.is_none() {
        send_error_message(ctx, msg, config, "new_thread.missing_user", None).await;
        return Ok(());
    }

    let user_id = user_id.unwrap();

    let user = match ctx.http.get_user(user_id).await {
        Ok(user) => user,
        Err(_) => return Err(ModmailError::Discord(DiscordError::UserNotFound)),
    };

    if user.bot {
        return Err(ModmailError::Discord(DiscordError::UserIsABot));
    }

    if thread_exists(user_id, pool).await {
        if let Some(channel_id_str) = get_thread_channel_by_user_id(user_id, pool).await {
            let mut params = HashMap::new();
            params.insert("user".to_string(), user.name.clone());
            params.insert("channel_id".to_string(), channel_id_str.clone());

            send_error_message(
                ctx,
                msg,
                config,
                "new_thread.user_has_thread_with_link",
                Some(&params),
            )
            .await;
        } else {
            send_error_message(ctx, msg, config, "new_thread.user_has_thread", None).await;
        }
        return Ok(());
    }

    let inbox_category_id = ChannelId::new(config.thread.inbox_category_id);
    let channel_name = user.name.to_lowercase().replace(" ", "-").to_string();
    let mut channel_builder = serenity::all::CreateChannel::new(&channel_name);
    channel_builder = channel_builder
        .kind(serenity::model::channel::ChannelType::Text)
        .category(inbox_category_id);

    let staff_guild_id = GuildId::new(config.bot.get_staff_guild_id());
    let guild_channel = match staff_guild_id
        .create_channel(&ctx.http, channel_builder)
        .await
    {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Failed to create channel: {}", e);
            send_error_message(ctx, msg, config, "new_thread.channel_creation_failed", None).await;
            return Ok(());
        }
    };

    let _ = match create_thread_for_user(&guild_channel, user_id.get() as i64, &user.name, pool)
        .await
    {
        Ok(thread_id) => thread_id,
        Err(e) => {
            eprintln!("Failed to create thread in database: {}", e);
            let _ = guild_channel.delete(&ctx.http).await;
            send_error_message(ctx, msg, config, "new_thread.database_error", None).await;
            return Ok(());
        }
    };

    send_welcome_message(ctx, &guild_channel, config, &user).await;

    match send_dm_to_user(ctx, &user, config).await {
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
