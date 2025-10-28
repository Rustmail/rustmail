use crate::commands::remove_staff::common::{extract_user_id, remove_user_from_channel};
use crate::config::Config;
use crate::db::thread_exists;
use crate::errors::CommandError::InvalidFormat;
use crate::errors::ThreadError::NotAThreadChannel;
use crate::errors::{ModmailError, ModmailResult, common};
use crate::handlers::guild_messages_handler::GuildMessagesHandler;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, Message, UserId};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn remove_staff(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let user_id_str = extract_user_id(&msg, config).await;

    if user_id_str.is_empty() {
        return Err(ModmailError::Command(InvalidFormat));
    }

    let user_id = match user_id_str.parse::<u64>() {
        Ok(id) => UserId::new(id),
        Err(_) => return Err(ModmailError::Command(InvalidFormat)),
    };

    if thread_exists(msg.author.id, pool).await {
        match remove_user_from_channel(&ctx, msg.channel_id, user_id).await {
            Ok(_) => {
                let mut params = HashMap::new();
                params.insert("user".to_string(), format!("<@{}>", user_id));

                let _ = MessageBuilder::system_message(&ctx, config)
                    .translated_content("add_staff.remove_success", Some(&params), None, None)
                    .await
                    .to_channel(msg.channel_id)
                    .send(true)
                    .await;

                Ok(())
            }
            Err(..) => Err(ModmailError::Command(InvalidFormat)),
        }
    } else {
        Err(ModmailError::Thread(NotAThreadChannel))
    }
}
