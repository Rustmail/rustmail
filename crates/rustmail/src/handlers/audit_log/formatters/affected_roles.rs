use serenity::all::audit_log::AffectedRole;

const EMBED_FIELD_VALUE_LIMIT: usize = 1024;
const SEPARATOR: &str = ", ";

pub fn format_affected_roles(roles: &[AffectedRole]) -> String {
    let entries: Vec<String> = roles.iter().map(format_affected_role).collect();
    let mut value = String::new();

    for (shown, entry) in entries.iter().enumerate() {
        let separator = if shown == 0 { "" } else { SEPARATOR };
        let hidden_after = entries.len() - shown - 1;
        let overflow_reserve = if hidden_after > 0 {
            SEPARATOR.len() + overflow_marker(hidden_after).len()
        } else {
            0
        };

        if value.len() + separator.len() + entry.len() + overflow_reserve > EMBED_FIELD_VALUE_LIMIT
        {
            value.push_str(separator);
            value.push_str(&overflow_marker(entries.len() - shown));
            return value;
        }

        value.push_str(separator);
        value.push_str(entry);
    }

    value
}

fn format_affected_role(role: &AffectedRole) -> String {
    format!("<@&{}> (`{}`)", role.id.get(), role.name.replace('`', "'"))
}

fn overflow_marker(hidden: usize) -> String {
    format!("+{hidden}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serenity::all::RoleId;

    fn role(id: u64, name: &str) -> AffectedRole {
        serde_json::from_value(serde_json::json!({ "id": RoleId::new(id), "name": name })).unwrap()
    }

    #[test]
    fn formats_mention_with_name() {
        let roles = [role(1, "Admin"), role(2, "Mod")];
        assert_eq!(
            format_affected_roles(&roles),
            "<@&1> (`Admin`), <@&2> (`Mod`)"
        );
    }

    #[test]
    fn replaces_backticks_in_names() {
        let roles = [role(1, "a`b")];
        assert_eq!(format_affected_roles(&roles), "<@&1> (`a'b`)");
    }

    #[test]
    fn stays_within_embed_limit_and_counts_hidden_roles() {
        let long_name = "x".repeat(90);
        let roles: Vec<AffectedRole> = (1..=30).map(|id| role(id, &long_name)).collect();
        let value = format_affected_roles(&roles);

        assert!(value.len() <= EMBED_FIELD_VALUE_LIMIT);
        let shown = value.matches("<@&").count();
        assert!(shown > 0);
        assert!(value.ends_with(&format!(", +{}", roles.len() - shown)));
    }

    #[test]
    fn shows_all_roles_when_they_fit() {
        let roles: Vec<AffectedRole> = (1..=5).map(|id| role(id, "Role")).collect();
        let value = format_affected_roles(&roles);
        assert_eq!(value.matches("<@&").count(), 5);
        assert!(!value.contains('+'));
    }
}
