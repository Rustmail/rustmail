use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CommandType, Context,
    CreateCommand, CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct RemoveStaffCommand;

#[async_trait::async_trait]
impl RegistrableCommand for RemoveStaffCommand {
    fn name(&self) -> &'static str {
        "delmod"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
            get_translated_message(config, "help.remove_staff", None, None, None, None).await
        }.boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();
        let name = self.name();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.remove_staff_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            let user_id_desc = get_translated_message(
                &config,
                "slash_command.remove_staff_user_id_argument",
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
                CreateCommand::new(name).kind(CommandType::User),
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
            let pool = config
                .db_pool
                .as_ref()
                .ok_or_else(database_connection_failed)?;

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

            if thread_exists_by_channel(command.channel_id, pool).await {
                match remove_user_from_channel(&ctx, command.channel_id, user_id).await {
                    Ok(_) => {
                        let mut params = HashMap::new();
                        params.insert("user".to_string(), format!("<@{}>", user_id));

                        let _ = MessageBuilder::system_message(&ctx, &config)
                            .translated_content(
                                "add_staff.remove_success",
                                Some(&params),
                                None,
                                None,
                            )
                            .await
                            .to_channel(command.channel_id)
                            .send_interaction_followup(&command, true)
                            .await;

                        Ok(())
                    }
                    Err(..) => Err(ModmailError::Command(CommandError::InvalidFormat)),
                }
            } else {
                Err(ModmailError::Thread(ThreadError::NotAThreadChannel))
            }
        })
    }
}
