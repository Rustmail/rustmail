use crate::prelude::commands::*;
use crate::prelude::errors::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertAction {
    Set,
    Cancel,
}

pub fn parse_alert_text(args: &str) -> ModmailResult<AlertAction> {
    match args.to_lowercase().as_str() {
        "" => Ok(AlertAction::Set),
        "cancel" | "c" => Ok(AlertAction::Cancel),
        _ => Err(ModmailError::Command(CommandError::InvalidArguments(
            args.to_string(),
        ))),
    }
}

pub fn parse_alert_slash(options: &SlashOptions) -> ModmailResult<AlertAction> {
    Ok(if options.boolean("cancel").unwrap_or(false) {
        AlertAction::Cancel
    } else {
        AlertAction::Set
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_actions() {
        assert_eq!(parse_alert_text("").unwrap(), AlertAction::Set);
        assert_eq!(parse_alert_text("cancel").unwrap(), AlertAction::Cancel);
        assert_eq!(parse_alert_text("C").unwrap(), AlertAction::Cancel);
        assert!(parse_alert_text("check").is_err());
    }

    #[test]
    fn slash_cancel_flag() {
        let cancel = SlashOptions::default().with("cancel", OptionValue::Boolean(true));
        assert_eq!(parse_alert_slash(&cancel).unwrap(), AlertAction::Cancel);
        assert_eq!(
            parse_alert_slash(&SlashOptions::default()).unwrap(),
            AlertAction::Set
        );
    }
}
