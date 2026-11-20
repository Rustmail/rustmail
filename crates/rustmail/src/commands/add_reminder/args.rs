use crate::prelude::commands::*;
use crate::prelude::errors::*;
use chrono::{DateTime, Duration, NaiveTime, TimeZone};
use serenity::all::RoleId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemindArgs {
    Add(NewReminder),
    Subscription(ReminderSubscriptionArgs),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewReminder {
    pub time: NaiveTime,
    pub content: ReminderContent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReminderContent {
    LeadingRoles(String),
    Separate {
        content: String,
        roles: Option<String>,
    },
}

pub fn parse_remind_text(args: &str) -> ModmailResult<RemindArgs> {
    let (first, rest) = args.split_once(char::is_whitespace).unwrap_or((args, ""));
    let rest = rest.trim();

    let subscription = match first.to_lowercase().as_str() {
        "subscribe" | "sub" => Some(Subscription::Subscribe),
        "unsubscribe" | "unsub" => Some(Subscription::Unsubscribe),
        _ => None,
    };
    if let Some(subscription) = subscription {
        let role =
            RoleQuery::parse(rest).ok_or(ModmailError::Command(CommandError::MissingArguments))?;
        return Ok(RemindArgs::Subscription(ReminderSubscriptionArgs {
            subscription,
            role,
        }));
    }

    Ok(RemindArgs::Add(NewReminder {
        time: parse_reminder_time(first).ok_or_else(invalid_format)?,
        content: ReminderContent::LeadingRoles(rest.to_string()),
    }))
}

pub fn parse_remind_slash(options: &SlashOptions) -> ModmailResult<RemindArgs> {
    let time = options
        .string("time")
        .and_then(parse_reminder_time)
        .ok_or_else(invalid_format)?;
    let content = options.string("content").ok_or_else(invalid_format)?;
    Ok(RemindArgs::Add(NewReminder {
        time,
        content: ReminderContent::Separate {
            content: content.to_string(),
            roles: options.string("roles").map(str::to_string),
        },
    }))
}

pub fn parse_reminder_time(raw: &str) -> Option<NaiveTime> {
    let (hour, minute) = raw.trim().split_once(':')?;
    let valid_hour = (1..=2).contains(&hour.len()) && hour.bytes().all(|b| b.is_ascii_digit());
    let valid_minute = minute.len() == 2 && minute.bytes().all(|b| b.is_ascii_digit());
    if !valid_hour || !valid_minute {
        return None;
    }
    NaiveTime::from_hms_opt(hour.parse().ok()?, minute.parse().ok()?, 0)
}

pub fn next_occurrence<Tz: TimeZone>(time: NaiveTime, now: &DateTime<Tz>) -> Option<DateTime<Tz>> {
    let today = now
        .timezone()
        .from_local_datetime(&now.date_naive().and_time(time))
        .earliest()?;
    Some(if today < *now {
        today + Duration::days(1)
    } else {
        today
    })
}

pub fn split_leading_roles(
    text: &str,
    resolve: impl Fn(&RoleQuery) -> Option<RoleId>,
) -> (Vec<RoleId>, String) {
    let is_separator = |c: char| c.is_whitespace() || c == ',';
    let mut roles = Vec::new();
    let mut rest = text;

    loop {
        let trimmed = rest.trim_start_matches(is_separator);
        let word_end = trimmed.find(is_separator).unwrap_or(trimmed.len());
        let word = &trimmed[..word_end];
        if !(word.starts_with('@') || word.starts_with("<@&")) {
            break;
        }
        match RoleQuery::parse(word).as_ref().and_then(&resolve) {
            Some(role) => {
                roles.push(role);
                rest = &trimmed[word_end..];
            }
            None => break,
        }
    }

    if roles.is_empty() {
        return (roles, text.to_string());
    }
    (roles, rest.trim_start_matches(is_separator).to_string())
}

pub fn parse_role_list(roles: &str, resolve: impl Fn(&RoleQuery) -> Option<RoleId>) -> Vec<RoleId> {
    let words: Vec<&str> = roles
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|word| !word.is_empty())
        .collect();
    let has_mentions = words.iter().any(|word| word.starts_with("<@&"));

    let mut found = Vec::new();
    for word in words {
        if has_mentions != word.starts_with("<@&") {
            continue;
        }
        if let Some(role) = RoleQuery::parse(word).as_ref().and_then(&resolve)
            && !found.contains(&role)
        {
            found.push(role);
        }
    }
    found
}

fn invalid_format() -> ModmailError {
    ModmailError::Command(CommandError::InvalidReminderFormat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, NaiveDate};
    use std::collections::HashMap;

    fn roles() -> HashMap<RoleId, String> {
        HashMap::from([
            (RoleId::new(1), "Support".to_string()),
            (RoleId::new(2), "Admins".to_string()),
        ])
    }

    fn resolve(query: &RoleQuery) -> Option<RoleId> {
        query.find(&roles())
    }

    #[test]
    fn reminder_times() {
        assert_eq!(
            parse_reminder_time("9:05"),
            NaiveTime::from_hms_opt(9, 5, 0)
        );
        assert_eq!(
            parse_reminder_time("23:59"),
            NaiveTime::from_hms_opt(23, 59, 0)
        );
        for raw in ["24:00", "12:60", "12:5", "123:00", "12", "ab:cd", "-1:00"] {
            assert_eq!(parse_reminder_time(raw), None, "{raw:?}");
        }
    }

    #[test]
    fn next_occurrence_rolls_over_to_tomorrow() {
        let tz = FixedOffset::east_opt(3600).unwrap();
        let now = tz
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2026, 1, 10)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            )
            .unwrap();

        let later = next_occurrence(NaiveTime::from_hms_opt(13, 0, 0).unwrap(), &now).unwrap();
        assert_eq!(later.date_naive().to_string(), "2026-01-10");

        let earlier = next_occurrence(NaiveTime::from_hms_opt(11, 0, 0).unwrap(), &now).unwrap();
        assert_eq!(earlier.date_naive().to_string(), "2026-01-11");
    }

    #[test]
    fn text_reminder() {
        assert_eq!(
            parse_remind_text("14:30 @Support check back").unwrap(),
            RemindArgs::Add(NewReminder {
                time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
                content: ReminderContent::LeadingRoles("@Support check back".into()),
            })
        );
        assert!(parse_remind_text("").is_err());
        assert!(parse_remind_text("tomorrow hi").is_err());
    }

    #[test]
    fn text_subscription() {
        assert_eq!(
            parse_remind_text("unsub Support").unwrap(),
            RemindArgs::Subscription(ReminderSubscriptionArgs {
                subscription: Subscription::Unsubscribe,
                role: RoleQuery::Name("Support".into()),
            })
        );
        assert!(parse_remind_text("sub").is_err());
    }

    #[test]
    fn slash_reminder() {
        let options = SlashOptions::default()
            .with("time", OptionValue::String("8:00".into()))
            .with("content", OptionValue::String("standup".into()));
        assert_eq!(
            parse_remind_slash(&options).unwrap(),
            RemindArgs::Add(NewReminder {
                time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                content: ReminderContent::Separate {
                    content: "standup".into(),
                    roles: None,
                },
            })
        );
        let bad_time = options
            .clone()
            .with("time", OptionValue::String("8h".into()));
        assert!(parse_remind_slash(&bad_time).is_err());
    }

    #[test]
    fn leading_roles_stop_at_the_first_other_word() {
        assert_eq!(
            split_leading_roles("@support, <@&2> call the user @Admins", resolve),
            (
                vec![RoleId::new(1), RoleId::new(2)],
                "call the user @Admins".to_string()
            )
        );
    }

    #[test]
    fn unknown_leading_role_ends_the_role_list() {
        assert_eq!(
            split_leading_roles("@Support @Ghost hi", resolve),
            (vec![RoleId::new(1)], "@Ghost hi".to_string())
        );
        assert_eq!(
            split_leading_roles("@Ghost hi", resolve),
            (vec![], "@Ghost hi".to_string())
        );
    }

    #[test]
    fn role_list_prefers_mentions_over_names() {
        assert_eq!(
            parse_role_list("<@&2> Support <@&2>", resolve),
            vec![RoleId::new(2)]
        );
        assert_eq!(
            parse_role_list("support, admins", resolve),
            vec![RoleId::new(1), RoleId::new(2)]
        );
    }
}
