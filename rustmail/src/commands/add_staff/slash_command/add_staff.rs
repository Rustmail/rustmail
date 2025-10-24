use crate::commands::add_staff::common::add_user_to_channel;
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::thread_exists;
use crate::errors::CommandError::InvalidFormat;
use crate::errors::ThreadError::NotAThreadChannel;
use crate::errors::{common, CommandError, ModmailError, ModmailResult};
use crate::i18n::get_translated_message;
use crate::types::logs::PaginationStore;
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub struct AddStaffCommand;

impl RegistrableCommand for AddStaffCommand {
    fn name(&self) -> &'static str {
        "addmod"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
        let config = config.clone();
        let name = self.name();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.add_staff_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            let user_id_desc = get_translated_message(
                &config,
                "slash_command.add_staff_user_id_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new(name).description(cmd_desc).add_option(
                    CreateCommandOption::new(CommandOptionType::User, "user_id", user_id_desc)
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
        _shutdown: Arc<Receiver<bool>>,
        _pagination: PaginationStore,
    ) -> BoxFuture<ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let pool = config
                .db_pool
                .as_ref()
                .ok_or_else(common::database_connection_failed)?;

            defer_response(&ctx, &command).await?;

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
                None => {
                    if let Some(user_id) = command.data.target_id {
                        user_id.to_user_id()
                    } else {
                        return Err(ModmailError::Command(CommandError::InvalidArguments(
                            "user_id".to_string(),
                        )));
                    }
                }
            };

            if thread_exists(command.user.id, pool).await {
                match add_user_to_channel(&ctx, command.channel_id, user_id).await {
                    Ok(_) => {
                        let mut params = HashMap::new();
                        params.insert("user".to_string(), format!("<@{}>", user_id));

                        let response = MessageBuilder::system_message(&ctx, &config)
                            .translated_content("add_staff.add_success", Some(&params), None, None)
                            .await
                            .to_channel(command.channel_id)
                            .build_interaction_message_followup()
                            .await;

                        let _ = command.create_followup(&ctx.http, response).await;

                        Ok(())
                    }
                    Err(..) => Err(ModmailError::Command(InvalidFormat)),
                }
            } else {
                Err(ModmailError::Thread(NotAThreadChannel))
            }
        })
    }
}
