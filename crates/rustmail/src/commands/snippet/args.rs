use crate::prelude::commands::*;
use crate::prelude::errors::*;

pub const MAX_SNIPPET_LENGTH: usize = 4000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnippetArgs {
    Create { key: String, content: String },
    List,
    Show { key: String },
    Edit { key: String, content: String },
    Delete { key: String },
    Use { key: String },
}

pub fn parse_snippet_text(args: &str) -> ModmailResult<SnippetArgs> {
    let (action, rest) = args.split_once(char::is_whitespace).unwrap_or((args, ""));
    let rest = rest.trim();
    let (key, content) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
    let content = content.trim();

    match action {
        "" => Err(usage("snippet.text_usage")),
        "list" => Ok(SnippetArgs::List),
        "create" if key.is_empty() || content.is_empty() => Err(usage("snippet.text_create_usage")),
        "create" => new_snippet(key, content),
        "edit" if key.is_empty() || content.is_empty() => Err(usage("snippet.text_edit_usage")),
        "edit" => edited_snippet(key, content),
        "show" if rest.is_empty() => Err(usage("snippet.text_show_usage")),
        "show" => Ok(SnippetArgs::Show {
            key: rest.to_string(),
        }),
        "delete" if rest.is_empty() => Err(usage("snippet.text_delete_usage")),
        "delete" => Ok(SnippetArgs::Delete {
            key: rest.to_string(),
        }),
        key => Ok(SnippetArgs::Use {
            key: key.to_string(),
        }),
    }
}

pub fn parse_snippet_slash(options: &SlashOptions) -> ModmailResult<SnippetArgs> {
    let key = || {
        options
            .string("key")
            .map(str::to_string)
            .ok_or(ModmailError::Command(CommandError::MissingArguments))
    };
    let content = || {
        options
            .string("content")
            .ok_or(ModmailError::Command(CommandError::MissingArguments))
    };

    match options.subcommand().as_slice() {
        ["create"] => new_snippet(&key()?, content()?),
        ["list"] => Ok(SnippetArgs::List),
        ["show"] => Ok(SnippetArgs::Show { key: key()? }),
        ["edit"] => edited_snippet(&key()?, content()?),
        ["delete"] => Ok(SnippetArgs::Delete { key: key()? }),
        ["use"] => Ok(SnippetArgs::Use { key: key()? }),
        _ => Err(usage("snippet.unknown_subcommand")),
    }
}

fn new_snippet(key: &str, content: &str) -> ModmailResult<SnippetArgs> {
    let valid_key = !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if !valid_key {
        return Err(ModmailError::Command(CommandError::InvalidSnippetKeyFormat));
    }
    check_length(content)?;
    Ok(SnippetArgs::Create {
        key: key.to_string(),
        content: content.to_string(),
    })
}

fn edited_snippet(key: &str, content: &str) -> ModmailResult<SnippetArgs> {
    check_length(content)?;
    Ok(SnippetArgs::Edit {
        key: key.to_string(),
        content: content.to_string(),
    })
}

fn check_length(content: &str) -> ModmailResult<()> {
    if content.len() > MAX_SNIPPET_LENGTH {
        return Err(ModmailError::Command(CommandError::SnippetContentTooLong));
    }
    Ok(())
}

fn usage(key: &'static str) -> ModmailError {
    ModmailError::Command(CommandError::Usage { key })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usage_key(result: ModmailResult<SnippetArgs>) -> &'static str {
        match result {
            Err(ModmailError::Command(CommandError::Usage { key })) => key,
            other => panic!("expected usage, got {other:?}"),
        }
    }

    #[test]
    fn text_actions() {
        assert_eq!(
            parse_snippet_text("create greet Hello  there").unwrap(),
            SnippetArgs::Create {
                key: "greet".into(),
                content: "Hello  there".into(),
            }
        );
        assert_eq!(parse_snippet_text("list").unwrap(), SnippetArgs::List);
        assert_eq!(
            parse_snippet_text("delete greet").unwrap(),
            SnippetArgs::Delete {
                key: "greet".into()
            }
        );
        assert_eq!(
            parse_snippet_text("greet").unwrap(),
            SnippetArgs::Use {
                key: "greet".into()
            }
        );
    }

    #[test]
    fn text_usage_messages() {
        assert_eq!(usage_key(parse_snippet_text("")), "snippet.text_usage");
        assert_eq!(
            usage_key(parse_snippet_text("create greet")),
            "snippet.text_create_usage"
        );
        assert_eq!(
            usage_key(parse_snippet_text("edit")),
            "snippet.text_edit_usage"
        );
        assert_eq!(
            usage_key(parse_snippet_text("show")),
            "snippet.text_show_usage"
        );
        assert_eq!(
            usage_key(parse_snippet_text("delete")),
            "snippet.text_delete_usage"
        );
    }

    #[test]
    fn create_validates_key_and_length() {
        assert!(matches!(
            parse_snippet_text("create bad!key hi"),
            Err(ModmailError::Command(CommandError::InvalidSnippetKeyFormat))
        ));
        let long = format!("create k {}", "x".repeat(MAX_SNIPPET_LENGTH + 1));
        assert!(matches!(
            parse_snippet_text(&long),
            Err(ModmailError::Command(CommandError::SnippetContentTooLong))
        ));
    }

    #[test]
    fn slash_subcommands() {
        let create = SlashOptions::default()
            .with_subcommand(&["create"])
            .with("key", OptionValue::String("greet".into()))
            .with("content", OptionValue::String("Hello".into()));
        assert_eq!(
            parse_snippet_slash(&create).unwrap(),
            SnippetArgs::Create {
                key: "greet".into(),
                content: "Hello".into(),
            }
        );

        let list = SlashOptions::default().with_subcommand(&["list"]);
        assert_eq!(parse_snippet_slash(&list).unwrap(), SnippetArgs::List);

        assert!(parse_snippet_slash(&SlashOptions::default()).is_err());
    }
}
