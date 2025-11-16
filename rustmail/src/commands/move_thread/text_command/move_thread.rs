use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn move_thread(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    if !is_in_thread(&msg, pool).await {
        return Err(ModmailError::Command(CommandError::NotInThread()));
    }

    let category_name = extract_category_name(&msg, config).await;
    if category_name.is_empty() {
        return Err(ModmailError::Thread(ThreadError::CategoryNotFound));
    }

    let categories = fetch_server_categories(&ctx, config).await;
    if categories.is_empty() {
        return Err(ModmailError::Discord(DiscordError::FailedToFetchCategories));
    }

    let target_category = find_best_match_category(&category_name, &categories);

    match target_category {
        Some((category_id, category_name)) => {
            if let Err(e) = move_channel_to_category_by_msg(&ctx, &msg, category_id).await {
                return Err(ModmailError::Command(CommandError::CommandFailed(
                    e.to_string(),
                )));
            }

            let mut params = HashMap::new();
            params.insert("category".to_string(), category_name.to_string());
            params.insert("staff".to_string(), msg.author.name.clone());

            let confirmation_msg = get_translated_message(
                config,
                "move_thread.success",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;

            let _ = MessageBuilder::system_message(&ctx.clone(), config)
                .content(confirmation_msg)
                .to_channel(msg.channel_id)
                .send(true)
                .await;

            let _ = msg.delete(&ctx.http).await;
        }
        None => {
            return Err(ModmailError::Thread(ThreadError::CategoryNotFound));
        }
    }

    Ok(())
}
