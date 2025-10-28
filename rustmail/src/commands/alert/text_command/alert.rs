use crate::commands::alert::common::{
    extract_alert_action, get_thread_user_id_from_msg, handle_cancel_alert_from_msg,
    handle_set_alert_from_msg,
};
use crate::config::Config;
use crate::errors::{ModmailResult, common};
use crate::handlers::guild_messages_handler::GuildMessagesHandler;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn alert(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let user_id = get_thread_user_id_from_msg(&ctx, &msg, config, pool).await?;
    let is_cancel = extract_alert_action(&msg, config).await;

    if is_cancel {
        handle_cancel_alert_from_msg(&ctx, &msg, config, user_id, pool).await
    } else {
        handle_set_alert_from_msg(&ctx, &msg, config, user_id, pool).await
    }
}
