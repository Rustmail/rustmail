use crate::prelude::config::*;
use crate::prelude::types::*;
use serenity::all::CreateCommand;
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

pub struct CommandCatalog {
    commands: HashMap<&'static str, Arc<dyn DynCommand>>,
    text_names: HashMap<&'static str, Arc<dyn DynCommand>>,
}

impl CommandCatalog {
    pub fn names(&self) -> Vec<&'static str> {
        let mut names: Vec<_> = self.commands.keys().copied().collect();
        names.sort_unstable();
        names
    }

    pub async fn doc(&self, name: &str, config: &Config) -> Option<String> {
        let command = self
            .commands
            .get(name)
            .or_else(|| self.text_names.get(name))?;
        Some(command.doc(config).await)
    }
}

pub struct CommandRegistry {
    commands: HashMap<&'static str, Arc<dyn DynCommand>>,
    text_names: HashMap<&'static str, Arc<dyn DynCommand>>,
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
        }
    }

    pub fn register<C: Command>(&mut self, command: C) {
        let command: Arc<dyn DynCommand> = Arc::new(command);
        let name = command.name();

        if self.commands.contains_key(name) {
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

    fn claim_text_name(&self, name: &str) {
        if self.text_names.contains_key(name) {
            panic!("text command name `{name}` is registered twice");
        }
    }

    pub fn slash(&self, name: &str) -> Option<Arc<dyn DynCommand>> {
        self.commands
            .get(name)
            .filter(|command| command.surfaces().slash())
            .cloned()
    }

    pub fn text(&self, name: &str) -> Option<Arc<dyn DynCommand>> {
        self.text_names.get(name).cloned()
    }

    pub fn is_community(&self, name: &str) -> bool {
        self.slash(name).is_some_and(|command| command.community())
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
            staff.extend(command.slash_definitions(config).await);
            if command.community() {
                community.extend(command.community_definitions(config).await);
            }
        }

        (staff, community)
    }

    pub fn catalog(&self) -> CommandCatalog {
        CommandCatalog {
            commands: self.commands.clone(),
            text_names: self.text_names.clone(),
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
    registry.register(NewThreadCommand);
    registry.register(AlertCommand);
    registry.register(SnippetCommand);
    registry.register(CategoryCommand);
    registry.register(StatusCommand {
        maintenance_mode: services.maintenance_mode.clone(),
    });

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
