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
pub mod anonreply;
pub mod baninfo;
pub mod category;
pub mod close;
pub mod delete;
pub mod edit;
pub mod force_close;
pub mod help;
pub mod id;
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
pub mod snippet;
pub mod status;
pub mod take;
pub mod version;

pub use add_reminder::*;
pub use add_staff::*;
pub use alert::*;
pub use anonreply::*;
pub use baninfo::*;
pub use category::*;
pub use close::*;
pub use delete::*;
pub use edit::*;
pub use force_close::*;
pub use help::*;
pub use id::*;
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
pub use snippet::*;
pub use status::*;
pub use take::*;
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

pub struct CommandRegistry {
    commands: HashMap<&'static str, Arc<dyn RegistrableCommand>>,
    text_commands: HashMap<&'static str, TextCommandFunc>,
    text_names: Vec<&'static str>,
    _shutdown: Arc<Receiver<bool>>,
    _pagination: PaginationStore,
}

impl CommandRegistry {
    pub fn new(shutdown: Receiver<bool>, pagination: PaginationStore) -> Self {
        Self {
            commands: HashMap::new(),
            text_commands: HashMap::new(),
            text_names: Vec::new(),
            _shutdown: Arc::new(shutdown),
            _pagination: pagination,
        }
    }

    pub fn _shutdown(&self) -> Arc<Receiver<bool>> {
        self._shutdown.clone()
    }

    /// Panics if the name is already registered.
    pub fn register_command<C: RegistrableCommand + 'static>(&mut self, command: C) {
        let name = command.name();
        if self.commands.insert(name, Arc::new(command)).is_some() {
            panic!("slash command `{name}` is registered twice");
        }
    }

    /// Registers a text command under its names; the first one is the name shown in help.
    /// Panics if a name or alias is already registered.
    pub fn register_text_command(&mut self, names: &[&'static str], func: TextCommandFunc) {
        self.text_names.push(names[0]);
        for &name in names {
            if self.text_commands.insert(name, func.clone()).is_some() {
                panic!("text command name `{name}` is registered twice");
            }
        }
    }

    pub fn text_command(&self, name: &str) -> Option<TextCommandFunc> {
        self.text_commands.get(name).cloned()
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn RegistrableCommand>> {
        self.commands.get(name).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn RegistrableCommand>> {
        self.commands.values().cloned().collect()
    }
}

/// Registers every command, slash and text. Called once when the bot starts.
pub fn register_all(registry: &mut CommandRegistry) {
    registry.register_command(AddStaffCommand);
    registry.register_command(AlertCommand);
    registry.register_command(CloseCommand);
    registry.register_command(DeleteCommand);
    registry.register_command(EditCommand);
    registry.register_command(ForceCloseCommand);
    registry.register_command(HelpCommand);
    registry.register_command(IdCommand);
    registry.register_command(MoveCommand);
    registry.register_command(NewThreadCommand);
    registry.register_command(RecoverCommand);
    registry.register_command(RemoveStaffCommand);
    registry.register_command(ReplyCommand);
    registry.register_command(AddReminderCommand);
    registry.register_command(RemoveReminderCommand);
    registry.register_command(ReminderSubscriptionCommand);
    registry.register_command(LogsCommand);
    registry.register_command(TakeCommand);
    registry.register_command(ReleaseCommand);
    registry.register_command(PingCommand);
    registry.register_command(SnippetCommand);
    registry.register_command(StatusCommand);
    registry.register_command(CategoryCommand);
    registry.register_command(RenameCommand);
    registry.register_command(BaninfoCommand);
    registry.register_command(VersionCommand);

    wrap_command!(registry, ["reply", "r"], reply);
    wrap_command!(registry, ["edit", "e"], edit);
    wrap_command!(registry, ["close", "c"], close);
    wrap_command!(registry, "recover", recover);
    wrap_command!(registry, "alert", alert);
    wrap_command!(registry, ["move", "mv"], move_thread);
    wrap_command!(registry, ["new_thread", "nt"], new_thread);
    wrap_command!(registry, "delete", delete);
    wrap_command!(registry, ["anonreply", "ar"], anonreply);
    wrap_command!(registry, ["force_close", "fc"], force_close);
    wrap_command!(registry, ["addmod", "am"], add_staff);
    wrap_command!(registry, ["delmod", "dm"], remove_staff);
    wrap_command!(registry, "id", id);
    wrap_command!(registry, "help", help);
    wrap_command!(registry, ["remind", "rem"], add_reminder);
    wrap_command!(registry, ["unremind", "urem"], remove_reminder);
    wrap_command!(registry, "logs", logs);
    wrap_command!(registry, "take", take);
    wrap_command!(registry, "release", release);
    wrap_command!(registry, "ping", ping);
    wrap_command!(registry, ["snippet", "s"], snippet_command);
    wrap_command!(registry, "status", status_command);
    wrap_command!(registry, "category", category_command);
    wrap_command!(registry, ["rename", "rn"], rename_ticket);
    wrap_command!(registry, ["baninfo", "bi"], baninfo);
    wrap_command!(registry, ["version", "v"], version);
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
