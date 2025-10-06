use crate::commands::{CommandRegistry, RegistrableCommand};
use crate::config::Config;
use crate::features::sync_features;
use crate::modules::message_recovery::{recover_missing_messages, send_recovery_summary};
use crate::modules::scheduled_closures::hydrate_scheduled_closures;
use serenity::all::{ActivityData, CreateCommand, GuildId};
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ReadyHandler {
    pub config: Config,
    pub registry: Arc<CommandRegistry>,
}

impl ReadyHandler {
    pub fn new(config: &Config, registry: Arc<CommandRegistry>) -> Self {
        Self {
            config: config.clone(),
            registry,
        }
    }
}

#[async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online !", ready.user.name);
        ctx.set_activity(Option::from(ActivityData::playing(&self.config.bot.status)));

        let config = self.config.clone();
        tokio::spawn({
            let ctx = ctx.clone();
            async move {
                let recovery_results = recover_missing_messages(&ctx, &config).await;
                send_recovery_summary(&ctx, &config, &recovery_results).await;
                sync_features(&ctx, &config).await;
                hydrate_scheduled_closures(&ctx, &config).await;
            }
        });

        let guild_id = GuildId::new(self.config.bot.get_staff_guild_id());

        let mut commands: Vec<CreateCommand> = Vec::new();

        for command in self.registry.all() {
            let mut cmds = command.register(&self.config).await;
            commands.append(&mut cmds);
        }

        if let Err(e) = guild_id.set_commands(&ctx.http, commands).await {
            eprintln!("set_commands() failed: {:?}", e);
        }
    }
}
