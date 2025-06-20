use crate::config::Config;
use crate::events;
use serenity::all::{ActivityData};
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
        events::ready::handle(&ctx, ready).await;
        ctx.set_activity(Option::from(ActivityData::playing(&self.config.bot.status)));
    }
}
