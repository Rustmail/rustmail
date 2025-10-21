use crate::commands::alert::common::{
    extract_alert_action, get_thread_user_id_from_msg, handle_cancel_alert_from_msg,
    handle_set_alert_from_msg,
};
use crate::config::Config;
use crate::errors::{common, ModmailResult};
use crate::types::logs::PaginationStore;
use serenity::all::{Context, Message};
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub async fn alert(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    _shutdown: Arc<Receiver<bool>>,
    _pagination: PaginationStore,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let user_id = get_thread_user_id_from_msg(ctx, msg, config, pool).await?;
    let is_cancel = extract_alert_action(msg, config).await;

    if is_cancel {
        handle_cancel_alert_from_msg(ctx, msg, config, user_id, pool).await
    } else {
        handle_set_alert_from_msg(ctx, msg, config, user_id, pool).await
    }
}
