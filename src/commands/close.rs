use serenity::all::{Context, Message};

use crate::{
    config::Config,
    db::close_thread,
    errors::{ModmailResult, common},
    utils::fetch_thread::fetch_thread,
};

pub async fn close(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;

    close_thread(&thread.id, db_pool).await?;

    let _ = msg.channel_id.delete(&ctx.http).await?;

    Ok(())
}
