use crate::prelude::commands::*;
use crate::prelude::errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameArgs {
    pub label: Option<String>,
}

pub fn parse_rename_text(args: &str) -> ModmailResult<RenameArgs> {
    Ok(RenameArgs { label: label(args) })
}

pub fn parse_rename_slash(options: &SlashOptions) -> ModmailResult<RenameArgs> {
    Ok(RenameArgs {
        label: options.string("label").and_then(label),
    })
}

fn label(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_label_keeps_inner_spaces() {
        assert_eq!(
            parse_rename_text("  billing  issue ")
                .unwrap()
                .label
                .as_deref(),
            Some("billing  issue")
        );
    }

    #[test]
    fn text_without_label_clears_it() {
        for args in ["", "   "] {
            assert_eq!(parse_rename_text(args).unwrap().label, None, "{args:?}");
        }
    }

    #[test]
    fn slash_label() {
        let options = SlashOptions::default().with("label", OptionValue::String(" vip ".into()));
        assert_eq!(
            parse_rename_slash(&options).unwrap().label.as_deref(),
            Some("vip")
        );
    }

    #[test]
    fn slash_missing_or_blank_label_clears_it() {
        let blank = SlashOptions::default().with("label", OptionValue::String("  ".into()));
        assert_eq!(parse_rename_slash(&blank).unwrap().label, None);
        assert_eq!(
            parse_rename_slash(&SlashOptions::default()).unwrap().label,
            None
        );
    }
}
