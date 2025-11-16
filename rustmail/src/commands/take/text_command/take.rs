use crate::modules::update_thread_status_ui;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{ChannelId, Context, Message};
use std::sync::Arc;

pub async fn take(
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

        if thread_name == format!("🔵-{}", msg.author.name) {
            return Err(ModmailError::Command(CommandError::TicketAlreadyTaken));
        }

        let config_clone = config.clone();

        tokio::spawn({
            let db_pool = db_pool.clone();
            async move {
                let mut ticket_status = match get_thread_status(&thread.id, &db_pool).await {
                    Some(status) => status,
                    None => {
                        return;
                    }
                };
                ticket_status.taken_by = Some(msg.author.id.to_string());
                let _ = update_thread_status_db(&thread.id, &ticket_status, &db_pool).await;

                tokio::spawn({
                    let ctx = ctx.clone();
                    async move {
                        let _ = update_thread_status_ui(&ctx, &ticket_status).await;
                    }
                });

                let mut params = std::collections::HashMap::new();
                params.insert("staff".to_string(), format!("<@{}>", msg.author.id));

                let _ = MessageBuilder::system_message(&ctx, &config_clone)
                    .translated_content("take.confirmation", Some(&params), None, None)
                    .await
                    .to_channel(msg.channel_id)
                    .send(true)
                    .await;
            }
        });

        Ok(())
    } else {
        Err(ModmailError::Thread(ThreadError::NotAThreadChannel))
    }
}
