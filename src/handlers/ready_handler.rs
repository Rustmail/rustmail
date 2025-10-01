use crate::commands;
use crate::config::Config;
use crate::features::sync_features;
use crate::modules::message_recovery::{recover_missing_messages, send_recovery_summary};
use crate::modules::scheduled_closures::hydrate_scheduled_closures;
use serenity::all::{ActivityData, GuildId};
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};
use crate::commands::id::slash_command::id;

#[derive(Clone)]
pub struct ReadyHandler {
    pub config: Config,
}

impl ReadyHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
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

        let _ = guild_id
            .set_commands(
                &ctx.http,
                vec![id::register(), commands::move_thread::register()],
            )
            .await;
    }
}
