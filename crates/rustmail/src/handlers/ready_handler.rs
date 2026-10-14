use crate::db::get_all_thread_status;
use crate::handlers::guild_ban_handler::backfill_tracked_members;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::features::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use serenity::all::{ActivityData, CreateCommand, GuildId};
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{Mutex, watch::Receiver};
use tokio::time::interval;

#[derive(Clone)]
pub struct ReadyHandler {
    pub config: Config,
    pub registry: Arc<CommandRegistry>,
    pub shutdown: Arc<Receiver<bool>>,
    pub bot_state: Arc<Mutex<BotState>>,
    backfill_started: Arc<AtomicBool>,
    update_checker_started: Arc<AtomicBool>,
    startup_tasks_started: Arc<AtomicBool>,
    recovery_running: Arc<AtomicBool>,
}

/// Clears an `AtomicBool` when dropped, so the flag is released even if the task panics.
struct FlagGuard(Arc<AtomicBool>);

impl Drop for FlagGuard {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
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
            backfill_started: Arc::new(AtomicBool::new(false)),
            update_checker_started: Arc::new(AtomicBool::new(false)),
            startup_tasks_started: Arc::new(AtomicBool::new(false)),
            recovery_running: Arc::new(AtomicBool::new(false)),
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

        // Discord sends READY again on every new gateway session (invalidated session,
        // reconnect...). Long-lived tasks must only be started once per bot run, otherwise
        // each reconnect adds another copy of every loop and timer.
        let first_ready = self
            .startup_tasks_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok();

        // Message recovery is useful after every reconnect, but must not overlap.
        let recovery_guard = self
            .recovery_running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
            .then(|| FlagGuard(self.recovery_running.clone()));

        tokio::spawn({
            let ctx = ctx.clone();
            let config = config.clone();

            async move {
                if let Some(_guard) = recovery_guard {
                    let recovery_results = recover_missing_messages(&ctx, &config).await;
                    send_recovery_summary(&ctx, &config, &recovery_results).await;
                }
                if first_ready {
                    sync_features(&ctx, &config).await;
                    hydrate_scheduled_closures(&ctx, &config).await;
                    hydrate_pending_category_selections(&ctx, &config).await;
                }
            }
        });

        if self
            .backfill_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            tokio::spawn({
                let ctx = ctx.clone();
                let config = config.clone();
                let mut shutdown = (*self.shutdown).clone();

                async move {
                    backfill_tracked_members(&ctx, &config, &mut shutdown).await;
                }
            });
        }

        if self.config.updates.enabled
            && self
                .update_checker_started
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
        {
            tokio::spawn({
                let ctx = ctx.clone();
                let config = config.clone();
                let shutdown = (*self.shutdown).clone();

                async move {
                    run_update_checker(ctx, config, shutdown).await;
                }
            });
        }

        if first_ready {
            println!("[startup] Starting background tasks");
            load_reminders(&ctx, &self.config, &pool.clone(), self.shutdown.clone()).await;
            update_threads_status(&ctx, &pool.clone(), (*self.shutdown).clone());
        } else {
            println!("[startup] New gateway session, background tasks already running");
        }

        let staff_guild_id = GuildId::new(self.config.bot.get_staff_guild_id());
        let community_guild_id = GuildId::new(self.config.bot.get_community_guild_id());

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

        if staff_guild_id == community_guild_id {
            if let Err(e) = staff_guild_id.set_commands(&ctx.http, guild_commands).await {
                eprintln!("set_commands() failed: {:?}", e);
            }
            return;
        }

        if let Err(e) = staff_guild_id
            .set_commands(&ctx.http, guild_commands.clone())
            .await
        {
            eprintln!("set_commands() failed: {:?}", e);
        }

        if let Err(e) = community_guild_id
            .set_commands(&ctx.http, community_commands)
            .await
        {
            eprintln!("set_commands() failed: {:?}", e);
        }
    }
}

fn update_threads_status(ctx: &Context, pool: &SqlitePool, mut shutdown: Receiver<bool>) {
    tokio::spawn({
        let ctx = ctx.clone();
        let pool = pool.clone();

        async move {
            let mut interval = interval(Duration::from_secs(60 * 10));

            interval.tick().await;

            loop {
                if *shutdown.borrow() {
                    return;
                }

                let tickets_status = get_all_thread_status(&pool).await;

                for ticket in tickets_status.iter() {
                    let ctx = ctx.clone();
                    let ticket = ticket.clone();

                    if let Err(e) = update_thread_status_ui(&ctx, &ticket).await {
                        eprintln!(
                            "Failed to update thread status for channel {}: {:?}",
                            ticket.channel_id, e
                        );
                    }

                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_millis(500)) => {}
                        _ = shutdown.changed() => return,
                    }
                }

                println!("Updated {} ticket statuses", tickets_status.len());

                tokio::select! {
                    _ = interval.tick() => {}
                    _ = shutdown.changed() => return,
                }
            }
        }
    });
}
