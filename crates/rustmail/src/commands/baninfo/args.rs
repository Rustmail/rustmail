use crate::prelude::commands::*;
use crate::prelude::errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaninfoArgs {
    pub query: String,
}

pub fn parse_baninfo_text(args: &str) -> ModmailResult<BaninfoArgs> {
    query(args)
}

pub fn parse_baninfo_slash(options: &SlashOptions) -> ModmailResult<BaninfoArgs> {
    query(options.string("query").unwrap_or_default())
}

fn query(raw: &str) -> ModmailResult<BaninfoArgs> {
    let query = raw.trim();
    if query.is_empty() {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }
    Ok(BaninfoArgs {
        query: query.to_string(),
    })
}

pub fn normalize_banned_user_query(query: &str) -> &str {
    let trimmed = query.trim();
    trimmed
        .strip_prefix("<@")
        .and_then(|mention| mention.strip_suffix('>'))
        .map(|inner| inner.strip_prefix('!').unwrap_or(inner))
        .unwrap_or_else(|| trimmed.trim_start_matches('@'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_query() {
        assert_eq!(parse_baninfo_text("bob smith").unwrap().query, "bob smith");
    }

    #[test]
    fn missing_query() {
        assert!(parse_baninfo_text("").is_err());
        let blank = SlashOptions::default().with("query", OptionValue::String("  ".into()));
        assert!(parse_baninfo_slash(&blank).is_err());
        assert!(parse_baninfo_slash(&SlashOptions::default()).is_err());
    }

    #[test]
    fn normalizes_mentions_and_at_names() {
        assert_eq!(normalize_banned_user_query("<@42>"), "42");
        assert_eq!(normalize_banned_user_query("<@!42>"), "42");
        assert_eq!(normalize_banned_user_query(" @bob "), "bob");
        assert_eq!(normalize_banned_user_query("bob"), "bob");
    }
}
