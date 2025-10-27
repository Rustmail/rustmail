use crate::commands::{CommandRegistry, CommunityRegistrable};
use crate::config::Config;
use crate::features::sync_features;
use crate::modules::message_recovery::{recover_missing_messages, send_recovery_summary};
use crate::modules::reminders::load_reminders;
use crate::modules::scheduled_closures::hydrate_scheduled_closures;
use serenity::all::{ActivityData, CreateCommand, GuildId};
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};
use std::sync::Arc;
use tokio::sync::watch::Receiver;

#[derive(Clone)]
pub struct ReadyHandler {
    pub config: Config,
    pub registry: Arc<CommandRegistry>,
    pub shutdown: Arc<Receiver<bool>>,
}

impl ReadyHandler {
    pub fn new(config: &Config, registry: Arc<CommandRegistry>, shutdown: Receiver<bool>) -> Self {
        Self {
            config: config.clone(),
            registry,
            shutdown: Arc::new(shutdown),
        }
    }
}

#[async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online !", ready.user.name);
        let pool = match &self.config.db_pool {
            Some(pool) => pool,
            None => {
                eprintln!("Database pool is not set in config.");
                return;
            }
        };

        ctx.set_activity(Option::from(ActivityData::playing(&self.config.bot.status)));

        tokio::spawn({
            let ctx = ctx.clone();
            let config = self.config.clone();
            async move {
                let recovery_results = recover_missing_messages(&ctx, &config).await;
                send_recovery_summary(&ctx, &config, &recovery_results).await;
                sync_features(&ctx, &config).await;
                hydrate_scheduled_closures(&ctx, &config).await;
            }
        });

        load_reminders(&ctx, &self.config, pool, self.shutdown.clone()).await;

        let guild_id = GuildId::new(self.config.bot.get_staff_guild_id());
        let guild_id2 = GuildId::new(self.config.bot.get_community_guild_id());

        let mut guild_commands: Vec<CreateCommand> = Vec::new();
        let mut community_commands: Vec<CreateCommand> = Vec::new();

        for command in self.registry.all() {
            let mut cmds = command.register(&self.config).await;
            guild_commands.append(&mut cmds);

            if let Some(commu) = command.as_community() {
                let mut commu_cmds = commu.register_community(&self.config).await;
                community_commands.append(&mut commu_cmds);
            }
        }

        if let Err(e) = guild_id
            .set_commands(&ctx.http, guild_commands.clone())
            .await
        {
            eprintln!("set_commands() failed: {:?}", e);
        }

        if let Err(e) = guild_id2.set_commands(&ctx.http, community_commands).await {
            eprintln!("set_commands() failed: {:?}", e);
        }
    }
}
