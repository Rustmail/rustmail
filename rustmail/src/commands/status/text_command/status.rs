use crate::config::Config;
use crate::errors::{ModmailResult, database_connection_failed};
use crate::handlers::GuildMessagesHandler;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn status_command(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    Ok(())
}
