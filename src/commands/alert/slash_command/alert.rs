use crate::commands::alert::common::{
    get_thread_user_id_from_command, handle_cancel_alert_from_command,
    handle_set_alert_from_command,
};
use crate::config::Config;
use crate::errors::{CommandError, ModmailError, ModmailResult, common};
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};

pub async fn register(config: &Config) -> CreateCommand {
    let cmd_desc = get_translated_message(
        config,
        "slash_command.alert_command_description",
        None,
        None,
        None,
        None,
    )
    .await;
    let cancel_desc = get_translated_message(
        config,
        "slash_command.alert_cancel_argument",
        None,
        None,
        None,
        None,
    )
    .await;

    CreateCommand::new("alert")
        .description(cmd_desc)
        .add_option(
            CreateCommandOption::new(CommandOptionType::Boolean, "cancel", cancel_desc)
                .required(false),
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

    defer_response(&ctx, &command).await?;

    let user_id = get_thread_user_id_from_command(ctx, command, config, pool).await?;

    let is_cancel = match command.data.options.iter().find(|opt| opt.name == "cancel") {
        Some(opt) => match &opt.value {
            CommandDataOptionValue::Boolean(cancel) => *cancel,
            _ => {
                return Err(ModmailError::Command(CommandError::InvalidArguments(
                    "user_id".to_string(),
                )));
            }
        },
        None => false,
    };

    if is_cancel {
        handle_cancel_alert_from_command(ctx, command, config, user_id, pool).await
    } else {
        handle_set_alert_from_command(ctx, command, config, user_id, pool).await
    }
}
