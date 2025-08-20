use crate::db::update_message_content;
use crate::errors::{common, ModmailResult};
use crate::config::Config;
use serenity::all::{Context, Message};
use crate::commands::edit::message_ops::{cleanup_command_message, edit_messages, format_new_message, get_message_ids};
use crate::commands::edit::validation::{parse_edit_command, validate_edit_permissions, EditCommandInput};
use crate::errors::common::{invalid_command, message_not_found};
use crate::utils::command::extract_reply_content::extract_reply_content;
use crate::utils::conversion::hex_string_to_int::hex_string_to_int;
use crate::utils::message::message_builder::MessageBuilder;

pub async fn edit(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let raw_content: String = match extract_command_content(msg, config) {
        Ok(content) => content,
        Err(e) => return Err(e)
    };

    let command_input: EditCommandInput = match parse_edit_command(&raw_content) {
        Ok(command_input) => command_input,
        Err(e) => return Err(e)
    };

    match validate_edit_permissions(
        command_input.message_number,
        msg.channel_id,
        msg.author.id,
        pool
    ).await {
        Ok(()) => (),
        Err(e) => return Err(e)
    };

    let ids = match get_message_ids(
        command_input.message_number,
        msg.author.id,
        pool,
        ctx,
        msg
    ).await {
        Ok(ids) => ids,
        Err(e) => return Err(e)
    };

    let dm_msg_id = match ids.dm_message_id {
        Some(msg_id) => msg_id,
        None => return Err(message_not_found("Inbox message ID not found")),
    };

    let inbox_message_id = match ids.inbox_message_id {
        Some(msg_id) => msg_id,
        None => return Err(message_not_found("DM message ID not found")),
    };

    let edited_messages_builder = match format_new_message(
        ctx,
        &msg,
        &command_input.new_content,
        &inbox_message_id,
        command_input.message_number as u64,
        config,
        pool
    ).await {
        Ok(edited_messages) => edited_messages,
        Err(e) => return Err(e)
    };

    let edit_result = edit_messages(
        ctx,
        msg.channel_id,
        dm_msg_id.clone(),
        inbox_message_id.clone(),
        edited_messages_builder,
        pool,
        config
    ).await;

    match edit_result {
        Ok(()) => {
            if config.notifications.show_success_on_edit {
                let _ = MessageBuilder::system_message(ctx, config)
                    .translated_content(
                        "success.message_edited",
                        None,
                        Some(msg.author.id),
                        msg.guild_id.map(|g| g.get())
                    ).await
                    .color(hex_string_to_int(&config.thread.system_message_color) as u32)
                    .to_channel(msg.channel_id)
                    .send()
                    .await;
            };

            cleanup_command_message(ctx, msg).await;

            match update_message_content(
                &dm_msg_id,
                &command_input.new_content,
                pool
            ).await {
                Ok(()) => (),
                Err(e) => return Err(e)
            }

            Ok(())
        },
        Err(e) => Err(e)
    }
}

fn extract_command_content(msg: &Message, config: &Config) -> ModmailResult<String> {
    extract_reply_content(&msg.content, &config.command.prefix, &["edit", "e"])
        .ok_or_else(|| invalid_command())
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::*;
    use crate::config::{BotConfig, CommandConfig, Config, ThreadConfig};

    fn create_test_config() -> Config {
        Config {
            bot: BotConfig {
                token: "test".to_string(),
                mode: crate::config::ServerMode::Dual {
                    community_guild_id: 184848,
                    staff_guild_id: 64456,
                },
                status: "test".to_string(),
                welcome_message: "Welcome to the server!".to_string(),
                close_message: "Thank you for your message!".to_string(),
                typing_proxy_from_user: false,
                typing_proxy_from_staff: false,
                enable_logs: true,
                enable_features: true,
                features_channel_id: Some(12345),
                logs_channel_id: Some(15456),
            },
            command: CommandConfig {
                prefix: "!".to_string(),
            },
            thread: ThreadConfig {
                inbox_category_id: 12345,
                embedded_message: false,
                user_message_color: "ffffff".to_string(),
                staff_message_color: "ffffff".to_string(),
                system_message_color: "ffffff".to_string(),
                block_quote: false,
                time_to_close_thread: 5,
            },
            notifications: crate::config::NotificationsConfig::default(),
            language: crate::config::LanguageConfig::default(),
            error_handling: crate::config::ErrorHandlingConfig::default(),
            db_pool: None,
            error_handler: None,
            thread_locks: Arc::new(Mutex::new(Default::default())),
        }
    }

    #[test]
    fn test_extract_command_content_valid() {
        let config = create_test_config();
        let mut msg = Message::default();
        msg.content = "!edit 3 New message content".to_string();

        let result = extract_command_content(&msg, &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3 New message content");
    }

    #[test]
    fn test_extract_command_content_invalid() {
        let config = create_test_config();
        let mut msg = Message::default();
        msg.content = "!other command".to_string();

        let result = extract_command_content(&msg, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_command_content_alias() {
        let config = create_test_config();
        let mut msg = Message::default();
        msg.content = "!e 5 Updated content".to_string();

        let result = extract_command_content(&msg, &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5 Updated content");
    }
}
