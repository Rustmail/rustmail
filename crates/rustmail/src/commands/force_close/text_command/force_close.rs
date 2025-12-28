use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn force_close(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    if !is_a_ticket_channel(msg.channel_id, db_pool).await {
        return match msg.category_id(&ctx.http).await {
            Some(category_id) if category_id == config.thread.inbox_category_id => {
                delete_channel(&ctx, msg.channel_id).await
            }
            _ => Err(ModmailError::Thread(ThreadError::NotAThreadChannel)),
        };
    }

    let thread = get_thread_by_channel_id(&msg.channel_id.to_string(), db_pool).await;

    match is_orphaned_thread_channel(msg.channel_id, db_pool).await {
        Ok(res) => {
            if !res {
                return Err(ModmailError::Thread(ThreadError::UserStillInServer));
            }

            if let Some(thread_info) = thread {
                if config.bot.enable_logs {
                    if let Some(logs_channel_id) = config.bot.logs_channel_id {
                        let base_url = config
                            .bot
                            .redirect_url
                            .trim_end_matches("/api/auth/callback")
                            .trim_end_matches('/');

                        let panel_url = format!("{}/panel/tickets/{}", base_url, thread_info.id);

                        let mut params = std::collections::HashMap::new();
                        params.insert("username".to_string(), thread_info.user_name.clone());
                        params.insert("user_id".to_string(), thread_info.user_id.to_string());
                        params.insert("panel_url".to_string(), panel_url);

                        let _ = MessageBuilder::system_message(&ctx, config)
                            .translated_content(
                                "logs.ticket_closed",
                                Some(&params),
                                Some(msg.author.id),
                                msg.guild_id.map(|g| g.get()),
                            )
                            .await
                            .to_channel(serenity::all::ChannelId::new(logs_channel_id))
                            .send(true)
                            .await;
                    }
                }
            }

            delete_channel(&ctx, msg.channel_id).await
        }
        Err(..) => Err(ModmailError::Database(DatabaseError::QueryFailed(
            "Failed to check if thread channel is orphaned".to_string(),
        ))),
    }
}
