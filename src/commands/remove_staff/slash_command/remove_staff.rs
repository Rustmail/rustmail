use crate::commands::remove_staff::common::remove_user_from_channel;
use crate::config::Config;
use crate::db::thread_exists;
use crate::errors::CommandError::InvalidFormat;
use crate::errors::ThreadError::NotAThreadChannel;
use crate::errors::{CommandError, ModmailError, ModmailResult, common};
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, CreateInteractionResponse, ResolvedOption,
};
use std::collections::HashMap;

pub async fn register(config: &Config) -> CreateCommand {
    let cmd_desc = get_translated_message(
        config,
        "slash_command.remove_staff_command_description",
        None,
        None,
        None,
        None,
    )
    .await;

    let user_id_desc = get_translated_message(
        config,
        "slash_command.remove_staff_user_id_argument",
        None,
        None,
        None,
        None,
    )
    .await;

    CreateCommand::new("remove_staff")
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
            _ => {
                return Err(ModmailError::Command(CommandError::InvalidArguments(
                    "user_id".to_string(),
                )));
            }
        },
        None => return Err(ModmailError::Command(CommandError::MissingArguments)),
    };

    if thread_exists(command.user.id, pool).await {
        match remove_user_from_channel(ctx, command.channel_id, user_id).await {
            Ok(_) => {
                let mut params = HashMap::new();
                params.insert("user".to_string(), format!("<@{}>", user_id));

                let response = MessageBuilder::system_message(ctx, config)
                    .translated_content("add_staff.remove_success", Some(&params), None, None)
                    .await
                    .to_channel(command.channel_id)
                    .build_interaction_message()
                    .await;

                let _ = command
                    .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                    .await;

                Ok(())
            }
            Err(..) => Err(ModmailError::Command(InvalidFormat)),
        }
    } else {
        Err(ModmailError::Thread(NotAThreadChannel))
    }
}
