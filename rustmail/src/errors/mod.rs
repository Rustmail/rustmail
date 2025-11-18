pub mod dictionary;
pub mod handler;
pub mod types;

pub use dictionary::*;
pub use handler::*;
pub use types::*;

pub mod common {
    use super::*;
    use crate::{
        command_error, database_error, message_error, permission_error, thread_error,
        validation_error,
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

    pub fn thread_not_found() -> ModmailError {
        thread_error!(ThreadNotFound)
    }

    pub fn message_not_found(entity: &str) -> ModmailError {
        message_error!(MessageNotFound, entity)
    }

    pub fn database_connection_failed() -> ModmailError {
        database_error!(ConnectionFailed)
    }

    pub fn validation_failed(reason: &str) -> ModmailError {
        validation_error!(InvalidInput, reason)
    }
}

pub use common::*;
