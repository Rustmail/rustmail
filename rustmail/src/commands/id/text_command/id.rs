use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn id(
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

        let mut params = std::collections::HashMap::new();
        params.insert("user".to_string(), format!("<@{}>", thread.user_id));
        params.insert(
            "id".to_string(),
            format!("||{}||", thread.user_id.to_string()),
        );

        let _ = MessageBuilder::system_message(&ctx, config)
            .translated_content("id.show_id", Some(&params), None, None)
            .await
            .to_channel(msg.channel_id)
            .send(true)
            .await?;

        Ok(())
    } else {
        Err(ModmailError::Thread(ThreadError::NotAThreadChannel))
    }
}
