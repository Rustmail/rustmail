use serenity::all::{GuildId, UserId};
use serenity::http::Http;
use std::sync::Arc;

pub async fn is_member(http: Arc<Http>, guild_id: u64, user_id: u64) -> bool {
    let guild = GuildId::new(guild_id);
    let user = UserId::new(user_id);

    guild.member(&http, user).await.is_ok()
}
