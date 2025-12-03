use crate::db::get_all_thread_status;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::features::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use serenity::all::{ActivityData, CreateCommand, GuildId};
use serenity::futures::future::join_all;
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{watch::Receiver, Mutex};
use tokio::time::interval;

#[derive(Clone)]
pub struct ReadyHandler {
    pub config: Config,
    pub registry: Arc<CommandRegistry>,
    pub shutdown: Arc<Receiver<bool>>,
    pub bot_state: Arc<Mutex<BotState>>,
}

impl ReadyHandler {
    pub fn new(
        config: &Config,
        registry: Arc<CommandRegistry>,
        shutdown: Receiver<bool>,
        bot_state: Arc<Mutex<BotState>>,
    ) -> Self {
        Self {
            config: config.clone(),
            registry,
            shutdown: Arc::new(shutdown),
            bot_state,
        }
    }
}

#[async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online !", ready.user.name);

        {
            let state = self.bot_state.lock().await;
            let mut ctx_lock = state.bot_context.write().await;
            *ctx_lock = Some(ctx.clone());
        }

        let pool = match &self.config.db_pool {
            Some(pool) => pool,
            None => {
                eprintln!("Database pool is not set in config.");
                return;
            }
        };

        let config = self.config.clone();

        ctx.set_activity(Option::from(ActivityData::playing(&self.config.bot.status)));

        tokio::spawn({
            let ctx = ctx.clone();
            let config = config.clone();

            async move {
                let recovery_results = recover_missing_messages(&ctx, &config).await;
                send_recovery_summary(&ctx, &config, &recovery_results).await;
                sync_features(&ctx, &config).await;
                hydrate_scheduled_closures(&ctx, &config).await;
            }
        });

        load_reminders(&ctx, &self.config, &pool.clone(), self.shutdown.clone()).await;

        update_threads_status(&ctx, &pool.clone());

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

fn update_threads_status(ctx: &Context, pool: &SqlitePool) {
    tokio::spawn({
        let ctx = ctx.clone();
        let pool = pool.clone();

        async move {
            let mut interval = interval(Duration::from_secs(60 * 10));

            interval.tick().await;

            loop {
                let tickets_status = get_all_thread_status(&pool).await;

                let mut handles = Vec::new();
                for ticket in tickets_status.iter() {
                    let ctx = ctx.clone();
                    let ticket = ticket.clone();
                    let handle = tokio::spawn(async move {
                        if let Err(e) = update_thread_status_ui(&ctx, &ticket).await {
                            eprintln!(
                                "Failed to update thread status for channel {}: {:?}",
                                ticket.channel_id, e
                            );
                        }
                    });
                    handles.push(handle);
                }

                join_all(handles).await;

                println!("Updated {} ticket statuses", tickets_status.len());

                interval.tick().await;
            }
        }
    });
}
