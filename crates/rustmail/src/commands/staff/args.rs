use crate::prelude::commands::*;
use crate::prelude::errors::*;
use crate::prelude::utils::*;
use serenity::all::{RoleId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaffTarget {
    User(UserId),
    Role(RoleId),
    UserOrRole(u64),
}

pub fn parse_staff_target_text(args: &str) -> ModmailResult<StaffTarget> {
    if args.is_empty() {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }
    if let Some(role) = args
        .strip_prefix("<@&")
        .and_then(|mention| mention.strip_suffix('>'))
    {
        return role
            .parse::<u64>()
            .ok()
            .filter(|&id| id != 0)
            .map(|id| StaffTarget::Role(RoleId::new(id)))
            .ok_or_else(|| invalid_target(args));
    }
    if args.starts_with("<@") {
        return parse_user_id(args)
            .map(StaffTarget::User)
            .ok_or_else(|| invalid_target(args));
    }
    args.parse::<u64>()
        .ok()
        .filter(|&id| id != 0)
        .map(StaffTarget::UserOrRole)
        .ok_or_else(|| invalid_target(args))
}

pub fn parse_staff_target_slash(options: &SlashOptions) -> ModmailResult<StaffTarget> {
    if let Some(user) = options.user("target") {
        return Ok(StaffTarget::User(user));
    }
    if let Some(role) = options.role("target") {
        return Ok(StaffTarget::Role(role));
    }
    match options.target() {
        Some(SlashTarget::User(user)) => Ok(StaffTarget::User(*user)),
        _ => Err(ModmailError::Command(CommandError::MissingArguments)),
    }
}

fn invalid_target(arg: &str) -> ModmailError {
    ModmailError::Command(CommandError::InvalidArguments(arg.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_mentions() {
        assert_eq!(
            parse_staff_target_text("<@42>").unwrap(),
            StaffTarget::User(UserId::new(42))
        );
        assert_eq!(
            parse_staff_target_text("<@!42>").unwrap(),
            StaffTarget::User(UserId::new(42))
        );
        assert_eq!(
            parse_staff_target_text("<@&42>").unwrap(),
            StaffTarget::Role(RoleId::new(42))
        );
    }

    #[test]
    fn text_bare_id_is_resolved_later() {
        assert_eq!(
            parse_staff_target_text("42").unwrap(),
            StaffTarget::UserOrRole(42)
        );
    }

    #[test]
    fn text_rejects_missing_and_invalid_targets() {
        for args in ["", "bob", "<@&x>", "<@x>", "0"] {
            assert!(parse_staff_target_text(args).is_err(), "{args:?}");
        }
    }

    #[test]
    fn slash_user_role_or_context_menu_target() {
        let user = SlashOptions::default().with("target", OptionValue::User(UserId::new(1)));
        assert_eq!(
            parse_staff_target_slash(&user).unwrap(),
            StaffTarget::User(UserId::new(1))
        );

        let role = SlashOptions::default().with("target", OptionValue::Role(RoleId::new(2)));
        assert_eq!(
            parse_staff_target_slash(&role).unwrap(),
            StaffTarget::Role(RoleId::new(2))
        );

        let menu = SlashOptions::default().with_target(SlashTarget::User(UserId::new(3)));
        assert_eq!(
            parse_staff_target_slash(&menu).unwrap(),
            StaffTarget::User(UserId::new(3))
        );

        assert!(parse_staff_target_slash(&SlashOptions::default()).is_err());
    }
}
