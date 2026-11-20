use crate::prelude::commands::*;
use crate::prelude::errors::*;
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PresenceMode {
    Online,
    Idle,
    Dnd,
    Invisible,
    Maintenance,
}

pub fn parse_status_text(args: &str) -> ModmailResult<PresenceMode> {
    if args.is_empty() {
        return Err(ModmailError::Command(CommandError::StatusIsMissing));
    }
    PresenceMode::from_str(args)
        .map_err(|_| ModmailError::Command(CommandError::InvalidStatusValue))
}

pub fn parse_status_slash(options: &SlashOptions) -> ModmailResult<PresenceMode> {
    parse_status_text(options.string("mode").unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes() {
        assert_eq!(parse_status_text("dnd").unwrap(), PresenceMode::Dnd);
        assert_eq!(
            parse_status_text("maintenance").unwrap(),
            PresenceMode::Maintenance
        );
    }

    #[test]
    fn missing_and_unknown_modes() {
        assert!(matches!(
            parse_status_text(""),
            Err(ModmailError::Command(CommandError::StatusIsMissing))
        ));
        assert!(matches!(
            parse_status_text("busy"),
            Err(ModmailError::Command(CommandError::InvalidStatusValue))
        ));
        assert!(parse_status_slash(&SlashOptions::default()).is_err());
    }
}
