use crate::config::Config;
use crate::errors::ModmailResult;
use crate::errors::common::invalid_command;
use crate::utils::command::extract_reply_content::extract_reply_content;
use serenity::all::Message;

pub fn extract_command_content(msg: &Message, config: &Config) -> ModmailResult<String> {
    extract_reply_content(&msg.content, &config.command.prefix, &["edit", "e"])
        .ok_or_else(invalid_command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        BotConfig, CommandConfig, Config, ErrorHandlingConfig, LanguageConfig, LogsConfig,
        NotificationsConfig, ReminderConfig, ThreadConfig,
    };
    use std::sync::{Arc, Mutex};

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
                enable_panel: false,
                features_channel_id: Some(12345),
                logs_channel_id: Some(15456),
                client_id: 123456789012345678,
                client_secret: "secret".to_string(),
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
                create_ticket_by_create_channel: false,
            },
            notifications: NotificationsConfig::default(),
            logs: LogsConfig::default(),
            language: LanguageConfig::default(),
            reminders: ReminderConfig::default(),
            error_handling: ErrorHandlingConfig::default(),
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
