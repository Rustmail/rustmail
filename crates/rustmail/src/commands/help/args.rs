use crate::prelude::commands::*;
use crate::prelude::errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelpArgs {
    pub command: Option<String>,
}

pub fn parse_help_text(args: &str) -> ModmailResult<HelpArgs> {
    Ok(HelpArgs {
        command: args.split_whitespace().next().map(str::to_string),
    })
}

pub fn parse_help_slash(options: &SlashOptions) -> ModmailResult<HelpArgs> {
    Ok(HelpArgs {
        command: options
            .string("command")
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(str::to_string),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_takes_the_first_word() {
        assert_eq!(
            parse_help_text("close now").unwrap().command.as_deref(),
            Some("close")
        );
        assert_eq!(parse_help_text("").unwrap().command, None);
    }

    #[test]
    fn slash_command_option() {
        let options = SlashOptions::default().with("command", OptionValue::String(" logs ".into()));
        assert_eq!(
            parse_help_slash(&options).unwrap().command.as_deref(),
            Some("logs")
        );
        assert_eq!(
            parse_help_slash(&SlashOptions::default()).unwrap().command,
            None
        );
    }
}
