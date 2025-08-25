use crate::config::Config;
use crate::db::{get_thread_channel_by_user_id, get_user_id_from_channel_id};
use serenity::all::{ChannelId, Context, EventHandler, TypingStartEvent, UserId};
use serenity::async_trait;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;

#[derive(Clone)]
pub struct TypingProxyHandler {
    pub config: Config,
}

impl TypingProxyHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

async fn handle_user_typing(ctx: &Context, event: &TypingStartEvent, pool: &SqlitePool) {
    if let Some(channel_id_str) = get_thread_channel_by_user_id(event.user_id, pool).await
        && let Ok(channel_id_num) = channel_id_str.parse::<u64>() {
            let channel_id = ChannelId::new(channel_id_num);
            let typing = channel_id.start_typing(&ctx.http);
            spawn(async move {
                sleep(Duration::from_secs(5)).await;
                typing.stop();
            });
        }
}

async fn handle_staff_typing(ctx: &Context, event: &TypingStartEvent, pool: &SqlitePool) {
    if let Some(user_id_val) =
        get_user_id_from_channel_id(&event.channel_id.to_string(), pool).await
    {
        let user_id = UserId::new(user_id_val as u64);
        if let Ok(dm_channel) = user_id.create_dm_channel(&ctx.http).await {
            let typing = dm_channel.start_typing(&ctx.http);
            spawn(async move {
                sleep(Duration::from_secs(5)).await;
                typing.stop();
            });
        }
    }
}

#[async_trait]
impl EventHandler for TypingProxyHandler {
    async fn typing_start(&self, ctx: Context, event: TypingStartEvent) {
        if event.user_id == ctx.cache.current_user().id {
            return;
        }
        let pool = match &self.config.db_pool {
            Some(pool) => pool,
            None => {
                eprintln!("Database pool is not set in config.");
                return;
            }
        };
        if self.config.bot.typing_proxy_from_user && event.guild_id.is_none() {
            handle_user_typing(&ctx, &event, pool).await;
        }
        if self.config.bot.typing_proxy_from_staff && event.guild_id.is_some() {
            handle_staff_typing(&ctx, &event, pool).await;
        }
    }
}
