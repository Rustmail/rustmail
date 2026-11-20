use crate::prelude::commands::*;
use crate::prelude::errors::*;
use serenity::all::MessageId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteArgs {
    Number(i64),
    Message(MessageId),
}

pub fn parse_delete_text(args: &str) -> ModmailResult<DeleteArgs> {
    if args.is_empty() {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }
    match args.parse::<i64>() {
        Ok(number) if number > 0 => Ok(DeleteArgs::Number(number)),
        _ => Err(invalid_number(args)),
    }
}

pub fn parse_delete_slash(options: &SlashOptions) -> ModmailResult<DeleteArgs> {
    if let Some(value) = options.number("message_id") {
        if value.fract() != 0.0 || value < 1.0 {
            return Err(invalid_number(&value.to_string()));
        }
        return Ok(DeleteArgs::Number(value as i64));
    }
    match options.target() {
        Some(SlashTarget::Message(message)) => Ok(DeleteArgs::Message(message.id)),
        _ => Err(ModmailError::Command(CommandError::MissingArguments)),
    }
}

fn invalid_number(arg: &str) -> ModmailError {
    ModmailError::Command(CommandError::InvalidArguments(arg.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_number() {
        assert_eq!(parse_delete_text("3").unwrap(), DeleteArgs::Number(3));
    }

    #[test]
    fn text_rejects_missing_and_invalid_numbers() {
        for args in ["", "abc", "0", "-1", "2 3"] {
            assert!(parse_delete_text(args).is_err(), "{args:?}");
        }
    }

    #[test]
    fn slash_number() {
        let options = SlashOptions::default().with("message_id", OptionValue::Number(5.0));
        assert_eq!(parse_delete_slash(&options).unwrap(), DeleteArgs::Number(5));
    }

    #[test]
    fn slash_rejects_fractional_and_non_positive_numbers() {
        for value in [1.5, 0.0, -4.0] {
            let options = SlashOptions::default().with("message_id", OptionValue::Number(value));
            assert!(parse_delete_slash(&options).is_err(), "{value}");
        }
    }

    #[test]
    fn slash_without_number_or_target_is_missing_arguments() {
        assert!(matches!(
            parse_delete_slash(&SlashOptions::default()),
            Err(ModmailError::Command(CommandError::MissingArguments))
        ));
    }
}
