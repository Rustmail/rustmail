use crate::config::Config;
use crate::db::get_thread_by_channel_id;
use crate::db::threads::is_a_ticket_channel;
use crate::errors::common::{database_connection_failed, thread_not_found};
use crate::errors::ThreadError::NotAThreadChannel;
use crate::errors::{CommandError, ModmailError, ModmailResult};
use crate::handlers::guild_messages_handler::GuildMessagesHandler;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{ChannelId, Context, Message};
use serenity::builder::EditChannel;
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

        let _ = thread_id
            .edit(
                &ctx.http,
                EditChannel::new().name(format!("🔵-{}", msg.author.name)),
            )
            .await;

        let mut params = std::collections::HashMap::new();
        params.insert("staff".to_string(), format!("<@{}>", msg.author.id));

        let _ = MessageBuilder::system_message(&ctx, config)
            .translated_content("take.confirmation", Some(&params), None, None)
            .await
            .to_channel(msg.channel_id)
            .send(true)
            .await?;

        Ok(())
    } else {
        Err(ModmailError::Thread(NotAThreadChannel))
    }
}
