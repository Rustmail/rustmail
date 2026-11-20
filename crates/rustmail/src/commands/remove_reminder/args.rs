use crate::prelude::commands::*;
use crate::prelude::errors::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoveReminderArgs {
    pub id: i64,
}

pub fn parse_remove_reminder_text(args: &str) -> ModmailResult<RemoveReminderArgs> {
    if args.is_empty() {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }
    match args.parse::<i64>() {
        Ok(id) if id > 0 => Ok(RemoveReminderArgs { id }),
        _ => Err(invalid_id(args)),
    }
}

pub fn parse_remove_reminder_slash(options: &SlashOptions) -> ModmailResult<RemoveReminderArgs> {
    let value = options
        .number("id")
        .ok_or(ModmailError::Command(CommandError::MissingArguments))?;
    if value.fract() != 0.0 || value < 1.0 {
        return Err(invalid_id(&value.to_string()));
    }
    Ok(RemoveReminderArgs { id: value as i64 })
}

fn invalid_id(arg: &str) -> ModmailError {
    ModmailError::Command(CommandError::InvalidArguments(arg.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_id() {
        assert_eq!(
            parse_remove_reminder_text("42").unwrap(),
            RemoveReminderArgs { id: 42 }
        );
    }

    #[test]
    fn text_rejects_missing_and_invalid_ids() {
        for args in ["", "abc", "0", "-3", "4 5"] {
            assert!(parse_remove_reminder_text(args).is_err(), "{args:?}");
        }
    }

    #[test]
    fn slash_id() {
        let options = SlashOptions::default().with("id", OptionValue::Number(7.0));
        assert_eq!(
            parse_remove_reminder_slash(&options).unwrap(),
            RemoveReminderArgs { id: 7 }
        );
    }

    #[test]
    fn slash_rejects_fractional_and_non_positive_ids() {
        for value in [1.5, 0.0, -2.0] {
            let options = SlashOptions::default().with("id", OptionValue::Number(value));
            assert!(parse_remove_reminder_slash(&options).is_err(), "{value}");
        }
        assert!(parse_remove_reminder_slash(&SlashOptions::default()).is_err());
    }
}
