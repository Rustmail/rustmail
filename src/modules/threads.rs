use crate::db::operations::create_thread;
use crate::utils::format_duration_since::format_duration_since;
use crate::utils::send_to_thread::send_to_thread;
use crate::{
    config::Config,
    utils::get_member_join_date::get_member_join_date,
};
use serenity::all::{ChannelId, ComponentInteraction, Context, CreateChannel, GuildId, Message};
use serenity::builder::EditMessage;
use crate::utils::message_builder::MessageBuilder;
use crate::db::operations::get_thread_channel_by_user_id;

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
        let is_new_thread: bool;
        match create_thread(&channel, msg, pool).await {
            Ok(_) => {
                let canonical_channel_id_str = get_thread_channel_by_user_id(msg.author.id, pool).await;
                let canonical_channel_id_matches = canonical_channel_id_str
                    .as_deref()
                    .map(|id| id == channel.id.to_string())
                    .unwrap_or(false);
                is_new_thread = canonical_channel_id_matches;
            }
            Err(e) => {
                eprintln!("Error creating thread: {}", e);
                let canonical_channel_id_str = get_thread_channel_by_user_id(msg.author.id, pool).await;
                if let Some(canonical_id) = canonical_channel_id_str {
                    if canonical_id != channel.id.to_string() {
                        let _ = channel.delete(&ctx.http).await;
                    }
                }
                return;
            }
        }

        let target_channel_id = if let Some(canonical_id_str) = get_thread_channel_by_user_id(msg.author.id, pool).await {
            if canonical_id_str != channel.id.to_string() {
                let _ = channel.delete(&ctx.http).await;
                ChannelId::new(canonical_id_str.parse::<u64>().unwrap_or(channel.id.get()))
            } else {
                channel.id
            }
        } else {
            channel.id
        };

        if is_new_thread {
            let open_thread_message = format!(
                "ACCOUNT AGE **{}**, ID **{}**\nNICKNAME **{}**, JOINED **{}**",
                format_duration_since(msg.author.id.created_at()),
                msg.author.id,
                msg.author.name,
                member_join_date
            );

            let _ = MessageBuilder::system_message(ctx, config)
                .to_channel(target_channel_id)
                .content(open_thread_message)
                .send()
                .await;

            let _ = MessageBuilder::system_message(ctx, config)
                .content(&config.bot.welcome_message)
                .to_user(msg.author.id)
                .send()
                .await;

            println!("Thread created successfully");
        }
        
        if let Err(e) = send_to_thread(ctx, target_channel_id, msg, config, false).await {
            eprintln!("Failed to forward message to thread: {:?}", e);
        }
    }
}

fn parse_thread_interaction(custom_id: &str) -> Option<String> {
    let parts: Vec<&str> = custom_id.split(':').collect();
    if parts.len() >= 2 && parts[0] == "ticket" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

pub async fn handle_thread_interaction(
    ctx: &Context,
    interaction: &mut ComponentInteraction,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let parts = match parse_thread_interaction(&interaction.data.custom_id) {
        Some(parts) => parts,
        None => {
            eprintln!("Invalid custom ID format: {}", interaction.data.custom_id);
            return Ok(());
        }
    };

    if parts == "delete" {
        interaction.channel_id.delete(&ctx.http).await?;
    }
    if parts == "keep" {
        let builder = EditMessage::default()
            .components(vec![]);

        interaction.message.edit(&ctx.http, builder).await?;
    }
    Ok(())
}
