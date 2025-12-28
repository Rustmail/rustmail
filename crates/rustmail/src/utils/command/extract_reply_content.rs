pub fn extract_reply_content(
    content: &str,
    prefix: &str,
    command_names: &[&str],
) -> Option<String> {
    command_names.iter().find_map(|&cmd| {
        let expected_start = format!("{prefix}{cmd}");
        content
            .strip_prefix(&expected_start)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
    })
}
