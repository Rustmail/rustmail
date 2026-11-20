use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{ChannelId, Context, GuildId, UserId};
use std::collections::HashMap;

pub fn get_user_recap(
    user_id: UserId,
    username: &str,
    member_join_date: &str,
    logs_info: &str,
) -> String {
    format!(
        "ACCOUNT AGE **{}**, ID **{}**\nNICKNAME **{}**, JOINED **{}** ago\n\n{}",
        format_duration_since(user_id.created_at()),
        user_id,
        username,
        member_join_date,
        logs_info
    )
}

pub async fn post_ticket_recap(
    ctx: &Context,
    config: &Config,
    channel_id: ChannelId,
    user_id: UserId,
    username: &str,
) {
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let member_join_date = get_member_join_date_for_user(ctx, user_id, community_guild_id)
        .await
        .unwrap_or_else(|| "Unknown".to_string());

    let logs_count = match &config.db_pool {
        Some(pool) => get_logs_from_user_id(&user_id.to_string(), pool)
            .await
            .map(|logs| logs.len())
            .unwrap_or(0),
        None => 0,
    };
    let params = HashMap::from([
        ("logs_count".to_string(), logs_count.to_string()),
        ("prefix".to_string(), config.command.prefix.clone()),
    ]);
    let logs_info = get_translated_message(
        config,
        "new_thread.show_logs",
        Some(&params),
        None,
        None,
        None,
    )
    .await;

    let _ = MessageBuilder::system_message(ctx, config)
        .content(get_user_recap(
            user_id,
            username,
            &member_join_date,
            &logs_info,
        ))
        .to_channel(channel_id)
        .send(true)
        .await;
}
