use crate::prelude::commands::*;
use crate::prelude::errors::*;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseAction {
    Now,
    After(Duration),
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloseArgs {
    pub action: CloseAction,
    pub silent: bool,
}

pub fn parse_close_text(args: &str) -> ModmailResult<CloseArgs> {
    let mut silent = false;
    let mut cancel = false;
    let mut duration = None;

    for token in args.split_whitespace() {
        if token.eq_ignore_ascii_case("-s") || token.eq_ignore_ascii_case("--silent") {
            silent = true;
        } else if token.eq_ignore_ascii_case("cancel")
            || token.eq_ignore_ascii_case("-c")
            || token.eq_ignore_ascii_case("--cancel")
        {
            cancel = true;
        } else if duration.is_none()
            && let Some(parsed) = parse_duration_spec(token)
        {
            duration = Some(parsed);
        } else {
            return Err(invalid_arguments(token));
        }
    }

    Ok(CloseArgs {
        action: action(cancel, duration),
        silent,
    })
}

pub fn parse_close_slash(options: &SlashOptions) -> ModmailResult<CloseArgs> {
    let duration = match options.string("time_before_close") {
        Some(spec) => Some(parse_duration_spec(spec).ok_or_else(|| invalid_arguments(spec))?),
        None => None,
    };

    Ok(CloseArgs {
        action: action(options.boolean("cancel").unwrap_or(false), duration),
        silent: options.boolean("silent").unwrap_or(false),
    })
}

fn action(cancel: bool, duration: Option<Duration>) -> CloseAction {
    match (cancel, duration) {
        (true, _) => CloseAction::Cancel,
        (false, Some(duration)) => CloseAction::After(duration),
        (false, None) => CloseAction::Now,
    }
}

fn invalid_arguments(arg: &str) -> ModmailError {
    ModmailError::Command(CommandError::InvalidArguments(arg.to_string()))
}

pub fn parse_duration_spec(spec: &str) -> Option<Duration> {
    if spec.is_empty() {
        return None;
    }

    let mut total: u64 = 0;
    let mut num: u64 = 0;

    for ch in spec.chars() {
        if ch.is_ascii_digit() {
            let digit = ch.to_digit(10)? as u64;
            num = num.saturating_mul(10).saturating_add(digit);
        } else {
            let unit_secs = match ch {
                's' | 'S' => 1,
                'm' | 'M' => 60,
                'h' | 'H' => 3600,
                'd' | 'D' => 86400,
                _ => return None,
            };
            total = total.saturating_add(num.saturating_mul(unit_secs));
            num = 0;
        }
    }
    total = total.saturating_add(num);

    if total == 0 {
        None
    } else {
        Some(Duration::from_secs(total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn after(secs: u64) -> CloseAction {
        CloseAction::After(Duration::from_secs(secs))
    }

    #[test]
    fn text_without_arguments_closes_now() {
        assert_eq!(
            parse_close_text("").unwrap(),
            CloseArgs {
                action: CloseAction::Now,
                silent: false
            }
        );
    }

    #[test]
    fn text_duration_and_silent_in_any_order() {
        let expected = CloseArgs {
            action: after(5400),
            silent: true,
        };
        assert_eq!(parse_close_text("1h30m -s").unwrap(), expected);
        assert_eq!(parse_close_text("--silent 1h30m").unwrap(), expected);
    }

    #[test]
    fn text_cancel_wins_over_duration() {
        for args in ["cancel", "-c 2h", "2h --CANCEL"] {
            assert_eq!(parse_close_text(args).unwrap().action, CloseAction::Cancel);
        }
    }

    #[test]
    fn text_rejects_unknown_token_and_second_duration() {
        assert!(parse_close_text("soon").is_err());
        assert!(parse_close_text("1h 2h").is_err());
    }

    #[test]
    fn slash_options() {
        let options = SlashOptions::default()
            .with("time_before_close", OptionValue::String("10m".into()))
            .with("silent", OptionValue::Boolean(true));
        assert_eq!(
            parse_close_slash(&options).unwrap(),
            CloseArgs {
                action: after(600),
                silent: true
            }
        );

        let cancel = SlashOptions::default().with("cancel", OptionValue::Boolean(true));
        assert_eq!(
            parse_close_slash(&cancel).unwrap().action,
            CloseAction::Cancel
        );

        assert_eq!(
            parse_close_slash(&SlashOptions::default()).unwrap().action,
            CloseAction::Now
        );
    }

    #[test]
    fn slash_rejects_invalid_duration_instead_of_closing_now() {
        let options =
            SlashOptions::default().with("time_before_close", OptionValue::String("soon".into()));
        assert!(parse_close_slash(&options).is_err());
    }

    #[test]
    fn duration_spec() {
        assert_eq!(parse_duration_spec("90"), Some(Duration::from_secs(90)));
        assert_eq!(
            parse_duration_spec("1d2h"),
            Some(Duration::from_secs(93600))
        );
        assert_eq!(parse_duration_spec("5m30"), Some(Duration::from_secs(330)));
        assert_eq!(parse_duration_spec("0m"), None);
        assert_eq!(parse_duration_spec("1x"), None);
        assert_eq!(parse_duration_spec(""), None);
    }
}
