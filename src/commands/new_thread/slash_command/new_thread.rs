use std::collections::HashMap;
use crate::config::Config;
use crate::i18n::get_translated_message;
use serenity::all::{ChannelId, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId, ResolvedOption, UserId};
use serenity::builder::CreateInteractionResponse;
use crate::commands::new_thread::common::{send_welcome_message};
use crate::db::{create_thread_for_user, get_thread_channel_by_user_id, thread_exists};
use crate::errors::{common, CommandError, DatabaseError, DiscordError, ModmailError, ModmailResult};
use crate::utils::message::message_builder::MessageBuilder;

pub async fn register(config: &Config) -> CreateCommand {
    let cmd_desc = get_translated_message(
        config,
        "slash_command.new_thread_command_description",
        None,
        None,
        None,
        None,
    )
    .await;

    let user_id_desc = get_translated_message(
        config,
        "slash_command.new_thread_user_id_argument",
        None,
        None,
        None,
        None,
    )
        .await;

    CreateCommand::new("new_thread")
        .description(cmd_desc)
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user_id", user_id_desc)
                .required(true),
        )
}

pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    _options: &[ResolvedOption<'_>],
    config: &Config,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let user_id = match command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "user_id")
    {
        Some(opt) => match &opt.value {
            CommandDataOptionValue::User(user_id) => *user_id,
            _ => return Err(ModmailError::Command(CommandError::InvalidArguments("user_id".to_string()))),
        },
        None => return Err(ModmailError::Command(CommandError::MissingArguments)),
    };

    let user = match ctx.http.get_user(user_id).await {
        Ok(user) => user,
        Err(_) => {
            return Err(ModmailError::Discord(DiscordError::UserNotFound))
        }
    };

    if user.bot {
        return Err(ModmailError::Discord(DiscordError::UserIsABot));
    }

    if thread_exists(user_id, pool).await {
        return if let Some(channel_id_str) = get_thread_channel_by_user_id(user_id, pool).await {
            Err(ModmailError::Command(CommandError::UserHasAlreadyAThreadWithLink(user.name.clone(), channel_id_str.clone())))
        } else {
            Err(ModmailError::Command(CommandError::UserHasAlreadyAThread()))
        };
    }

    let inbox_category_id = ChannelId::new(config.thread.inbox_category_id);
    let channel_name = user.name.to_lowercase().replace(" ", "-").to_string();
    let mut channel_builder = serenity::all::CreateChannel::new(&channel_name);
    channel_builder = channel_builder
        .kind(serenity::model::channel::ChannelType::Text)
        .category(inbox_category_id);

    let staff_guild_id = GuildId::new(config.bot.get_staff_guild_id());
    let guild_channel = match staff_guild_id
        .create_channel(&ctx.http, channel_builder)
        .await
    {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Failed to create channel: {}", e);
            return Err(ModmailError::Discord(DiscordError::ChannelCreationFailed));
        }
    };

    let _ = match create_thread_for_user(&guild_channel, user_id.get() as i64, &user.name, pool)
        .await
    {
        Ok(thread_id) => thread_id,
        Err(e) => {
            eprintln!("Failed to create thread in database: {}", e);
            let _ = guild_channel.delete(&ctx.http).await;
            return Err(ModmailError::Database(DatabaseError::InsertFailed(e.to_string())));
        }
    };

    send_welcome_message(ctx, &guild_channel, config, &user).await;

    let mut params = HashMap::new();
    params.insert("user".to_string(), user.name.clone());
    params.insert("channel_id".to_string(), guild_channel.to_string());
    params.insert("staff".to_string(), command.user.name.clone());

    println!("Thread created for user {} in channel {}", user.name, guild_channel.to_string());

    let response = CreateInteractionResponse::Message(
        MessageBuilder::system_message(ctx, config)
            .translated_content("new_thread.success_with_dm", Some(&params), None, None).await
            .to_channel(command.channel_id)
            .build_interaction_message()
            .await
    );

    let _ = command.create_response(&ctx.http, response).await?;

    Ok(())
}
