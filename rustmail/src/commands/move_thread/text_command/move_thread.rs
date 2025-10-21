use crate::commands::move_thread::common::{
    extract_category_name, fetch_server_categories, find_best_match_category, is_in_thread,
    move_channel_to_category_by_msg, send_success_message,
};
use crate::config::Config;
use crate::errors::CommandError::NotInThread;
use crate::errors::ThreadError::CategoryNotFound;
use crate::errors::{common, CommandError, DiscordError, ModmailError, ModmailResult};
use crate::types::logs::PaginationStore;
use serenity::all::{Context, Message};
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub async fn move_thread(
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

    if !is_in_thread(msg, pool).await {
        return Err(ModmailError::Command(NotInThread()));
    }

    let category_name = extract_category_name(msg, config).await;
    if category_name.is_empty() {
        return Err(ModmailError::Thread(CategoryNotFound));
    }

    let categories = fetch_server_categories(ctx, config).await;
    if categories.is_empty() {
        return Err(ModmailError::Discord(DiscordError::FailedToFetchCategories));
    }

    let target_category = find_best_match_category(&category_name, &categories);

    match target_category {
        Some((category_id, category_name)) => {
            if let Err(e) = move_channel_to_category_by_msg(ctx, msg, category_id).await {
                return Err(ModmailError::Command(CommandError::CommandFailed(
                    e.to_string(),
                )));
            }

            send_success_message(ctx, msg, config, &category_name).await;
        }
        None => {
            return Err(ModmailError::Thread(CategoryNotFound));
        }
    }

    Ok(())
}
