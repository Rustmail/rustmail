use serenity::all::{ChannelId, Context, Message, UserId};
use crate::db::{get_message_ids_by_number, get_thread_by_channel_id};
use crate::db::messages::get_thread_message_by_inbox_message_id;
use crate::errors::{ModmailError, ModmailResult, CommandError, ValidationError as ErrorValidationError, command_error};
use crate::errors::common::{not_found, permission_denied, thread_not_found};
use crate::i18n::get_translated_message;

#[derive(Debug)]
pub struct EditCommandInput {
    pub message_number: i64,
    pub new_content: String,
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidFormat,
    MissingMessageNumber,
    MissingContent,
    InvalidMessageNumber,
    EmptyContent,
}

impl From<ValidationError> for ModmailError {
    fn from(err: ValidationError) -> Self {
        match err {
            ValidationError::InvalidFormat => command_error!(InvalidFormat),
            ValidationError::MissingMessageNumber => command_error!(MissingArguments),
            ValidationError::MissingContent => command_error!(MissingArguments),
            ValidationError::InvalidMessageNumber => ModmailError::Validation(ErrorValidationError::InvalidInput("message number must be positive".to_string())),
            ValidationError::EmptyContent => ModmailError::Validation(ErrorValidationError::RequiredFieldMissing("message content".to_string())),
        }
    }
}

impl ValidationError {
    pub async fn _error_message(&self, config: &crate::config::Config, msg: &Message) -> String {
        let key = match self {
            ValidationError::InvalidFormat => "edit.validation.invalid_format",
            ValidationError::MissingMessageNumber => "edit.validation.missing_number",
            ValidationError::MissingContent => "edit.validation.missing_content",
            ValidationError::InvalidMessageNumber => "edit.validation.invalid_number",
            ValidationError::EmptyContent => "edit.validation.empty_content",
        };
        get_translated_message(
            config,
            key,
            None,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
            None
        ).await
    }

    pub async fn _send_error(&self, ctx: &Context, msg: &Message, config: &crate::config::Config) {
        let error_msg = self._error_message(config, msg).await;
        let _ = msg.reply(ctx, error_msg).await;
    }
}

pub fn parse_edit_command(raw_content: &str) -> ModmailResult<EditCommandInput> {
    let trimmed_content = raw_content.trim();

    if trimmed_content.is_empty() {
        return Err(ValidationError::InvalidFormat.into());
    }

    let mut parts = trimmed_content.splitn(2, ' ');

    let message_number_str = parts.next().ok_or(ValidationError::MissingMessageNumber)?;
    let message_content = parts.next().ok_or(ValidationError::MissingContent)?;

    let message_number: i64 = message_number_str
        .parse::<i64>()
        .map_err(|_| ValidationError::InvalidMessageNumber)?;

    if message_number <= 0 {
        return Err(ValidationError::InvalidMessageNumber.into());
    }

    let new_content = message_content.trim();
    if new_content.is_empty() {
        return Err(ValidationError::EmptyContent.into());
    }

    Ok(EditCommandInput {
        message_number,
        new_content: new_content.to_string(),
    })
}

pub async fn validate_edit_permissions(
    message_number: i64,
    channel_id: ChannelId,
    user_id: UserId,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    let thread_id = match get_thread_by_channel_id(&channel_id.to_string(), pool).await {
        Some(thread) => thread.id,
        None => return Err(thread_not_found())
    };

    let ids = match get_message_ids_by_number(
        message_number,
        user_id,
        &thread_id,
        pool
    ).await {
        Some(ids) => ids,
        None => return Err(
            not_found("An error occurred during the retrieval of message_ids in the edit command.")
        ),
    };

    let inbox_message_id = match ids.inbox_message_id {
        Some(inbox_message_id) => inbox_message_id,
        None => return Err(
            not_found("inbox_message_id doesn't exist !")
        )
    };

    let thread_message = match get_thread_message_by_inbox_message_id(
        &inbox_message_id,
        pool
    ).await {
        Ok(thread_message) => thread_message,
        Err(e) => return Err(permission_denied())
    };

    if thread_message.user_id != user_id.get() as i64 {
        return Err(permission_denied());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_command() {
        let result = parse_edit_command("3 New message content");
        assert!(result.is_ok());

        let input = result.unwrap();
        assert_eq!(input.message_number, 3);
        assert_eq!(input.new_content, "New message content");
    }

    #[test]
    fn test_parse_command_with_extra_spaces() {
        let result = parse_edit_command("  5   This is a test message  ");
        assert!(result.is_ok());

        let input = result.unwrap();
        assert_eq!(input.message_number, 5);
        assert_eq!(input.new_content, "This is a test message");
    }

    #[test]
    fn test_parse_invalid_number() {
        let result = parse_edit_command("abc New content");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_negative_number() {
        let result = parse_edit_command("-1 New content");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_zero_number() {
        let result = parse_edit_command("0 New content");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_content() {
        let result = parse_edit_command("3");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_content() {
        let result = parse_edit_command("3   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_input() {
        let result = parse_edit_command("");
        assert!(result.is_err());
    }
}
