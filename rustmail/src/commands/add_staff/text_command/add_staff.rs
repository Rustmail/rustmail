use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message, UserId};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn add_staff(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let user_id_str = extract_staff_id(&msg, config).await;

    if user_id_str.is_empty() {
        return Err(ModmailError::Command(CommandError::InvalidFormat));
    }

    let user_id = match user_id_str.parse::<u64>() {
        Ok(id) => UserId::new(id),
        Err(_) => return Err(ModmailError::Command(CommandError::InvalidFormat)),
    };

    if thread_exists(msg.author.id, pool).await {
        match add_user_to_channel(&ctx, msg.channel_id, user_id).await {
            Ok(_) => {
                let mut params = HashMap::new();
                params.insert("user".to_string(), format!("<@{}>", user_id));

                let _ = MessageBuilder::system_message(&ctx, config)
                    .translated_content("add_staff.add_success", Some(&params), None, None)
                    .await
                    .to_channel(msg.channel_id)
                    .send(true)
                    .await;

                Ok(())
            }
            Err(..) => Err(ModmailError::Command(CommandError::InvalidFormat)),
        }
    } else {
        Err(ModmailError::Thread(ThreadError::NotAThreadChannel))
    }
}
