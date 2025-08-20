use serenity::all::{Context, GuildId, Message, UserId};
use std::collections::HashMap;
use crate::{
    config::Config,
    db::close_thread,
    errors::{common, ModmailResult},
};
use crate::utils::message::message_builder::MessageBuilder;
use crate::utils::thread::fetch_thread::fetch_thread;

pub async fn close(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;
    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

    if user_still_member {
        let _ = MessageBuilder::system_message(ctx, config)
            .content(&config.bot.close_message)
            .to_user(user_id)
            .send()
            .await;
    } else {
        let mut params = HashMap::new();
        params.insert("username".to_string(), thread.user_name.clone());

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "user.left_server_close",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get())
            ).await
            .to_channel(msg.channel_id)
            .send()
            .await;
    }

    close_thread(&thread.id, db_pool).await?;

    let _ = msg.channel_id.delete(&ctx.http).await?;

    Ok(())
}
