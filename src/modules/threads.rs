use crate::db::operations::create_thread;
use crate::utils::build_message_from_ticket::build_message_from_ticket;
use crate::utils::format_ticket_message::Sender;
use crate::utils::send_to_thread::send_to_thread;
use crate::{
    config::Config, utils::format_account_age::format_account_age,
    utils::format_ticket_message::format_ticket_message,
};
use serenity::all::{ChannelId, Context, CreateChannel, CreateMessage, GuildId, Message};

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
        let open_thread_message = format!(
            "ACCOUNT AGE **{}**",
            format_account_age(msg.author.id.created_at())
        );
        let response = format_ticket_message(
            &ctx,
            Sender::System {
                user_id: ctx.cache.current_user().id,
                username: ctx.cache.current_user().name.clone(),
            },
            &open_thread_message,
            config,
        );
        let response = response.await;
        let thread_message = build_message_from_ticket(response, CreateMessage::new());
        let _ = channel.send_message(&ctx.http, thread_message).await;

        match create_thread(&channel, msg, pool).await {
            Ok(_) => println!("Thread created successfully"),
            Err(e) => eprintln!("Error creating thread: {}", e),
        }
        if let Err(e) = send_to_thread(ctx, channel.id, msg, config, false).await {
            eprintln!("Failed to forward message to thread: {:?}", e);
        }
    }
}
