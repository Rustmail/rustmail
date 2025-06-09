#[cfg(test)]
mod tests {
    use crate::errors::{
        CommandError, DatabaseError, DictionaryManager, DiscordError, ErrorHandler, MessageError,
        ModmailError, PermissionError, ThreadError, ValidationError, common,
    };
    use crate::i18n::languages::Language;
    use serenity::all::UserId;
    use std::collections::HashMap;
    use toml;

    #[test]
    fn test_error_creation_macros() {
        let db_err = ModmailError::Database(DatabaseError::ConnectionFailed);
        match db_err {
            ModmailError::Database(DatabaseError::ConnectionFailed) => {}
            _ => panic!("Database error creation failed"),
        }

        let discord_err = ModmailError::Discord(DiscordError::ApiError("Test error".to_string()));
        match discord_err {
            ModmailError::Discord(DiscordError::ApiError(msg)) => {
                assert_eq!(msg, "Test error");
            }
            _ => panic!("Discord error creation failed"),
        }

        let cmd_err = ModmailError::Command(CommandError::InvalidFormat);
        match cmd_err {
            ModmailError::Command(CommandError::InvalidFormat) => {}
            _ => panic!("Command error creation failed"),
        }
    }

    #[test]
    fn test_common_error_functions() {
        let not_found = common::not_found("Test Entity");
        match not_found {
            ModmailError::Database(DatabaseError::NotFound(entity)) => {
                assert_eq!(entity, "Test Entity");
            }
            _ => panic!("not_found function failed"),
        }

        let permission_denied = common::permission_denied();
        match permission_denied {
            ModmailError::Permission(PermissionError::InsufficientPermissions) => {}
            _ => panic!("permission_denied function failed"),
        }

        let invalid_cmd = common::invalid_command();
        match invalid_cmd {
            ModmailError::Command(CommandError::InvalidFormat) => {}
            _ => panic!("invalid_command function failed"),
        }
    }

    #[test]
    fn test_error_display() {
        let db_error = ModmailError::Database(DatabaseError::ConnectionFailed);
        let display_str = format!("{}", db_error);
        assert!(display_str.contains("Database error"));
        assert!(display_str.contains("Failed to connect"));

        let validation_error =
            ModmailError::Validation(ValidationError::InvalidInput("test input".to_string()));
        let display_str = format!("{}", validation_error);
        assert!(display_str.contains("Validation error"));
        assert!(display_str.contains("test input"));
    }

    #[test]
    fn test_error_conversion() {
        let sqlx_err = sqlx::Error::RowNotFound;
        let modmail_err: ModmailError = sqlx_err.into();
        match modmail_err {
            ModmailError::Database(DatabaseError::NotFound(_)) => {}
            _ => panic!("sqlx error conversion failed"),
        }

        let invalid_toml = "invalid = [";
        let toml_err = toml::from_str::<toml::Value>(invalid_toml).unwrap_err();
        let modmail_err: ModmailError = toml_err.into();
        match modmail_err {
            ModmailError::Config(_) => {}
            _ => panic!("toml error conversion failed"),
        }
    }

    #[tokio::test]
    async fn test_error_handler_creation() {
        let handler = ErrorHandler::new();
        let user_id = UserId::new(12345);

        let lang = handler.get_user_language(user_id, None).await;
        assert_eq!(lang, Language::English);
    }

    #[tokio::test]
    async fn test_user_language_preferences() {
        let handler = ErrorHandler::new();
        let user_id = UserId::new(12345);

        let prefs = crate::i18n::languages::LanguagePreferences::new(Language::French);
        handler.set_user_language(user_id, prefs).await;

        let lang = handler.get_user_language(user_id, None).await;
        assert_eq!(lang, Language::French);
    }

    #[tokio::test]
    async fn test_guild_language_settings() {
        let handler = ErrorHandler::new();
        let guild_id = 67890;
        let user_id = UserId::new(12345);

        handler
            .set_guild_language(guild_id, Language::Spanish)
            .await;

        let lang = handler.get_user_language(user_id, Some(guild_id)).await;
        assert_eq!(lang, Language::Spanish);
    }

    #[test]
    fn test_dictionary_manager() {
        let manager = DictionaryManager::new();

        let english_dict = manager.get_dictionary(Language::English);
        assert!(english_dict.is_some());

        let message =
            manager.get_message(Language::English, "database.connection_failed", None, None);
        assert!(message.contains("Failed to connect"));
    }

    #[test]
    fn test_dictionary_with_parameters() {
        let manager = DictionaryManager::new();

        let mut params = HashMap::new();
        params.insert("error".to_string(), "Connection timeout".to_string());

        let message = manager.get_message(
            Language::English,
            "database.query_failed",
            Some(&params),
            None,
        );
        assert!(message.contains("Connection timeout"));
    }

    #[test]
    fn test_dictionary_fallback() {
        let manager = DictionaryManager::new();

        let message =
            manager.get_message(Language::Chinese, "database.connection_failed", None, None);
        assert!(!message.is_empty());
        assert!(!message.contains("Missing translation"));
    }

    #[test]
    fn test_error_translation() {
        let manager = DictionaryManager::new();
        let error = common::thread_not_found();

        let english_msg = manager.translate_error(&error, Language::English);
        assert!(english_msg.contains("Thread not found") || english_msg.contains("not found"));

        let french_msg = manager.translate_error(&error, Language::French);
        assert!(french_msg.contains("Thread non trouvé") || french_msg.contains("non trouvé"));
    }

    #[tokio::test]
    async fn test_formatted_error() {
        let handler = ErrorHandler::new();
        let user_id = UserId::new(12345);
        let error = common::permission_denied();

        let formatted = handler.handle_error(&error, Some(user_id), None).await;

        assert!(!formatted.message.is_empty());
        assert_eq!(formatted.language, Language::English);
        assert_eq!(formatted.error_type, "Permission");
        assert!(formatted.user_facing);
    }

    #[test]
    fn test_error_severity() {
        let handler = ErrorHandler::new();

        let critical_error = ModmailError::Discord(DiscordError::InvalidToken);
        let severity = handler.get_error_severity(&critical_error);
        assert_eq!(severity, crate::errors::handler::ErrorSeverity::Critical);

        let low_error = ModmailError::Command(CommandError::InvalidFormat);
        let severity = handler.get_error_severity(&low_error);
        assert_eq!(severity, crate::errors::handler::ErrorSeverity::Low);
    }

    #[test]
    fn test_error_logging_decision() {
        let handler = ErrorHandler::new();

        let critical_error = ModmailError::Discord(DiscordError::InvalidToken);
        assert!(handler.should_log_error(&critical_error));

        let low_error = ModmailError::Command(CommandError::InvalidFormat);
        assert!(!handler.should_log_error(&low_error));
    }

    #[test]
    fn test_supported_languages() {
        let handler = ErrorHandler::new();
        let languages = handler.get_supported_languages();

        assert!(languages.contains(&Language::English));
        assert!(languages.contains(&Language::French));
        assert!(!languages.is_empty());
    }

    #[test]
    fn test_language_validation() {
        let handler = ErrorHandler::new();

        assert!(handler.is_language_supported(Language::English));
        assert!(handler.is_language_supported(Language::French));
    }

    #[tokio::test]
    async fn test_language_detection() {
        let handler = ErrorHandler::new();
        let user_id = UserId::new(12345);

        let detected = handler
            .detect_language_from_interaction(user_id, Some("fr"))
            .await;
        assert_eq!(detected, Language::French);

        let user_lang = handler.get_user_language(user_id, None).await;
        assert_eq!(user_lang, Language::French);
    }

    #[test]
    fn test_error_context() {
        let context = crate::errors::handler::ErrorContext::new()
            .with_command("test".to_string())
            .with_user(UserId::new(12345))
            .with_info("key".to_string(), "value".to_string());

        assert_eq!(context.command, Some("test".to_string()));
        assert_eq!(context.user_id, Some(UserId::new(12345)));
        assert_eq!(
            context.additional_info.get("key"),
            Some(&"value".to_string())
        );
    }

    #[test]
    fn test_message_number_error() {
        let error = common::message_number_not_found(42);
        match error {
            ModmailError::Message(MessageError::MessageNumberNotFound(num)) => {
                assert_eq!(num, 42);
            }
            _ => panic!("Message number error creation failed"),
        }
    }

    #[test]
    fn test_validation_error_with_details() {
        let error = ModmailError::Validation(ValidationError::InvalidInput(
            "Username must be 3-20 characters".to_string(),
        ));
        match error {
            ModmailError::Validation(ValidationError::InvalidInput(msg)) => {
                assert!(msg.contains("Username"));
                assert!(msg.contains("3-20"));
            }
            _ => panic!("Validation error with details failed"),
        }
    }

    #[test]
    fn test_thread_error_types() {
        let not_found = ModmailError::Thread(ThreadError::ThreadNotFound);
        let already_exists = ModmailError::Thread(ThreadError::ThreadAlreadyExists);
        let creation_failed = ModmailError::Thread(ThreadError::ThreadCreationFailed);

        match not_found {
            ModmailError::Thread(ThreadError::ThreadNotFound) => {}
            _ => panic!("Thread not found error failed"),
        }

        match already_exists {
            ModmailError::Thread(ThreadError::ThreadAlreadyExists) => {}
            _ => panic!("Thread already exists error failed"),
        }

        match creation_failed {
            ModmailError::Thread(ThreadError::ThreadCreationFailed) => {}
            _ => panic!("Thread creation failed error failed"),
        }
    }

    #[test]
    fn test_message_error_types() {
        let too_long = ModmailError::Message(MessageError::MessageTooLong);
        let empty = ModmailError::Message(MessageError::MessageEmpty);
        let edit_failed =
            ModmailError::Message(MessageError::EditFailed("Permission denied".to_string()));

        match too_long {
            ModmailError::Message(MessageError::MessageTooLong) => {}
            _ => panic!("Message too long error failed"),
        }

        match empty {
            ModmailError::Message(MessageError::MessageEmpty) => {}
            _ => panic!("Message empty error failed"),
        }

        match edit_failed {
            ModmailError::Message(MessageError::EditFailed(msg)) => {
                assert_eq!(msg, "Permission denied");
            }
            _ => panic!("Message edit failed error failed"),
        }
    }

    #[test]
    fn test_permission_error_types() {
        let not_staff = ModmailError::Permission(PermissionError::NotStaffMember);
        let user_blocked = ModmailError::Permission(PermissionError::UserBlocked);
        let bot_missing = ModmailError::Permission(PermissionError::BotMissingPermissions(
            "SEND_MESSAGES".to_string(),
        ));

        match not_staff {
            ModmailError::Permission(PermissionError::NotStaffMember) => {}
            _ => panic!("Not staff member error failed"),
        }

        match user_blocked {
            ModmailError::Permission(PermissionError::UserBlocked) => {}
            _ => panic!("User blocked error failed"),
        }

        match bot_missing {
            ModmailError::Permission(PermissionError::BotMissingPermissions(perm)) => {
                assert_eq!(perm, "SEND_MESSAGES");
            }
            _ => panic!("Bot missing permissions error failed"),
        }
    }
}
