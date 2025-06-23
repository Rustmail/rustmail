use serenity::all::{Context, CreateMessage, GuildId, Message, UserId};
use std::collections::HashMap;

use crate::{
    config::Config,
    db::close_thread,
    errors::{ModmailResult, common},
    i18n::get_translated_message,
    utils::{
        build_message_from_ticket::build_message_from_ticket,
        fetch_thread::fetch_thread,
        format_ticket_message::{Sender, format_ticket_message},
    },
};

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
        let dm_channel = match user_id.create_dm_channel(&ctx.http).await {
            Ok(channel) => channel,
            Err(_) => {
                let err_msg = get_translated_message(
                    config,
                    "close.user_not_found",
                    None,
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                    None,
                )
                .await;
                return Err(common::user_not_found());
            }
        };

        let close_message = format_ticket_message(
            &ctx,
            Sender::System {
                user_id: ctx.cache.current_user().id,
                username: ctx.cache.current_user().name.clone(),
            },
            &config.bot.close_message,
            config,
        );
        let close_message = close_message.await;
        let dm_message = build_message_from_ticket(close_message, CreateMessage::new());
        let _ = dm_channel.send_message(&ctx.http, dm_message).await;
    } else {
        let mut params = HashMap::new();
        params.insert("username".to_string(), thread.user_name.clone());

        let info_message = get_translated_message(
            config,
            "user.left_server_close",
            Some(&params),
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
            None,
        )
        .await;

        let info_response = format_ticket_message(
            &ctx,
            Sender::System {
                user_id: ctx.cache.current_user().id,
                username: ctx.cache.current_user().name.clone(),
            },
            &info_message,
            config,
        );
        let info_response = info_response.await;
        let thread_message = build_message_from_ticket(info_response, CreateMessage::new());
        let _ = msg.channel_id.send_message(&ctx.http, thread_message).await;
    }

    close_thread(&thread.id, db_pool).await?;

    let _ = msg.channel_id.delete(&ctx.http).await?;

    Ok(())
}
