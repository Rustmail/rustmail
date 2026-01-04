use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::types::*;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::any::Any;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub mod add_reminder;
pub mod add_staff;
pub mod alert;
pub mod anonreply;
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
pub mod reply;
pub mod snippet;
pub mod status;
pub mod take;

pub use add_reminder::*;
pub use add_staff::*;
pub use alert::*;
pub use anonreply::*;
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
pub use reply::*;
pub use snippet::*;
pub use status::*;
pub use take::*;

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

pub struct CommandRegistry {
    commands: HashMap<&'static str, Arc<dyn RegistrableCommand>>,
    _shutdown: Arc<Receiver<bool>>,
    _pagination: PaginationStore,
}

impl CommandRegistry {
    pub fn new(shutdown: Receiver<bool>, pagination: PaginationStore) -> Self {
        Self {
            commands: HashMap::new(),
            _shutdown: Arc::new(shutdown),
            _pagination: pagination,
        }
    }

    pub fn _shutdown(&self) -> Arc<Receiver<bool>> {
        self._shutdown.clone()
    }

    pub fn register_command<C: RegistrableCommand + 'static>(&mut self, command: C) {
        self.commands.insert(command.name(), Arc::new(command));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn RegistrableCommand>> {
        self.commands.get(name).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn RegistrableCommand>> {
        self.commands.values().cloned().collect()
    }
}
