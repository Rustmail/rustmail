use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MoveCommand;

#[async_trait::async_trait]
impl RegistrableCommand for MoveCommand {
    fn name(&self) -> &'static str {
        "move"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.move", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
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
                        CommandOptionType::Channel,
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
        _options: &[ResolvedOption<'_>],
        config: &Config,
        _handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
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

            let category_option = command
                .data
                .options
                .iter()
                .find(|opt| opt.name == "category")
                .ok_or(ModmailError::Command(CommandError::MissingArguments))?;

            let category_channel = match &category_option.value {
                CommandDataOptionValue::Channel(category) => category,
                _ => return Err(ModmailError::Command(CommandError::MissingArguments)),
            };

            let category_name = category_channel
                .name(&ctx.http)
                .await
                .map_err(|_| ModmailError::Command(CommandError::MissingArguments))?;

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
                    params.insert("staff".to_string(), command.user.id.to_string());

                    let _ = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "move_thread.success",
                            Some(&params),
                            Some(command.user.id),
                            command.guild_id.map(|g| g.get()),
                        )
                        .await
                        .to_channel(command.channel_id)
                        .send_interaction_followup(&command, true)
                        .await;

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
