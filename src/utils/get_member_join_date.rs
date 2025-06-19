use serenity::all::{Context, GuildId, Message};

use crate::utils::format_duration_since::format_duration_since;

pub async fn get_member_join_date(
    ctx: &Context,
    msg: &Message,
    guild_id: GuildId,
) -> Option<String> {
    if let Ok(member) = ctx.http.get_member(guild_id, msg.author.id).await {
        if let Some(timestamp) = member.joined_at {
            Some(format_duration_since(timestamp))
        } else {
            None
        }
    } else {
        None
    }
}
