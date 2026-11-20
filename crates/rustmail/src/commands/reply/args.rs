use crate::prelude::commands::*;
use crate::prelude::errors::*;
use serenity::all::Attachment;

#[derive(Debug, Clone, Default)]
pub struct ReplyArgs {
    pub content: Option<String>,
    pub snippet: Option<String>,
    pub attachments: Vec<Attachment>,
    pub anonymous: bool,
}

pub fn parse_reply_text(args: &str, anonymous: bool) -> ModmailResult<ReplyArgs> {
    let snippet = args
        .strip_prefix("{{")
        .and_then(|s| s.strip_suffix("}}"))
        .map(|key| key.trim().to_string());

    Ok(ReplyArgs {
        content: (snippet.is_none() && !args.is_empty()).then(|| args.to_string()),
        snippet,
        attachments: Vec::new(),
        anonymous,
    })
}

pub fn parse_reply_slash(options: &SlashOptions) -> ModmailResult<ReplyArgs> {
    Ok(ReplyArgs {
        content: options.string("message").map(str::to_string),
        snippet: options.string("snippet").map(str::to_string),
        attachments: options
            .attachment("attachment")
            .cloned()
            .into_iter()
            .collect(),
        anonymous: options.boolean("anonymous").unwrap_or(false),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_message() {
        let args = parse_reply_text("hello\nsecond line", false).unwrap();
        assert_eq!(args.content.as_deref(), Some("hello\nsecond line"));
        assert_eq!(args.snippet, None);
        assert!(!args.anonymous);
    }

    #[test]
    fn text_snippet() {
        let args = parse_reply_text("{{ welcome }}", true).unwrap();
        assert_eq!(args.snippet.as_deref(), Some("welcome"));
        assert_eq!(args.content, None);
        assert!(args.anonymous);
    }

    #[test]
    fn text_empty_leaves_content_to_attachments() {
        let args = parse_reply_text("", false).unwrap();
        assert_eq!(args.content, None);
        assert_eq!(args.snippet, None);
    }

    #[test]
    fn slash_options() {
        let options = SlashOptions::default()
            .with("message", OptionValue::String("hi".into()))
            .with("snippet", OptionValue::String("faq".into()))
            .with("anonymous", OptionValue::Boolean(true));
        let args = parse_reply_slash(&options).unwrap();
        assert_eq!(args.content.as_deref(), Some("hi"));
        assert_eq!(args.snippet.as_deref(), Some("faq"));
        assert!(args.anonymous);
        assert!(args.attachments.is_empty());
    }
}
