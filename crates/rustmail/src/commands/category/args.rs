use crate::prelude::commands::*;
use crate::prelude::errors::*;
use serenity::all::RoleId;

pub const MIN_SELECTION_TIMEOUT_SECONDS: i64 = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CategoryArgs {
    Create {
        discord_category_id: u64,
        name: String,
        description: Option<String>,
        emoji: Option<String>,
    },
    List,
    Rename {
        old_name: String,
        new_name: String,
    },
    Move {
        name: String,
        position: i64,
    },
    Delete {
        name: String,
    },
    SetEnabled {
        name: String,
        enabled: bool,
    },
    Timeout {
        seconds: i64,
    },
    Selection {
        enabled: bool,
    },
    Roles(RolesArgs),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RolesArgs {
    Add { name: String, role: RoleId },
    Remove { name: String, role: RoleId },
    List { name: String },
    Clear { name: String },
}

pub fn parse_category_text(args: &str) -> ModmailResult<CategoryArgs> {
    let (action, rest) = split_first(args);
    match action {
        "" => Err(usage("category.text_usage")),
        "create" => parse_create_text(rest),
        "list" => Ok(CategoryArgs::List),
        "rename" => {
            let (old_name, new_name) = split_first(rest);
            Ok(CategoryArgs::Rename {
                old_name: required(old_name, "category.text_usage")?,
                new_name: required(new_name, "category.text_usage")?,
            })
        }
        "move" => {
            let (name, position) = rest.rsplit_once(char::is_whitespace).unwrap_or(("", rest));
            Ok(CategoryArgs::Move {
                name: required(name, "category.text_usage")?,
                position: position
                    .trim()
                    .parse()
                    .map_err(|_| usage("category.text_usage"))?,
            })
        }
        "delete" | "remove" => Ok(CategoryArgs::Delete {
            name: required(rest, "category.text_usage")?,
        }),
        "enable" | "disable" => Ok(CategoryArgs::SetEnabled {
            name: required(rest, "category.text_usage")?,
            enabled: action == "enable",
        }),
        "timeout" => timeout(rest.parse().ok()),
        "on" => Ok(CategoryArgs::Selection { enabled: true }),
        "off" => Ok(CategoryArgs::Selection { enabled: false }),
        "roles" => parse_roles_text(rest).map(CategoryArgs::Roles),
        _ => Err(usage("category.unknown_subcommand")),
    }
}

fn parse_create_text(rest: &str) -> ModmailResult<CategoryArgs> {
    let (discord_id, details) = split_first(rest);
    if discord_id.is_empty() || details.is_empty() {
        return Err(usage("category.create_usage"));
    }
    let mut segments = details.split('|').map(str::trim);
    create(
        discord_id,
        segments.next().unwrap_or_default(),
        segments.next(),
        segments.next(),
    )
}

fn parse_roles_text(rest: &str) -> ModmailResult<RolesArgs> {
    let (action, rest) = split_first(rest);
    let name = || required(rest, "category.roles_usage");
    match action {
        "add" | "remove" => {
            let (name, role) = rest
                .rsplit_once(char::is_whitespace)
                .ok_or_else(|| usage("category.roles_usage"))?;
            let name = required(name, "category.roles_usage")?;
            let role = parse_role(role).ok_or_else(|| usage("category.roles_usage"))?;
            Ok(if action == "add" {
                RolesArgs::Add { name, role }
            } else {
                RolesArgs::Remove { name, role }
            })
        }
        "list" => Ok(RolesArgs::List { name: name()? }),
        "clear" => Ok(RolesArgs::Clear { name: name()? }),
        _ => Err(usage("category.roles_usage")),
    }
}

pub fn parse_category_slash(options: &SlashOptions) -> ModmailResult<CategoryArgs> {
    let string = |name: &str, usage_key: &'static str| {
        required(options.string(name).unwrap_or_default(), usage_key)
    };
    let category_name = || string("name", "category.text_usage");
    let role = || {
        options
            .role("role")
            .ok_or_else(|| usage("category.roles_usage"))
    };

    match options.subcommand().as_slice() {
        ["create"] => create(
            options.string("discord_category_id").unwrap_or_default(),
            options.string("name").unwrap_or_default(),
            options.string("description"),
            options.string("emoji"),
        ),
        ["list"] => Ok(CategoryArgs::List),
        ["delete"] => Ok(CategoryArgs::Delete {
            name: category_name()?,
        }),
        ["rename"] => Ok(CategoryArgs::Rename {
            old_name: string("old_name", "category.text_usage")?,
            new_name: string("new_name", "category.text_usage")?,
        }),
        ["move"] => Ok(CategoryArgs::Move {
            name: category_name()?,
            position: options
                .integer("position")
                .ok_or_else(|| usage("category.text_usage"))?,
        }),
        ["enable"] => Ok(CategoryArgs::SetEnabled {
            name: category_name()?,
            enabled: true,
        }),
        ["disable"] => Ok(CategoryArgs::SetEnabled {
            name: category_name()?,
            enabled: false,
        }),
        ["timeout"] => timeout(options.integer("seconds")),
        ["on"] => Ok(CategoryArgs::Selection { enabled: true }),
        ["off"] => Ok(CategoryArgs::Selection { enabled: false }),
        ["roles", action] => {
            let name = string("name", "category.roles_usage")?;
            Ok(CategoryArgs::Roles(match *action {
                "add" => RolesArgs::Add {
                    name,
                    role: role()?,
                },
                "remove" => RolesArgs::Remove {
                    name,
                    role: role()?,
                },
                "list" => RolesArgs::List { name },
                "clear" => RolesArgs::Clear { name },
                _ => return Err(usage("category.roles_usage")),
            }))
        }
        _ => Err(usage("category.unknown_subcommand")),
    }
}

fn create(
    discord_id: &str,
    name: &str,
    description: Option<&str>,
    emoji: Option<&str>,
) -> ModmailResult<CategoryArgs> {
    let name = required(name, "category.create_usage")?;
    let discord_category_id = discord_id
        .trim()
        .parse()
        .map_err(|_| ModmailError::Command(CommandError::InvalidDiscordCategory))?;
    let optional = |value: Option<&str>| {
        value
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
    };
    Ok(CategoryArgs::Create {
        discord_category_id,
        name,
        description: optional(description),
        emoji: optional(emoji),
    })
}

fn timeout(seconds: Option<i64>) -> ModmailResult<CategoryArgs> {
    seconds
        .filter(|&seconds| seconds >= MIN_SELECTION_TIMEOUT_SECONDS)
        .map(|seconds| CategoryArgs::Timeout { seconds })
        .ok_or_else(|| usage("category.text_usage"))
}

fn parse_role(raw: &str) -> Option<RoleId> {
    let raw = raw.trim();
    raw.strip_prefix("<@&")
        .and_then(|mention| mention.strip_suffix('>'))
        .unwrap_or(raw)
        .parse::<u64>()
        .ok()
        .filter(|&id| id != 0)
        .map(RoleId::new)
}

fn split_first(text: &str) -> (&str, &str) {
    let text = text.trim();
    let (first, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
    (first, rest.trim())
}

fn required(value: &str, usage_key: &'static str) -> ModmailResult<String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(usage(usage_key));
    }
    Ok(value.to_string())
}

fn usage(key: &'static str) -> ModmailError {
    ModmailError::Command(CommandError::Usage { key })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usage_key(result: ModmailResult<CategoryArgs>) -> &'static str {
        match result {
            Err(ModmailError::Command(CommandError::Usage { key })) => key,
            other => panic!("expected usage, got {other:?}"),
        }
    }

    #[test]
    fn text_create_with_optional_parts() {
        assert_eq!(
            parse_category_text("create 42 Billing | Payment issues | 💳").unwrap(),
            CategoryArgs::Create {
                discord_category_id: 42,
                name: "Billing".into(),
                description: Some("Payment issues".into()),
                emoji: Some("💳".into()),
            }
        );
        assert_eq!(
            parse_category_text("create 42 Billing").unwrap(),
            CategoryArgs::Create {
                discord_category_id: 42,
                name: "Billing".into(),
                description: None,
                emoji: None,
            }
        );
    }

    #[test]
    fn text_create_errors() {
        assert_eq!(
            usage_key(parse_category_text("create 42")),
            "category.create_usage"
        );
        assert!(matches!(
            parse_category_text("create abc Billing"),
            Err(ModmailError::Command(CommandError::InvalidDiscordCategory))
        ));
    }

    #[test]
    fn text_move_accepts_names_with_spaces() {
        assert_eq!(
            parse_category_text("move Billing support 2").unwrap(),
            CategoryArgs::Move {
                name: "Billing support".into(),
                position: 2,
            }
        );
        assert_eq!(
            usage_key(parse_category_text("move Billing")),
            "category.text_usage"
        );
    }

    #[test]
    fn text_toggles_and_timeout() {
        assert_eq!(
            parse_category_text("disable Billing").unwrap(),
            CategoryArgs::SetEnabled {
                name: "Billing".into(),
                enabled: false,
            }
        );
        assert_eq!(
            parse_category_text("off").unwrap(),
            CategoryArgs::Selection { enabled: false }
        );
        assert_eq!(
            parse_category_text("timeout 60").unwrap(),
            CategoryArgs::Timeout { seconds: 60 }
        );
        assert_eq!(
            usage_key(parse_category_text("timeout 10")),
            "category.text_usage"
        );
    }

    #[test]
    fn text_roles() {
        assert_eq!(
            parse_category_text("roles add Billing support <@&7>").unwrap(),
            CategoryArgs::Roles(RolesArgs::Add {
                name: "Billing support".into(),
                role: RoleId::new(7),
            })
        );
        assert_eq!(
            parse_category_text("roles clear Billing").unwrap(),
            CategoryArgs::Roles(RolesArgs::Clear {
                name: "Billing".into()
            })
        );
        assert_eq!(
            usage_key(parse_category_text("roles add Billing")),
            "category.roles_usage"
        );
        assert_eq!(
            usage_key(parse_category_text("roles nope")),
            "category.roles_usage"
        );
    }

    #[test]
    fn text_usage_and_unknown() {
        assert_eq!(usage_key(parse_category_text("")), "category.text_usage");
        assert_eq!(
            usage_key(parse_category_text("explode")),
            "category.unknown_subcommand"
        );
    }

    #[test]
    fn slash_subcommands() {
        let rename = SlashOptions::default()
            .with_subcommand(&["rename"])
            .with("old_name", OptionValue::String("A".into()))
            .with("new_name", OptionValue::String(" B ".into()));
        assert_eq!(
            parse_category_slash(&rename).unwrap(),
            CategoryArgs::Rename {
                old_name: "A".into(),
                new_name: "B".into(),
            }
        );

        let roles = SlashOptions::default()
            .with_subcommand(&["roles", "remove"])
            .with("name", OptionValue::String("A".into()))
            .with("role", OptionValue::Role(RoleId::new(3)));
        assert_eq!(
            parse_category_slash(&roles).unwrap(),
            CategoryArgs::Roles(RolesArgs::Remove {
                name: "A".into(),
                role: RoleId::new(3),
            })
        );

        let timeout = SlashOptions::default()
            .with_subcommand(&["timeout"])
            .with("seconds", OptionValue::Integer(5));
        assert!(parse_category_slash(&timeout).is_err());
    }
}
