use serenity::all::{
    Attachment, ChannelId, CommandInteraction, Message, ResolvedOption, ResolvedTarget,
    ResolvedValue, RoleId, UserId,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum OptionValue {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    User(UserId),
    Channel(ChannelId),
    Role(RoleId),
    Attachment(Box<Attachment>),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SlashTarget {
    User(UserId),
    Message(Box<Message>),
}

#[derive(Debug, Clone, Default)]
pub struct SlashOptions {
    subcommand: Vec<String>,
    values: HashMap<String, OptionValue>,
    target: Option<SlashTarget>,
}

#[allow(dead_code)]
impl SlashOptions {
    pub fn from_interaction(command: &CommandInteraction) -> Self {
        let mut options = Self::default();
        options.read(command.data.options());

        options.target = match command.data.target() {
            Some(ResolvedTarget::User(user, _)) => Some(SlashTarget::User(user.id)),
            Some(ResolvedTarget::Message(message)) => {
                Some(SlashTarget::Message(Box::new(message.clone())))
            }
            _ => None,
        };

        options
    }

    fn read(&mut self, resolved: Vec<ResolvedOption<'_>>) {
        for option in resolved {
            let value = match option.value {
                ResolvedValue::SubCommand(nested) | ResolvedValue::SubCommandGroup(nested) => {
                    self.subcommand.push(option.name.to_string());
                    self.read(nested);
                    continue;
                }
                ResolvedValue::String(v) => OptionValue::String(v.to_string()),
                ResolvedValue::Integer(v) => OptionValue::Integer(v),
                ResolvedValue::Number(v) => OptionValue::Number(v),
                ResolvedValue::Boolean(v) => OptionValue::Boolean(v),
                ResolvedValue::User(user, _) => OptionValue::User(user.id),
                ResolvedValue::Channel(channel) => OptionValue::Channel(channel.id),
                ResolvedValue::Role(role) => OptionValue::Role(role.id),
                ResolvedValue::Attachment(attachment) => {
                    OptionValue::Attachment(Box::new(attachment.clone()))
                }
                _ => continue,
            };
            self.values.insert(option.name.to_string(), value);
        }
    }

    pub fn subcommand(&self) -> Vec<&str> {
        self.subcommand.iter().map(String::as_str).collect()
    }

    pub fn with_subcommand(mut self, path: &[&str]) -> Self {
        self.subcommand = path.iter().map(|name| name.to_string()).collect();
        self
    }

    pub fn with(mut self, name: &str, value: OptionValue) -> Self {
        self.values.insert(name.to_string(), value);
        self
    }

    pub fn with_target(mut self, target: SlashTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn string(&self, name: &str) -> Option<&str> {
        match self.values.get(name) {
            Some(OptionValue::String(v)) => Some(v),
            _ => None,
        }
    }

    pub fn integer(&self, name: &str) -> Option<i64> {
        match self.values.get(name) {
            Some(OptionValue::Integer(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn number(&self, name: &str) -> Option<f64> {
        match self.values.get(name) {
            Some(OptionValue::Number(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn boolean(&self, name: &str) -> Option<bool> {
        match self.values.get(name) {
            Some(OptionValue::Boolean(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn user(&self, name: &str) -> Option<UserId> {
        match self.values.get(name) {
            Some(OptionValue::User(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn channel(&self, name: &str) -> Option<ChannelId> {
        match self.values.get(name) {
            Some(OptionValue::Channel(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn role(&self, name: &str) -> Option<RoleId> {
        match self.values.get(name) {
            Some(OptionValue::Role(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn attachment(&self, name: &str) -> Option<&Attachment> {
        match self.values.get(name) {
            Some(OptionValue::Attachment(v)) => Some(v),
            _ => None,
        }
    }

    pub fn target(&self) -> Option<&SlashTarget> {
        self.target.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_accessors_ignore_other_kinds() {
        let options = SlashOptions::default()
            .with("time", OptionValue::String("1h".into()))
            .with("silent", OptionValue::Boolean(true));

        assert_eq!(options.string("time"), Some("1h"));
        assert_eq!(options.boolean("silent"), Some(true));
        assert_eq!(options.boolean("time"), None);
        assert_eq!(options.string("missing"), None);
    }

    #[test]
    fn subcommand_path() {
        let options = SlashOptions::default()
            .with_subcommand(&["timeout", "set"])
            .with("minutes", OptionValue::Integer(5));
        assert_eq!(options.subcommand(), ["timeout", "set"]);
        assert_eq!(options.integer("minutes"), Some(5));
        assert!(SlashOptions::default().subcommand().is_empty());
    }

    #[test]
    fn target_user() {
        let options = SlashOptions::default().with_target(SlashTarget::User(UserId::new(42)));
        assert!(matches!(options.target(), Some(SlashTarget::User(id)) if id.get() == 42));
    }
}
