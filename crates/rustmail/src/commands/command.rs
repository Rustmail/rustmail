use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::CreateCommand;
use std::future::Future;

/// Where a command can be run from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surfaces {
    Both,
    SlashOnly,
    TextOnly,
}

impl Surfaces {
    pub fn slash(self) -> bool {
        self != Surfaces::TextOnly
    }

    pub fn text(self) -> bool {
        self != Surfaces::SlashOnly
    }
}

/// One action staff can run, written once for both surfaces.
///
/// Each surface only parses its input into `Args`; `run` holds the behaviour. The dispatchers
/// own everything around it: the staff and maintenance checks, deferring slash commands (with the
/// ephemeral rule), and answering errors returned by `parse_*` or `run`. So a command never
/// defers, and reports problems by returning an error rather than sending a message itself.
pub trait Command: Send + Sync + 'static {
    type Args: Send;

    /// The slash command name, also the main text name.
    fn name(&self) -> &'static str;

    /// Extra text names.
    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }

    fn surfaces(&self) -> Surfaces {
        Surfaces::Both
    }

    /// Also registered in the community guild (dual mode). Still staff-only.
    fn community(&self) -> bool {
        false
    }

    fn doc(&self, config: &Config) -> impl Future<Output = String> + Send;

    /// The slash and context menu definitions. Unused for `Surfaces::TextOnly`.
    fn slash_definitions(
        &self,
        _config: &Config,
    ) -> impl Future<Output = Vec<CreateCommand>> + Send {
        async { Vec::new() }
    }

    /// Parses what follows the command name in a text command.
    fn parse_text(&self, _args: &str) -> ModmailResult<Self::Args> {
        Err(invalid_command())
    }

    fn parse_slash(&self, _options: &SlashOptions) -> ModmailResult<Self::Args> {
        Err(invalid_command())
    }

    fn run(
        &self,
        args: Self::Args,
        invocation: &Invocation,
        config: &Config,
    ) -> impl Future<Output = ModmailResult<()>> + Send;
}

/// Object-safe form of `Command`, stored by the registry.
pub trait DynCommand: Send + Sync {
    fn name(&self) -> &'static str;
    fn aliases(&self) -> &'static [&'static str];
    fn surfaces(&self) -> Surfaces;
    fn community(&self) -> bool;
    fn doc<'a>(&'a self, config: &'a Config) -> BoxFuture<'a, String>;
    fn slash_definitions<'a>(&'a self, config: &'a Config) -> BoxFuture<'a, Vec<CreateCommand>>;
    fn run_text<'a>(
        &'a self,
        args: &'a str,
        invocation: &'a Invocation,
        config: &'a Config,
    ) -> BoxFuture<'a, ModmailResult<()>>;
    fn run_slash<'a>(
        &'a self,
        options: &'a SlashOptions,
        invocation: &'a Invocation,
        config: &'a Config,
    ) -> BoxFuture<'a, ModmailResult<()>>;
}

impl<C: Command> DynCommand for C {
    fn name(&self) -> &'static str {
        Command::name(self)
    }

    fn aliases(&self) -> &'static [&'static str] {
        Command::aliases(self)
    }

    fn surfaces(&self) -> Surfaces {
        Command::surfaces(self)
    }

    fn community(&self) -> bool {
        Command::community(self)
    }

    fn doc<'a>(&'a self, config: &'a Config) -> BoxFuture<'a, String> {
        Box::pin(Command::doc(self, config))
    }

    fn slash_definitions<'a>(&'a self, config: &'a Config) -> BoxFuture<'a, Vec<CreateCommand>> {
        Box::pin(Command::slash_definitions(self, config))
    }

    fn run_text<'a>(
        &'a self,
        args: &'a str,
        invocation: &'a Invocation,
        config: &'a Config,
    ) -> BoxFuture<'a, ModmailResult<()>> {
        Box::pin(async move {
            let args = self.parse_text(args)?;
            self.run(args, invocation, config).await
        })
    }

    fn run_slash<'a>(
        &'a self,
        options: &'a SlashOptions,
        invocation: &'a Invocation,
        config: &'a Config,
    ) -> BoxFuture<'a, ModmailResult<()>> {
        Box::pin(async move {
            let args = self.parse_slash(options)?;
            self.run(args, invocation, config).await
        })
    }
}
