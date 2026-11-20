use crate::prelude::commands::*;
use crate::prelude::errors::*;
use crate::prelude::utils::*;
use serenity::all::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogsArgs {
    pub user: Option<UserId>,
}

pub fn parse_logs_text(args: &str) -> ModmailResult<LogsArgs> {
    if args.is_empty() {
        return Ok(LogsArgs { user: None });
    }
    parse_user_id(args)
        .map(|user| LogsArgs { user: Some(user) })
        .ok_or_else(|| ModmailError::Command(CommandError::InvalidArguments(args.to_string())))
}

pub fn parse_logs_slash(options: &SlashOptions) -> ModmailResult<LogsArgs> {
    Ok(LogsArgs {
        user: options.user("id"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_without_user_targets_the_ticket() {
        assert_eq!(parse_logs_text("").unwrap(), LogsArgs { user: None });
    }

    #[test]
    fn text_user_id_or_mention() {
        for args in ["42", "<@42>"] {
            assert_eq!(parse_logs_text(args).unwrap().user, Some(UserId::new(42)));
        }
    }

    #[test]
    fn text_rejects_anything_else() {
        assert!(parse_logs_text("bob").is_err());
    }

    #[test]
    fn slash_user_option() {
        let options = SlashOptions::default().with("id", OptionValue::User(UserId::new(7)));
        assert_eq!(
            parse_logs_slash(&options).unwrap().user,
            Some(UserId::new(7))
        );
        assert_eq!(
            parse_logs_slash(&SlashOptions::default()).unwrap().user,
            None
        );
    }
}
