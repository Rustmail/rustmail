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
use tokio::sync::watch::Receiver;

pub mod add_reminder;
pub mod add_staff;
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
pub mod remove_staff;
pub mod rename;
pub mod reply;
pub mod slash_options;
pub mod snippet;
pub mod status;
pub mod take;
pub mod ticket;
pub mod version;

pub use add_reminder::*;
pub use add_staff::*;
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
pub use remove_staff::*;
pub use rename::*;
pub use reply::*;
pub use slash_options::*;
pub use snippet::*;
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

/// A command found by its slash name.
pub enum SlashEntry {
    Command(Arc<dyn DynCommand>),
    /// Not yet migrated to `Command`.
    Legacy(Arc<dyn RegistrableCommand>),
}

/// A command found by its text name or alias.
pub enum TextEntry {
    Command(Arc<dyn DynCommand>),
    /// Not yet migrated to `Command`.
    Legacy(TextCommandFunc),
}

pub struct CommandRegistry {
    commands: HashMap<&'static str, Arc<dyn DynCommand>>,
    text_names: HashMap<&'static str, Arc<dyn DynCommand>>,
    legacy_slash: HashMap<&'static str, Arc<dyn RegistrableCommand>>,
    legacy_text: HashMap<&'static str, TextCommandFunc>,
    _shutdown: Arc<Receiver<bool>>,
    _pagination: PaginationStore,
}

impl CommandRegistry {
    pub fn new(shutdown: Receiver<bool>, pagination: PaginationStore) -> Self {
        Self {
            commands: HashMap::new(),
            text_names: HashMap::new(),
            legacy_slash: HashMap::new(),
            legacy_text: HashMap::new(),
            _shutdown: Arc::new(shutdown),
            _pagination: pagination,
        }
    }

    pub fn _shutdown(&self) -> Arc<Receiver<bool>> {
        self._shutdown.clone()
    }

    /// Registers a command on the surfaces it supports, under its name and aliases.
    /// Panics if a name or alias is already taken.
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

    /// Panics if the name is already registered.
    pub fn register_legacy_command<C: RegistrableCommand + 'static>(&mut self, command: C) {
        let name = command.name();
        if self.commands.contains_key(name)
            || self.legacy_slash.insert(name, Arc::new(command)).is_some()
        {
            panic!("slash command `{name}` is registered twice");
        }
    }

    /// Registers a legacy text command under its names. Panics if a name is already taken.
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

    /// Whether the slash command is also registered in the community guild.
    pub fn is_community(&self, name: &str) -> bool {
        match self.slash(name) {
            Some(SlashEntry::Command(command)) => command.community(),
            Some(SlashEntry::Legacy(command)) => command.as_community().is_some(),
            None => false,
        }
    }

    /// Slash definitions for the staff guild and for the community guild.
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

    /// Names listed by help, sorted. Legacy text-only commands aren't listed: their text names
    /// can't be told apart from the aliases of slash commands.
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

    /// The help text of a command, found by name or text alias.
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

/// Registers every command, slash and text. Called once when the bot starts.
pub fn register_all(registry: &mut CommandRegistry) {
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

    registry.register_legacy_command(AddStaffCommand);
    registry.register_legacy_command(AlertCommand);
    registry.register_legacy_command(EditCommand);
    registry.register_legacy_command(HelpCommand);
    registry.register_legacy_command(MoveCommand);
    registry.register_legacy_command(NewThreadCommand);
    registry.register_legacy_command(RemoveStaffCommand);
    registry.register_legacy_command(AddReminderCommand);
    registry.register_legacy_command(ReminderSubscriptionCommand);
    registry.register_legacy_command(LogsCommand);
    registry.register_legacy_command(SnippetCommand);
    registry.register_legacy_command(StatusCommand);
    registry.register_legacy_command(CategoryCommand);
    registry.register_legacy_command(BaninfoCommand);

    wrap_command!(registry, ["edit", "e"], edit);
    wrap_command!(registry, "alert", alert);
    wrap_command!(registry, ["move", "mv"], move_thread);
    wrap_command!(registry, ["new_thread", "nt"], new_thread);
    wrap_command!(registry, ["addmod", "am"], add_staff);
    wrap_command!(registry, ["delmod", "dm"], remove_staff);
    wrap_command!(registry, "help", help);
    wrap_command!(registry, ["remind", "rem"], add_reminder);
    wrap_command!(registry, "logs", logs);
    wrap_command!(registry, ["snippet", "s"], snippet_command);
    wrap_command!(registry, "status", status_command);
    wrap_command!(registry, "category", category_command);
    wrap_command!(registry, ["baninfo", "bi"], baninfo);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_command_name_and_alias_is_unique() {
        let (_tx, rx) = tokio::sync::watch::channel(false);
        let mut registry = CommandRegistry::new(rx, PaginationStore::default());
        // register_* panic on a duplicate name or alias.
        register_all(&mut registry);
    }
}
