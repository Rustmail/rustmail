use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{ChannelId, Context, Message};
use std::sync::Arc;

pub async fn release(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    if is_a_ticket_channel(msg.channel_id, &db_pool).await {
        let thread = match get_thread_by_channel_id(&msg.channel_id.to_string(), db_pool).await {
            Some(thread) => thread,
            None => return Err(thread_not_found()),
        };

        let parse_thread_id = thread.channel_id.parse::<u64>().unwrap_or(0);

        let thread_id = ChannelId::new(parse_thread_id);

        let thread_name = thread_id
            .name(&ctx)
            .await
            .unwrap_or_else(|_| "Unknown".to_string());

        if thread_name == thread.user_name {
            return Err(ModmailError::Command(CommandError::TicketAlreadyReleased));
        }

        rename_channel_with_timeout(
            &ctx,
            &config,
            thread_id,
            thread.user_name.clone(),
            Some(&msg),
            None,
        )
        .await?;

        let mut params = std::collections::HashMap::new();
        params.insert("staff".to_string(), format!("<@{}>", msg.author.id));

        let _ = MessageBuilder::system_message(&ctx, config)
            .translated_content("release.confirmation", Some(&params), None, None)
            .await
            .to_channel(msg.channel_id)
            .send(true)
            .await?;

        Ok(())
    } else {
        Err(ModmailError::Thread(ThreadError::NotAThreadChannel))
    }
}
