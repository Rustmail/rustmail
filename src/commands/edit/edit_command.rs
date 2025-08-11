use crate::db::update_message_content;
use crate::errors::{ModmailResult, common};
use crate::{config::Config, utils::extract_reply_content::extract_reply_content};
use serenity::all::{Context, Message};

use crate::commands::edit::message_ops::{
    cleanup_command_message, edit_messages, format_new_message, get_message_ids,
};
use crate::commands::edit::validation::{parse_edit_command, validate_edit_permissions};

pub async fn edit(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let error_handler = config
        .error_handler
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let raw_content = extract_command_content(msg, config)?;

    let command_input = parse_edit_command(&raw_content)?;

    validate_edit_permissions(command_input.message_number, msg.author.id, pool).await?;

    let (dm_msg_id, inbox_msg_id) = get_message_ids(
        command_input.message_number,
        msg.author.id,
        pool,
        config,
        ctx,
        msg,
    )
    .await
    .map_err(|_| common::message_number_not_found(command_input.message_number))?;

    let (thread_message, dm_message) = format_new_message(
        ctx,
        &msg.author.name,
        msg.author.id,
        &command_input.new_content,
        Some(command_input.message_number as u64),
        config,
    )
    .await;

    let edit_result = edit_messages(
        ctx,
        msg.channel_id,
        dm_msg_id.clone(),
        inbox_msg_id,
        &thread_message,
        &dm_message,
        pool,
        config,
        msg,
    )
    .await;

    match edit_result {
        crate::commands::edit::message_ops::EditResult::Success => {
            if config.notifications.show_success_on_edit {
                let _ = error_handler
                    .send_success_message(
                        ctx,
                        msg.channel_id,
                        "success.message_edited",
                        None,
                        Some(msg.author.id),
                        msg.guild_id.map(|g| g.get()),
                    )
                    .await;
            }

            cleanup_command_message(ctx, msg).await;
            match dm_msg_id {
                Some(dm_msg_id) => {
                    let _ =
                        update_message_content(&dm_msg_id, &command_input.new_content, pool).await;
                }
                None => {}
            }
            Ok(())
        }
        crate::commands::edit::message_ops::EditResult::PartialSuccess(warning) => {
            if config.notifications.show_partial_success_on_edit {
                let _ = msg.reply(ctx, warning).await;
            }
            Ok(())
        }
        crate::commands::edit::message_ops::EditResult::Failure(error_msg) => {
            if config.notifications.show_failure_on_edit {
                let _ = msg.reply(ctx, error_msg).await;
            }
            Err(common::validation_failed("Edit operation failed"))
        }
    }
}

fn extract_command_content(msg: &Message, config: &Config) -> ModmailResult<String> {
    extract_reply_content(&msg.content, &config.command.prefix, &["edit", "e"])
        .ok_or_else(|| common::invalid_command())
}

#[cfg(test)]
mod tests {
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
                logs_channel_id: 64456,
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
            },
            notifications: crate::config::NotificationsConfig::default(),
            language: crate::config::LanguageConfig::default(),
            error_handling: crate::config::ErrorHandlingConfig::default(),
            db_pool: None,
            error_handler: None,
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
