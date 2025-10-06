use crate::commands::add_staff::slash_command::add_staff;
use crate::commands::alert::slash_command::alert;
use crate::commands::close::slash_command::close;
use crate::commands::delete::slash_command::delete;
use crate::commands::edit::slash_command::edit;
use crate::commands::force_close::slash_command::force_close;
use crate::commands::help::slash_command::help;
use crate::commands::id::slash_command::id;
use crate::commands::move_thread::slash_command::move_thread;
use crate::commands::new_thread::slash_command::new_thread;
use crate::commands::recover::slash_command::recover;
use crate::commands::remove_staff::slash_command::remove_staff;
use crate::commands::reply::slash_command::reply;
use crate::config::Config;
use crate::features::sync_features;
use crate::modules::message_recovery::{recover_missing_messages, send_recovery_summary};
use crate::modules::scheduled_closures::hydrate_scheduled_closures;
use serenity::all::{ActivityData, GuildId};
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};

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

        if let Err(e) = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    id::register(&self.config).await,
                    move_thread::register(&self.config).await,
                    new_thread::register(&self.config).await,
                    close::register(&self.config).await,
                    edit::register(&self.config).await,
                    add_staff::register(&self.config).await,
                    remove_staff::register(&self.config).await,
                    alert::register(&self.config).await,
                    force_close::register(&self.config).await,
                    reply::register(&self.config).await,
                    delete::register(&self.config).await,
                    recover::register(&self.config).await,
                    help::register(&self.config).await,
                ],
            )
            .await
        {
            eprintln!("set_commands() failed: {:?}", e);
        }
    }
}
