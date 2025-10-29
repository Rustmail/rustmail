pub mod dictionary;
pub mod handler;
pub mod types;

pub use dictionary::*;
pub use handler::*;
pub use types::*;

pub mod common {
    use super::*;
    use crate::{
        command_error, database_error, discord_error, message_error, permission_error,
        thread_error, validation_error,
    };

    pub fn not_found(entity: &str) -> ModmailError {
        database_error!(NotFound, entity)
    }

    pub fn incorrect_message_id(entity: &str) -> ModmailError {
        message_error!(MessageNotFound, entity)
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

    pub fn message_not_found(entity: &str) -> ModmailError {
        message_error!(MessageNotFound, entity)
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
    use crate::prelude::db::*;
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
    use crate::{database_error, discord_error};

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

pub use common::*;
pub use conversions::*;
pub use results::*;
