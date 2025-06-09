use crate::config::Config;
use crate::db::operations::create_thread;
use crate::utils::send_to_thread::send_to_thread;
use serenity::all::{ChannelId, Context, CreateChannel, GuildId, Message};

pub async fn create_channel(ctx: &Context, msg: &Message, config: &Config) {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            eprintln!("Database pool is not set in config.");
            return;
        }
    };
    let channel_builder = CreateChannel::new(&msg.author.name)
        .category(ChannelId::new(config.thread.inbox_category_id));
    let guild_id = GuildId::new(config.bot.guild_id);

    if let Ok(channel) = guild_id.create_channel(&ctx.http, channel_builder).await {
        let _ = channel
            .say(
                &ctx.http,
                format!("Channel created for {}", msg.author.name),
            )
            .await;

        match create_thread(&channel, msg, pool).await {
            Ok(_) => println!("Thread created successfully"),
            Err(e) => eprintln!("Error creating thread: {}", e),
        }
        if let Err(e) = send_to_thread(ctx, channel.id, msg, config, false).await {
            eprintln!("Failed to forward message to thread: {:?}", e);
        }
    }
}
