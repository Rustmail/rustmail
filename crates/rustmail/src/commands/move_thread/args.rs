use crate::prelude::commands::*;
use crate::prelude::errors::*;
use serenity::all::ChannelId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveArgs {
    Category(ChannelId),
    Named(String),
}

pub fn parse_move_text(args: &str) -> ModmailResult<MoveArgs> {
    if args.is_empty() {
        return Err(ModmailError::Thread(ThreadError::CategoryNotFound));
    }
    Ok(MoveArgs::Named(args.to_string()))
}

pub fn parse_move_slash(options: &SlashOptions) -> ModmailResult<MoveArgs> {
    options
        .channel("category")
        .map(MoveArgs::Category)
        .ok_or(ModmailError::Command(CommandError::MissingArguments))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_category_name() {
        assert_eq!(
            parse_move_text("billing support").unwrap(),
            MoveArgs::Named("billing support".into())
        );
        assert!(parse_move_text("").is_err());
    }

    #[test]
    fn slash_category_channel() {
        let options =
            SlashOptions::default().with("category", OptionValue::Channel(ChannelId::new(4)));
        assert_eq!(
            parse_move_slash(&options).unwrap(),
            MoveArgs::Category(ChannelId::new(4))
        );
        assert!(parse_move_slash(&SlashOptions::default()).is_err());
    }
}
