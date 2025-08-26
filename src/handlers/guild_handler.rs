use crate::config::Config;
use crate::db::close_thread;
use crate::db::operations::get_thread_by_channel_id;
use crate::db::threads::is_an_opened_ticket_channel;
use async_trait::async_trait;
use serenity::all::{Context, GuildChannel, Message};
use serenity::client::EventHandler;

pub struct GuildHandler {
    pub config: Config,
}

impl GuildHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EventHandler for GuildHandler {
    async fn channel_delete(
        &self,
        _ctx: Context,
        channel: GuildChannel,
        _messages: Option<Vec<Message>>,
    ) {
        let pool = match &self.config.db_pool {
            Some(pool) => pool,
            None => {
                eprintln!("Database pool is not set in config.");
                return;
            }
        };

        if is_an_opened_ticket_channel(channel.id, pool).await {
            let thread = match get_thread_by_channel_id(&channel.id.to_string(), pool).await {
                Some(thread) => thread,
                None => {
                    return;
                }
            };

            match close_thread(&thread.id, pool).await {
                Ok(_) => {
                    println!("Close thread successfully by deleted channel!");
                }
                Err(e) => {
                    eprintln!(
                        "Failed to close thread for deleted channel {}: {}",
                        channel.id, e
                    );
                }
            }
        }
    }
}
