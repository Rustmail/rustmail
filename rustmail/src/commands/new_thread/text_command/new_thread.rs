use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{ChannelId, Context, GuildId, Message};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn new_thread(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let user_id = extract_user_id(&msg, config).await;
    if user_id.is_none() {
        send_error_message(&ctx, &msg, config, "new_thread.missing_user", None).await;
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
                &ctx,
                &msg,
                config,
                "new_thread.user_has_thread_with_link",
                Some(&params),
            )
            .await;
        } else {
            send_error_message(&ctx, &msg, config, "new_thread.user_has_thread", None).await;
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
            send_error_message(
                &ctx,
                &msg,
                config,
                "new_thread.channel_creation_failed",
                None,
            )
            .await;
            return Ok(());
        }
    };

    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());

    let member_join_date = get_member_join_date_for_user(&ctx, user_id, community_guild_id)
        .await
        .unwrap_or_else(|| "Unknown".to_string());

    let logs_count = match get_logs_from_user_id(&user_id.clone().to_string(), pool).await {
        Ok(logs) => logs.len(),
        Err(_) => 0,
    };

    let params = {
        let mut p = HashMap::new();
        p.insert("logs_count".to_string(), logs_count.to_string());
        p.insert("prefix".to_string(), config.command.prefix.clone());
        p
    };

    let logs_info = get_translated_message(
        &config,
        "new_thread.show_logs",
        Some(&params),
        None,
        None,
        None,
    )
    .await;

    let recap = get_user_recap(user_id, &user.name, &member_join_date, &logs_info);

    let _ = MessageBuilder::system_message(&ctx, &config)
        .content(recap)
        .to_channel(guild_channel.id)
        .send(true)
        .await;

    let _ = match create_thread_for_user(&guild_channel, user_id.get() as i64, &user.name, pool)
        .await
    {
        Ok(thread_id) => thread_id,
        Err(e) => {
            eprintln!("Failed to create thread in database: {}", e);
            let _ = guild_channel.delete(&ctx.http).await;
            send_error_message(&ctx, &msg, config, "new_thread.database_error", None).await;
            return Ok(());
        }
    };

    send_welcome_message(&ctx, &guild_channel, config, &user).await;

    match send_dm_to_user(&ctx, &user, config).await {
        Ok(_) => {
            send_success_message(&ctx, &msg, config, &user, &guild_channel, true).await;
        }
        Err(dm_error) => {
            eprintln!("Failed to send DM to user: {}", dm_error);
            send_success_message(&ctx, &msg, config, &user, &guild_channel, false).await;
        }
    }

    Ok(())
}
