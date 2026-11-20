use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::CreateCommand;
use std::future::Future;

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

pub trait Command: Send + Sync + 'static {
    type Args: Send;

    fn name(&self) -> &'static str;

    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }

    fn surfaces(&self) -> Surfaces {
        Surfaces::Both
    }

    fn community(&self) -> bool {
        false
    }

    fn ephemeral(&self) -> bool {
        false
    }

    fn doc(&self, config: &Config) -> impl Future<Output = String> + Send;

    fn slash_definitions(
        &self,
        _config: &Config,
    ) -> impl Future<Output = Vec<CreateCommand>> + Send {
        async { Vec::new() }
    }

    fn community_definitions(
        &self,
        config: &Config,
    ) -> impl Future<Output = Vec<CreateCommand>> + Send {
        self.slash_definitions(config)
    }

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

pub trait DynCommand: Send + Sync {
    fn name(&self) -> &'static str;
    fn aliases(&self) -> &'static [&'static str];
    fn surfaces(&self) -> Surfaces;
    fn community(&self) -> bool;
    fn ephemeral(&self) -> bool;
    fn doc<'a>(&'a self, config: &'a Config) -> BoxFuture<'a, String>;
    fn slash_definitions<'a>(&'a self, config: &'a Config) -> BoxFuture<'a, Vec<CreateCommand>>;
    fn community_definitions<'a>(&'a self, config: &'a Config)
    -> BoxFuture<'a, Vec<CreateCommand>>;
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

    fn ephemeral(&self) -> bool {
        Command::ephemeral(self)
    }

    fn doc<'a>(&'a self, config: &'a Config) -> BoxFuture<'a, String> {
        Box::pin(Command::doc(self, config))
    }

    fn slash_definitions<'a>(&'a self, config: &'a Config) -> BoxFuture<'a, Vec<CreateCommand>> {
        Box::pin(Command::slash_definitions(self, config))
    }

    fn community_definitions<'a>(
        &'a self,
        config: &'a Config,
    ) -> BoxFuture<'a, Vec<CreateCommand>> {
        Box::pin(Command::community_definitions(self, config))
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
