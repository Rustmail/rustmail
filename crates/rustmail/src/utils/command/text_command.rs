pub fn split_text_command<'a>(content: &'a str, prefix: &str) -> Option<(&'a str, &'a str)> {
    let rest = content.strip_prefix(prefix)?;
    match rest.find(char::is_whitespace) {
        Some(i) => Some((&rest[..i], rest[i..].trim())),
        None => Some((rest, "")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_char_prefix() {
        assert_eq!(split_text_command("!close", "!"), Some(("close", "")));
        assert_eq!(
            split_text_command("!close 1h -s", "!"),
            Some(("close", "1h -s"))
        );
    }

    #[test]
    fn multi_char_prefix_without_arguments() {
        assert_eq!(split_text_command("mm!close", "mm!"), Some(("close", "")));
        assert_eq!(split_text_command("mm!c 2h", "mm!"), Some(("c", "2h")));
    }

    #[test]
    fn newline_separates_arguments() {
        assert_eq!(
            split_text_command("!reply\nhello there", "!"),
            Some(("reply", "hello there"))
        );
    }

    #[test]
    fn missing_prefix() {
        assert_eq!(split_text_command("close", "!"), None);
    }
}
