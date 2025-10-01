use crate::commands::move_thread::common::{
    fetch_server_categories, find_best_match_category, move_channel_to_category_by_command_option,
};
use crate::config::Config;
use crate::db::get_user_id_from_channel_id;
use crate::errors::{
    CommandError, DatabaseError, DiscordError, ModmailError, ModmailResult, ThreadError,
};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, ResolvedOption,
};
use std::collections::HashMap;

pub fn register() -> CreateCommand {
    CreateCommand::new("move")
        .description("Move a thread in an other category")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "category",
                "The category where you want to move_thread the thread",
            )
            .required(true),
        )
}

pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    _options: &[ResolvedOption<'_>],
    config: &Config,
) -> ModmailResult<()> {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            return Err(ModmailError::Database(DatabaseError::ConnectionFailed));
        }
    };

    if !get_user_id_from_channel_id(&command.channel_id.to_string(), pool)
        .await
        .is_some()
    {
        return Err(ModmailError::Command(CommandError::NotInThread()));
    }

    let category_name = match command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "category")
    {
        Some(opt) => match &opt.value {
            serenity::all::CommandDataOptionValue::String(name) => name.trim().to_string(),
            _ => String::new(),
        },
        None => String::new(),
    };

    if category_name.is_empty() {
        return Err(ModmailError::Thread(ThreadError::CategoryNotFound));
    }

    let categories = fetch_server_categories(ctx, config).await;
    if categories.is_empty() {
        return Err(ModmailError::Discord(DiscordError::FailedToFetchCategories));
    }

    let target_category = find_best_match_category(&category_name, &categories);

    match target_category {
        Some((category_id, category_name)) => {
            if let Err(e) =
                move_channel_to_category_by_command_option(ctx, command, category_id).await
            {
                eprintln!("Failed to move channel: {}", e);
                return Err(ModmailError::Discord(DiscordError::FailedToMoveChannel));
            }

            let mut params = HashMap::new();
            params.insert("category".to_string(), category_name.to_string());
            params.insert("staff".to_string(), command.user.name.clone());

            let response = MessageBuilder::system_message(&ctx, &config)
                .translated_content(
                    "move_thread.success",
                    Some(&params),
                    Some(command.user.id),
                    command.guild_id.map(|g| g.get()),
                )
                .await
                .to_channel(command.channel_id)
                .build_interaction_message()
                .await;

            command
                .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                .await?;

            Ok(())
        }
        None => {
            let mut params = HashMap::new();
            params.insert("category".to_string(), category_name);

            Err(ModmailError::Thread(ThreadError::CategoryNotFound))
        }
    }
}
