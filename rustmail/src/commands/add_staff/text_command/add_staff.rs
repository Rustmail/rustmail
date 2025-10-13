use crate::commands::add_staff::common::add_user_to_channel;
use crate::commands::add_staff::common::extract_user_id;
use crate::config::Config;
use crate::db::thread_exists;
use crate::errors::CommandError::InvalidFormat;
use crate::errors::ThreadError::NotAThreadChannel;
use crate::errors::{ModmailError, ModmailResult, common};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, Message, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub async fn add_staff(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    _shutdown: Arc<Receiver<bool>>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let user_id_str = extract_user_id(msg, config).await;

    if user_id_str.is_empty() {
        return Err(ModmailError::Command(InvalidFormat));
    }

    let user_id = match user_id_str.parse::<u64>() {
        Ok(id) => UserId::new(id),
        Err(_) => return Err(ModmailError::Command(InvalidFormat)),
    };

    if thread_exists(msg.author.id, pool).await {
        match add_user_to_channel(ctx, msg.channel_id, user_id).await {
            Ok(_) => {
                let mut params = HashMap::new();
                params.insert("user".to_string(), format!("<@{}>", user_id));

                let _ = MessageBuilder::system_message(ctx, config)
                    .translated_content("add_staff.add_success", Some(&params), None, None)
                    .await
                    .to_channel(msg.channel_id)
                    .send()
                    .await;

                Ok(())
            }
            Err(..) => Err(ModmailError::Command(InvalidFormat)),
        }
    } else {
        Err(ModmailError::Thread(NotAThreadChannel))
    }
}
