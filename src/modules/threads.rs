use crate::db::operations::create_thread;
use crate::db::thread_exists;
use crate::utils::build_message_from_ticket::build_message_from_ticket;
use crate::utils::format_duration_since::format_duration_since;
use crate::utils::format_ticket_message::Sender;
use crate::utils::send_to_thread::send_to_thread;
use crate::{
    config::Config, utils::format_ticket_message::format_ticket_message,
    utils::get_member_join_date::get_member_join_date,
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
    let member_join_date = match get_member_join_date(ctx, msg, community_guild_id).await {
        Some(date) => date,
        None => "Unknown".to_string(),
    };

    let staff_guild_id = GuildId::new(config.bot.get_staff_guild_id());
    if let Ok(channel) = staff_guild_id.create_channel(&ctx.http, channel_builder).await {
        let open_thread_message = format!(
            "ACCOUNT AGE **{}**, ID **{}**\nNICKNAME **{}**, JOINED **{}**",
            format_duration_since(msg.author.id.created_at()),
            msg.author.id,
            msg.author.name,
            member_join_date
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
            Ok(_) => {
                let response = format_ticket_message(
                    &ctx,
                    Sender::System {
                        user_id: ctx.cache.current_user().id,
                        username: ctx.cache.current_user().name.clone(),
                    },
                    &config.bot.welcome_message,
                    config,
                );
                let response = response.await;
                let dm_message = build_message_from_ticket(response, CreateMessage::new());
                let _ = msg.channel_id.send_message(&ctx.http, dm_message).await;
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
