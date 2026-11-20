use serenity::all::UserId;

pub fn parse_user_id(raw: &str) -> Option<UserId> {
    let raw = raw.trim();
    let id = raw
        .strip_prefix("<@")
        .and_then(|mention| mention.strip_suffix('>'))
        .map(|inner| inner.strip_prefix('!').unwrap_or(inner))
        .unwrap_or(raw);
    id.parse::<u64>()
        .ok()
        .filter(|&id| id != 0)
        .map(UserId::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_id_and_mentions() {
        for raw in ["42", " 42 ", "<@42>", "<@!42>"] {
            assert_eq!(parse_user_id(raw), Some(UserId::new(42)), "{raw:?}");
        }
    }

    #[test]
    fn rejects_roles_names_and_zero() {
        for raw in ["", "<@&42>", "bob", "0", "<@>"] {
            assert_eq!(parse_user_id(raw), None, "{raw:?}");
        }
    }
}
