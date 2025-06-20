use serenity::all::{Context, CreateMessage, Message, UserId};

use crate::{
    config::Config,
    db::close_thread,
    errors::{ModmailResult, common},
    utils::{
        build_message_from_ticket::build_message_from_ticket,
        fetch_thread::fetch_thread,
        format_ticket_message::{Sender, format_ticket_message}
    },
    i18n::get_translated_message,
};

pub async fn close(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;
    let user_id = UserId::new(thread.user_id as u64);

    let close_thread_message = get_translated_message(
        config,
        "thread.closed",
        None,
        Some(user_id),
        msg.guild_id.map(|g| g.get()),
        None
    ).await;
    let response = format_ticket_message(
        &ctx,
        Sender::System {
            user_id: ctx.cache.current_user().id,
            username: ctx.cache.current_user().name.clone(),
        },
        &close_thread_message,
        config,
    );
    let response = response.await;
    let thread_message = build_message_from_ticket(response, CreateMessage::new());

    let _ = user_id.direct_message(&ctx.http, thread_message).await?;

    close_thread(&thread.id, db_pool).await?;

    let _ = msg.channel_id.delete(&ctx.http).await?;

    Ok(())
}
