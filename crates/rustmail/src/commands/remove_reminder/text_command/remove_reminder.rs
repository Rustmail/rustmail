use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn remove_reminder(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let content =
        match extract_reply_content(&msg.content, &config.command.prefix, &["unremind", "urem"]) {
            Some(c) => c,
            None => {
                return Err(ModmailError::Command(CommandError::InvalidArguments(
                    "".to_string(),
                )));
            }
        };

    let reminder_id = match content.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            return Err(ModmailError::Command(CommandError::InvalidArguments(
                "Reminder ID".to_string(),
            )));
        }
    };

    let reminder = match get_reminder_by_id(reminder_id as i64, pool).await {
        Ok(Some(r)) => {
            if r.completed {
                return Err(ModmailError::Command(
                    CommandError::ReminderAlreadyCompleted(reminder_id.to_string()),
                ));
            } else {
                r
            }
        }
        Ok(None) => {
            return Err(ModmailError::Database(DatabaseError::NotFound(
                "".to_string(),
            )));
        }
        Err(e) => {
            return Err(ModmailError::Database(DatabaseError::QueryFailed(
                e.to_string(),
            )));
        }
    };

    match update_reminder_status(&reminder, true, pool).await {
        Ok(_) => {
            let mut params = HashMap::new();
            params.insert("id".to_string(), reminder_id.to_string());

            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content("remove_reminder.confirmation", Some(&params), None, None)
                .await
                .to_channel(msg.channel_id)
                .send(true)
                .await;
        }
        Err(e) => {
            return Err(ModmailError::Database(DatabaseError::QueryFailed(
                e.to_string(),
            )));
        }
    }

    Ok(())
}
