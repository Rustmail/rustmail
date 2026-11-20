use crate::prelude::config::*;
use crate::prelude::utils::*;
use serenity::all::{Context, GuildChannel, User};
use std::collections::HashMap;

pub async fn send_ticket_welcome(
    ctx: &Context,
    config: &Config,
    channel: &GuildChannel,
    user: &User,
    notify_user: bool,
) {
    let params = HashMap::from([("user".to_string(), user.name.clone())]);

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

    if notify_user {
        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "new_thread.welcome_message",
                Some(&params),
                Some(user.id),
                Some(channel.guild_id.get()),
            )
            .await
            .to_user(user.id)
            .send(true)
            .await;
    }
}
