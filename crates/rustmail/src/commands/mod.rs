use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::types::*;
use crate::wrap_command;
use serenity::all::{CommandInteraction, Context, CreateCommand, Message, ResolvedOption};
use std::any::Any;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::watch::Receiver;

pub mod add_reminder;
pub mod alert;
pub mod baninfo;
pub mod category;
pub mod close;
pub mod command;
pub mod delete;
pub mod edit;
pub mod force_close;
pub mod help;
pub mod id;
pub mod invocation;
pub mod logs;
pub mod move_thread;
pub mod new_thread;
pub mod ping;
pub mod recover;
pub mod release;
pub mod reminder_subscription;
pub mod remove_reminder;
pub mod rename;
pub mod reply;
pub mod slash_options;
pub mod snippet;
pub mod staff;
pub mod status;
pub mod take;
pub mod ticket;
pub mod version;

pub use add_reminder::*;
pub use alert::*;
pub use baninfo::*;
pub use category::*;
pub use close::*;
pub use command::*;
pub use delete::*;
pub use edit::*;
pub use force_close::*;
pub use help::*;
pub use id::*;
pub use invocation::*;
pub use logs::*;
pub use move_thread::*;
pub use new_thread::*;
pub use ping::*;
pub use recover::*;
pub use release::*;
pub use reminder_subscription::*;
pub use remove_reminder::*;
pub use rename::*;
pub use reply::*;
pub use slash_options::*;
pub use snippet::*;
pub use staff::*;
pub use status::*;
pub use take::*;
pub use ticket::*;
pub use version::*;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait RegistrableCommand: Any + Send + Sync {
    fn as_community(&self) -> Option<&dyn CommunityRegistrable> {
        None
    }

    fn name(&self) -> &'static str;

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String>;

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>>;

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        options: &[ResolvedOption<'_>],
        config: &Config,
        handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>>;
}

pub trait CommunityRegistrable: RegistrableCommand {
    fn register_community(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>>;
}

pub type TextCommandFunc = Arc<
    dyn for<'a> Fn(
            Context,
            Message,
            &'a Config,
            Arc<GuildMessagesHandler>,
        ) -> BoxFuture<'a, ModmailResult<()>>
        + Send
        + Sync
        + 'static,
>;

pub enum SlashEntry {
    Command(Arc<dyn DynCommand>),
    Legacy(Arc<dyn RegistrableCommand>),
}

pub enum TextEntry {
    Command(Arc<dyn DynCommand>),
    Legacy(TextCommandFunc),
}

pub struct CommandCatalog {
    commands: HashMap<&'static str, Arc<dyn DynCommand>>,
    text_names: HashMap<&'static str, Arc<dyn DynCommand>>,
    legacy_slash: HashMap<&'static str, Arc<dyn RegistrableCommand>>,
}

impl CommandCatalog {
    pub fn names(&self) -> Vec<&'static str> {
        let mut names: Vec<_> = self
            .commands
            .keys()
            .chain(self.legacy_slash.keys())
            .copied()
            .collect();
        names.sort_unstable();
        names
    }

    pub async fn doc(&self, name: &str, config: &Config) -> Option<String> {
        if let Some(command) = self
            .commands
            .get(name)
            .or_else(|| self.text_names.get(name))
        {
            return Some(command.doc(config).await);
        }
        match self.legacy_slash.get(name) {
            Some(command) => Some(command.doc(config).await),
            None => None,
        }
    }
}

pub struct CommandRegistry {
    commands: HashMap<&'static str, Arc<dyn DynCommand>>,
    text_names: HashMap<&'static str, Arc<dyn DynCommand>>,
    legacy_slash: HashMap<&'static str, Arc<dyn RegistrableCommand>>,
    legacy_text: HashMap<&'static str, TextCommandFunc>,
}

#[derive(Clone)]
pub struct CommandServices {
    pub shutdown: Arc<Receiver<bool>>,
    pub pagination: PaginationStore,
    pub maintenance_mode: Arc<AtomicBool>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            text_names: HashMap::new(),
            legacy_slash: HashMap::new(),
            legacy_text: HashMap::new(),
        }
    }

    pub fn register<C: Command>(&mut self, command: C) {
        let command: Arc<dyn DynCommand> = Arc::new(command);
        let name = command.name();

        if self.commands.contains_key(name) || self.legacy_slash.contains_key(name) {
            panic!("command `{name}` is registered twice");
        }
        self.commands.insert(name, command.clone());

        if command.surfaces().text() {
            for &text_name in std::iter::once(&name).chain(command.aliases()) {
                self.claim_text_name(text_name);
                self.text_names.insert(text_name, command.clone());
            }
        }
    }

    pub fn register_legacy_command<C: RegistrableCommand + 'static>(&mut self, command: C) {
        let name = command.name();
        if self.commands.contains_key(name)
            || self.legacy_slash.insert(name, Arc::new(command)).is_some()
        {
            panic!("slash command `{name}` is registered twice");
        }
    }

    pub fn register_legacy_text_command(&mut self, names: &[&'static str], func: TextCommandFunc) {
        for &name in names {
            self.claim_text_name(name);
            self.legacy_text.insert(name, func.clone());
        }
    }

    fn claim_text_name(&self, name: &str) {
        if self.text_names.contains_key(name) || self.legacy_text.contains_key(name) {
            panic!("text command name `{name}` is registered twice");
        }
    }

    pub fn slash(&self, name: &str) -> Option<SlashEntry> {
        if let Some(command) = self.commands.get(name)
            && command.surfaces().slash()
        {
            return Some(SlashEntry::Command(command.clone()));
        }
        self.legacy_slash.get(name).cloned().map(SlashEntry::Legacy)
    }

    pub fn text(&self, name: &str) -> Option<TextEntry> {
        if let Some(command) = self.text_names.get(name) {
            return Some(TextEntry::Command(command.clone()));
        }
        self.legacy_text.get(name).cloned().map(TextEntry::Legacy)
    }

    pub fn is_community(&self, name: &str) -> bool {
        match self.slash(name) {
            Some(SlashEntry::Command(command)) => command.community(),
            Some(SlashEntry::Legacy(command)) => command.as_community().is_some(),
            None => false,
        }
    }

    pub async fn slash_definitions(
        &self,
        config: &Config,
    ) -> (Vec<CreateCommand>, Vec<CreateCommand>) {
        let mut staff = Vec::new();
        let mut community = Vec::new();

        for command in self.commands.values() {
            if !command.surfaces().slash() {
                continue;
            }
            let definitions = command.slash_definitions(config).await;
            if command.community() {
                community.extend(definitions.iter().cloned());
            }
            staff.extend(definitions);
        }

        for command in self.legacy_slash.values() {
            staff.append(&mut command.register(config).await);
            if let Some(commu) = command.as_community() {
                community.append(&mut commu.register_community(config).await);
            }
        }

        (staff, community)
    }

    pub fn catalog(&self) -> CommandCatalog {
        CommandCatalog {
            commands: self.commands.clone(),
            text_names: self.text_names.clone(),
            legacy_slash: self.legacy_slash.clone(),
        }
    }
}

pub fn register_all(registry: &mut CommandRegistry, services: &CommandServices) {
    registry.register(CloseCommand);
    registry.register(RemoveReminderCommand);
    registry.register(ReplyCommand);
    registry.register(AnonReplyCommand);
    registry.register(VersionCommand);
    registry.register(IdCommand);
    registry.register(PingCommand);
    registry.register(TakeCommand);
    registry.register(ReleaseCommand);
    registry.register(RenameCommand);
    registry.register(RecoverCommand);
    registry.register(ForceCloseCommand);
    registry.register(DeleteCommand);
    registry.register(LogsCommand {
        pagination: services.pagination.clone(),
    });
    registry.register(BaninfoCommand);
    registry.register(AddStaffCommand);
    registry.register(RemoveStaffCommand);
    registry.register(ReminderSubscriptionCommand);
    registry.register(AddReminderCommand {
        shutdown: services.shutdown.clone(),
    });
    registry.register(MoveCommand);
    registry.register(EditCommand);
    registry.register(StatusCommand {
        maintenance_mode: services.maintenance_mode.clone(),
    });

    registry.register_legacy_command(AlertCommand);
    registry.register_legacy_command(NewThreadCommand);
    registry.register_legacy_command(SnippetCommand);
    registry.register_legacy_command(CategoryCommand);

    wrap_command!(registry, "alert", alert);
    wrap_command!(registry, ["new_thread", "nt"], new_thread);
    wrap_command!(registry, ["snippet", "s"], snippet_command);
    wrap_command!(registry, "category", category_command);

    let catalog = registry.catalog();
    registry.register(HelpCommand { catalog });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_command_name_and_alias_is_unique() {
        let (_tx, rx) = tokio::sync::watch::channel(false);
        let services = CommandServices {
            shutdown: Arc::new(rx),
            pagination: PaginationStore::default(),
            maintenance_mode: Arc::default(),
        };
        let mut registry = CommandRegistry::new();
        register_all(&mut registry, &services);
    }
}
