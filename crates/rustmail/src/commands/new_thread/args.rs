use crate::prelude::commands::*;
use crate::prelude::errors::*;
use crate::prelude::utils::*;
use serenity::all::UserId;

pub fn parse_new_thread_text(args: &str) -> ModmailResult<UserId> {
    if args.is_empty() {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }
    parse_user_id(args)
        .ok_or_else(|| ModmailError::Command(CommandError::InvalidArguments(args.to_string())))
}

pub fn parse_new_thread_slash(options: &SlashOptions) -> ModmailResult<UserId> {
    if let Some(user) = options.user("user_id") {
        return Ok(user);
    }
    match options.target() {
        Some(SlashTarget::User(user)) => Ok(*user),
        _ => Err(ModmailError::Command(CommandError::MissingArguments)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_user() {
        assert_eq!(parse_new_thread_text("<@!9>").unwrap(), UserId::new(9));
        assert!(parse_new_thread_text("").is_err());
        assert!(parse_new_thread_text("bob").is_err());
    }

    #[test]
    fn slash_option_or_context_menu_target() {
        let option = SlashOptions::default().with("user_id", OptionValue::User(UserId::new(1)));
        assert_eq!(parse_new_thread_slash(&option).unwrap(), UserId::new(1));

        let menu = SlashOptions::default().with_target(SlashTarget::User(UserId::new(2)));
        assert_eq!(parse_new_thread_slash(&menu).unwrap(), UserId::new(2));

        assert!(parse_new_thread_slash(&SlashOptions::default()).is_err());
    }
}
