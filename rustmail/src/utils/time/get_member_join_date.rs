use crate::prelude::utils::*;
use serenity::all::{Context, GuildId, Message, UserId};

pub async fn get_member_join_date(
    ctx: &Context,
    msg: &Message,
    guild_id: GuildId,
) -> Option<String> {
    if let Ok(member) = ctx.http.get_member(guild_id, msg.author.id).await {
        member.joined_at.map(format_duration_since)
    } else {
        None
    }
}

pub async fn get_member_join_date_for_user(
    ctx: &Context,
    user_id: UserId,
    guild_id: GuildId,
) -> Option<String> {
    if let Ok(member) = ctx.http.get_member(guild_id, user_id).await {
        member.joined_at.map(format_duration_since)
    } else {
        None
    }
}
