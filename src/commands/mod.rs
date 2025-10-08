use crate::config::Config;
use crate::errors::ModmailResult;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

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
pub mod move_thread;
pub mod new_thread;
pub mod recover;
pub mod remove_reminder;
pub mod remove_staff;
pub mod reply;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub trait RegistrableCommand: Send + Sync {
    fn name(&self) -> &'static str;

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>>;

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        options: &[ResolvedOption<'_>],
        config: &Config,
    ) -> BoxFuture<ModmailResult<()>>;
}

pub struct CommandRegistry {
    commands: HashMap<&'static str, Arc<dyn RegistrableCommand>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
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
