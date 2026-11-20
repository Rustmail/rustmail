use crate::prelude::commands::*;
use crate::prelude::errors::*;
use serenity::all::RoleId;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subscription {
    Subscribe,
    Unsubscribe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleQuery {
    Id(RoleId),
    Name(String),
}

impl RoleQuery {
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if let Some(id) = raw
            .strip_prefix("<@&")
            .and_then(|mention| mention.strip_suffix('>'))
        {
            return id
                .parse::<u64>()
                .ok()
                .filter(|&id| id != 0)
                .map(|id| RoleQuery::Id(RoleId::new(id)));
        }
        let name = raw.trim_start_matches('@');
        (!name.is_empty()).then(|| RoleQuery::Name(name.to_string()))
    }

    pub fn find(&self, roles: &HashMap<RoleId, String>) -> Option<RoleId> {
        match self {
            RoleQuery::Id(id) => roles.contains_key(id).then_some(*id),
            RoleQuery::Name(name) => roles
                .iter()
                .find(|(_, role_name)| role_name.eq_ignore_ascii_case(name))
                .map(|(id, _)| *id),
        }
    }

    pub fn label(&self) -> String {
        match self {
            RoleQuery::Id(id) => format!("<@&{id}>"),
            RoleQuery::Name(name) => name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReminderSubscriptionArgs {
    pub subscription: Subscription,
    pub role: RoleQuery,
}

pub fn parse_reminder_subscription_slash(
    options: &SlashOptions,
) -> ModmailResult<ReminderSubscriptionArgs> {
    let subscription = match options.string("action") {
        Some("subscribe") => Subscription::Subscribe,
        Some("unsubscribe") => Subscription::Unsubscribe,
        Some(other) => {
            return Err(ModmailError::Command(CommandError::InvalidArguments(
                other.to_string(),
            )));
        }
        None => return Err(ModmailError::Command(CommandError::MissingArguments)),
    };
    let role = options
        .role("role")
        .ok_or(ModmailError::Command(CommandError::MissingArguments))?;
    Ok(ReminderSubscriptionArgs {
        subscription,
        role: RoleQuery::Id(role),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roles() -> HashMap<RoleId, String> {
        HashMap::from([
            (RoleId::new(1), "Support".to_string()),
            (RoleId::new(2), "Admins".to_string()),
        ])
    }

    #[test]
    fn parses_mentions_and_names() {
        assert_eq!(
            RoleQuery::parse("<@&5>"),
            Some(RoleQuery::Id(RoleId::new(5)))
        );
        assert_eq!(
            RoleQuery::parse("@Support"),
            Some(RoleQuery::Name("Support".into()))
        );
        assert_eq!(
            RoleQuery::parse("night shift"),
            Some(RoleQuery::Name("night shift".into()))
        );
        assert_eq!(RoleQuery::parse(""), None);
        assert_eq!(RoleQuery::parse("<@&0>"), None);
    }

    #[test]
    fn finds_roles_by_id_or_case_insensitive_name() {
        assert_eq!(
            RoleQuery::Id(RoleId::new(2)).find(&roles()),
            Some(RoleId::new(2))
        );
        assert_eq!(RoleQuery::Id(RoleId::new(9)).find(&roles()), None);
        assert_eq!(
            RoleQuery::Name("support".into()).find(&roles()),
            Some(RoleId::new(1))
        );
        assert_eq!(RoleQuery::Name("nobody".into()).find(&roles()), None);
    }

    #[test]
    fn slash_action_and_role() {
        let options = SlashOptions::default()
            .with("action", OptionValue::String("unsubscribe".into()))
            .with("role", OptionValue::Role(RoleId::new(1)));
        assert_eq!(
            parse_reminder_subscription_slash(&options).unwrap(),
            ReminderSubscriptionArgs {
                subscription: Subscription::Unsubscribe,
                role: RoleQuery::Id(RoleId::new(1)),
            }
        );
        assert!(parse_reminder_subscription_slash(&SlashOptions::default()).is_err());
    }
}
