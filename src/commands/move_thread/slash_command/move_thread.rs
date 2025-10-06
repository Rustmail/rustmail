use crate::commands::move_thread::common::{
    fetch_server_categories, find_best_match_category, move_channel_to_category_by_command_option,
};
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::get_user_id_from_channel_id;
use crate::errors::{
    CommandError, DatabaseError, DiscordError, ModmailError, ModmailResult, ThreadError,
};
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedOption,
};
use std::collections::HashMap;

pub struct MoveCommand;

#[async_trait::async_trait]
impl RegistrableCommand for MoveCommand {
    fn name(&self) -> &'static str {
        "move"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.move_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let catagory_field_desc = get_translated_message(
                &config,
                "slash_command.move_command_name_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("move").description(cmd_desc).add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "category",
                        catagory_field_desc,
                    )
                    .required(true),
                ),
            ]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        options: &[ResolvedOption<'_>],
        config: &Config,
    ) -> BoxFuture<ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let pool = match &config.db_pool {
                Some(pool) => pool,
                None => {
                    return Err(ModmailError::Database(DatabaseError::ConnectionFailed));
                }
            };

            defer_response(&ctx, &command).await?;

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

            let categories = fetch_server_categories(&ctx, &config).await;
            if categories.is_empty() {
                return Err(ModmailError::Discord(DiscordError::FailedToFetchCategories));
            }

            let target_category = find_best_match_category(&category_name, &categories);

            match target_category {
                Some((category_id, category_name)) => {
                    if let Err(e) =
                        move_channel_to_category_by_command_option(&ctx, &command, category_id)
                            .await
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
                        .build_interaction_message_followup()
                        .await;

                    command.create_followup(&ctx.http, response).await?;

                    Ok(())
                }
                None => {
                    let mut params = HashMap::new();
                    params.insert("category".to_string(), category_name);

                    Err(ModmailError::Thread(ThreadError::CategoryNotFound))
                }
            }
        })
    }
}
