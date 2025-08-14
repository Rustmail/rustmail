use crate::db::operations::create_thread;
use crate::utils::format_duration_since::format_duration_since;
use crate::utils::send_to_thread::send_to_thread;
use crate::{
    config::Config,
    utils::get_member_join_date::get_member_join_date,
};
use serenity::all::{ChannelId, Context, CreateChannel, GuildId, Message};
use crate::utils::message_builder::MessageBuilder;

pub async fn create_channel(ctx: &Context, msg: &Message, config: &Config) {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            eprintln!("Database pool is not set in config.");
            return;
        }
    };

    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    if let Err(_) = community_guild_id.member(&ctx.http, msg.author.id).await {
        let error_msg = crate::i18n::get_translated_message(
            config,
            "server.not_in_community",
            None,
            Some(msg.author.id),
            Some(community_guild_id.get()),
            None
        ).await;
        
        let error = crate::errors::common::validation_failed(&error_msg);
        if let Some(error_handler) = &config.error_handler {
            let _ = error_handler.reply_with_error(ctx, msg, &error).await;
        }
        return;
    }

    let channel_builder = CreateChannel::new(&msg.author.name)
        .category(ChannelId::new(config.thread.inbox_category_id));
    
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let member_join_date = get_member_join_date(ctx, msg, community_guild_id).await.unwrap_or_else(|| "Unknown".to_string());

    let staff_guild_id = GuildId::new(config.bot.get_staff_guild_id());
    if let Ok(channel) = staff_guild_id.create_channel(&ctx.http, channel_builder).await {
        let open_thread_message = format!(
            "ACCOUNT AGE **{}**, ID **{}**\nNICKNAME **{}**, JOINED **{}**",
            format_duration_since(msg.author.id.created_at()),
            msg.author.id,
            msg.author.name,
            member_join_date
        );

        let _ = MessageBuilder::system_message(ctx, config)
            .to_channel(channel.id)
            .content(open_thread_message)
            .send()
            .await;

        match create_thread(&channel, msg, pool).await {
            Ok(_) => {
                let _ = MessageBuilder::system_message(ctx, config)
                    .content(&config.bot.welcome_message)
                    .to_user(msg.author.id)
                    .send()
                    .await;
                println!("Thread created successfully");
            }
            Err(e) => {
                eprintln!("Error creating thread: {}", e);
                return;
            }
        }
        
        if let Err(e) = send_to_thread(ctx, channel.id, msg, config, false).await {
            eprintln!("Failed to forward message to thread: {:?}", e);
        }
    }
}
