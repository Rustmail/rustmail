pub mod dictionary;
pub mod handler;
pub mod types;

pub use types::{
    CommandError, ConfigError, DatabaseError, DiscordError, MessageError, ModmailError,
    ModmailResult, PermissionError, ThreadError, ValidationError,
};

pub use dictionary::{DictionaryManager, ErrorDictionary, ErrorMessage};

pub use handler::{
    ErrorContext, ErrorHandler, ErrorHandling, ErrorSeverity, FormattedError, FormattedMessage,
    MessageType,
};

pub use crate::{
    command_error, database_error, discord_error, handle_error, message_error, permission_error,
    send_success, thread_error, translate_error, validation_error,
};

pub mod common {
    use super::*;
    use serenity::all::UserId;

    pub fn not_found(entity: &str) -> ModmailError {
        database_error!(NotFound, entity)
    }

    pub fn permission_denied() -> ModmailError {
        permission_error!(InsufficientPermissions)
    }

    pub fn invalid_command() -> ModmailError {
        command_error!(InvalidFormat)
    }

    pub fn user_not_found() -> ModmailError {
        discord_error!(UserNotFound)
    }

    pub fn channel_not_found() -> ModmailError {
        discord_error!(ChannelNotFound)
    }

    pub fn thread_not_found() -> ModmailError {
        thread_error!(ThreadNotFound)
    }

    pub fn message_not_found() -> ModmailError {
        message_error!(MessageNotFound)
    }

    pub fn message_number_not_found(number: i64) -> ModmailError {
        ModmailError::Message(MessageError::MessageNumberNotFound(number))
    }

    pub fn database_connection_failed() -> ModmailError {
        database_error!(ConnectionFailed)
    }

    pub fn validation_failed(reason: &str) -> ModmailError {
        validation_error!(InvalidInput, reason)
    }

    pub fn not_staff_member() -> ModmailError {
        permission_error!(NotStaffMember)
    }

    pub fn user_blocked() -> ModmailError {
        permission_error!(UserBlocked)
    }
}

pub mod results {
    use super::*;
    use crate::db::repr::Thread;
    use serenity::all::{Channel, Message};

    pub type DatabaseResult<T> = Result<T, DatabaseError>;

    pub type DiscordResult<T> = Result<T, DiscordError>;

    pub type CommandResult<T> = Result<T, CommandError>;

    pub type ThreadResult<T> = Result<T, ThreadError>;

    pub type MessageResult<T> = Result<T, MessageError>;

    pub type ThreadQueryResult = ModmailResult<Option<Thread>>;
    pub type MessageQueryResult = ModmailResult<Option<Message>>;
    pub type ChannelQueryResult = ModmailResult<Option<Channel>>;
}

pub mod conversions {
    use super::*;

    pub fn from_serenity_with_context(err: serenity::Error, context: &str) -> ModmailError {
        match err {
            serenity::Error::Http(http_err) => {
                discord_error!(ApiError, format!("{}: {}", context, http_err))
            }
            serenity::Error::Model(model_err) => {
                discord_error!(ApiError, format!("{}: {}", context, model_err))
            }
            _ => discord_error!(ApiError, format!("{}: {}", context, err)),
        }
    }

    pub fn from_sqlx_with_context(err: sqlx::Error, context: &str) -> ModmailError {
        match err {
            sqlx::Error::RowNotFound => {
                database_error!(NotFound, context)
            }
            sqlx::Error::Database(db_err) => {
                database_error!(QueryFailed, format!("{}: {}", context, db_err))
            }
            _ => database_error!(QueryFailed, format!("{}: {}", context, err)),
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use crate::i18n::languages::Language;

    pub fn create_test_error_handler() -> ErrorHandler {
        ErrorHandler::new()
    }

    pub fn create_test_dictionary(language: Language) -> ErrorDictionary {
        ErrorDictionary::new(language)
    }

    pub fn assert_error_type(error: &ModmailError, expected_type: &str) {
        let actual_type = match error {
            ModmailError::Database(_) => "Database",
            ModmailError::Discord(_) => "Discord",
            ModmailError::Command(_) => "Command",
            ModmailError::Thread(_) => "Thread",
            ModmailError::Message(_) => "Message",
            ModmailError::Config(_) => "Config",
            ModmailError::Validation(_) => "Validation",
            ModmailError::Permission(_) => "Permission",
            ModmailError::Generic(_) => "Generic",
        };
        assert_eq!(actual_type, expected_type);
    }
}

#[cfg(test)]
pub mod tests;

pub mod constants {
    pub mod database {
        pub const CONNECTION_FAILED: &str = "database.connection_failed";
        pub const QUERY_FAILED: &str = "database.query_failed";
        pub const NOT_FOUND: &str = "database.not_found";
        pub const TRANSACTION_FAILED: &str = "database.transaction_failed";
    }

    pub mod discord {
        pub const CHANNEL_NOT_FOUND: &str = "discord.channel_not_found";
        pub const USER_NOT_FOUND: &str = "discord.user_not_found";
        pub const PERMISSION_DENIED: &str = "discord.permission_denied";
        pub const DM_CREATION_FAILED: &str = "discord.dm_creation_failed";
        pub const API_ERROR: &str = "discord.api_error";
    }

    pub mod command {
        pub const INVALID_FORMAT: &str = "command.invalid_format";
        pub const MISSING_ARGUMENTS: &str = "command.missing_arguments";
        pub const INVALID_ARGUMENTS: &str = "command.invalid_arguments";
        pub const UNKNOWN_COMMAND: &str = "command.unknown_command";
        pub const INSUFFICIENT_PERMISSIONS: &str = "command.insufficient_permissions";
    }

    pub mod thread {
        pub const NOT_FOUND: &str = "thread.not_found";
        pub const ALREADY_EXISTS: &str = "thread.already_exists";
        pub const CREATION_FAILED: &str = "thread.creation_failed";
    }

    pub mod message {
        pub const NOT_FOUND: &str = "message.not_found";
        pub const NUMBER_NOT_FOUND: &str = "message.number_not_found";
        pub const EDIT_FAILED: &str = "message.edit_failed";
        pub const SEND_FAILED: &str = "message.send_failed";
        pub const TOO_LONG: &str = "message.too_long";
        pub const EMPTY: &str = "message.empty";
    }

    pub mod permission {
        pub const NOT_STAFF_MEMBER: &str = "permission.not_staff_member";
        pub const USER_BLOCKED: &str = "permission.user_blocked";
        pub const INSUFFICIENT_PERMISSIONS: &str = "permission.insufficient_permissions";
    }

    pub mod success {
        pub const MESSAGE_SENT: &str = "success.message_sent";
        pub const MESSAGE_EDITED: &str = "success.message_edited";
        pub const THREAD_CREATED: &str = "success.thread_created";
    }
}
