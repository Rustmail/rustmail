use crate::prelude::commands::*;
use crate::prelude::errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditArgs {
    pub number: i64,
    pub content: String,
}

pub fn parse_edit_text(args: &str) -> ModmailResult<EditArgs> {
    let (number, content) = args.split_once(char::is_whitespace).unwrap_or((args, ""));
    edit_args(number.parse().ok(), number, content)
}

pub fn parse_edit_slash(options: &SlashOptions) -> ModmailResult<EditArgs> {
    let number = options
        .integer("message_id")
        .ok_or(ModmailError::Command(CommandError::MissingArguments))?;
    edit_args(
        Some(number),
        &number.to_string(),
        options.string("message").unwrap_or_default(),
    )
}

fn edit_args(number: Option<i64>, raw_number: &str, content: &str) -> ModmailResult<EditArgs> {
    if raw_number.is_empty() {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }
    let number = number.filter(|&number| number > 0).ok_or_else(|| {
        ModmailError::Command(CommandError::InvalidArguments(raw_number.to_string()))
    })?;
    let content = content.trim();
    if content.is_empty() {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }
    Ok(EditArgs {
        number,
        content: content.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_number_and_content() {
        assert_eq!(
            parse_edit_text("3 New message  content ").unwrap(),
            EditArgs {
                number: 3,
                content: "New message  content".into(),
            }
        );
    }

    #[test]
    fn text_rejects_missing_or_invalid_parts() {
        for args in ["", "3", "3   ", "abc hello", "0 hello", "-1 hello"] {
            assert!(parse_edit_text(args).is_err(), "{args:?}");
        }
    }

    #[test]
    fn slash_options() {
        let options = SlashOptions::default()
            .with("message_id", OptionValue::Integer(2))
            .with("message", OptionValue::String("fixed".into()));
        assert_eq!(
            parse_edit_slash(&options).unwrap(),
            EditArgs {
                number: 2,
                content: "fixed".into(),
            }
        );
        let zero = options.clone().with("message_id", OptionValue::Integer(0));
        assert!(parse_edit_slash(&zero).is_err());
        let blank = options.with("message", OptionValue::String(" ".into()));
        assert!(parse_edit_slash(&blank).is_err());
    }
}
