use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::threads::is_a_ticket_channel;
use crate::db::get_thread_by_channel_id;
use crate::errors::common::{database_connection_failed, thread_not_found};
use crate::errors::ThreadError::NotAThreadChannel;
use crate::errors::{CommandError, ModmailError, ModmailResult};
use crate::handlers::guild_interaction_handler::InteractionHandler;
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateCommand, EditChannel, ResolvedOption,
};
use serenity::FutureExt;
use std::sync::Arc;

pub struct TakeCommand;

#[async_trait::async_trait]
impl RegistrableCommand for TakeCommand {
    fn name(&self) -> &'static str {
        "take"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.take", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.help_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new(self.name()).description(cmd_desc)]
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
            let db_pool = config
                .db_pool
                .as_ref()
                .ok_or_else(database_connection_failed)?;

            defer_response(&ctx, &command).await?;

            if is_a_ticket_channel(command.channel_id, &db_pool).await {
                let thread = match get_thread_by_channel_id(
                    &command.channel_id.to_string(),
                    db_pool,
                )
                .await
                {
                    Some(thread) => thread,
                    None => return Err(thread_not_found()),
                };

                let parse_thread_id = thread.channel_id.parse::<u64>().unwrap_or(0);

                let thread_id = ChannelId::new(parse_thread_id);

                let thread_name = thread_id
                    .name(&ctx)
                    .await
                    .unwrap_or_else(|_| "Unknown".to_string());

                if thread_name == format!("🔵-{}", command.user.name) {
                    return Err(ModmailError::Command(CommandError::TicketAlreadyTaken));
                }

                let _ = thread_id
                    .edit(
                        &ctx.http,
                        EditChannel::new().name(format!("🔵-{}", command.user.name)),
                    )
                    .await;

                let mut params = std::collections::HashMap::new();
                params.insert("staff".to_string(), format!("<@{}>", command.user.id));

                let response = MessageBuilder::system_message(&ctx, &config)
                    .translated_content("take.confirmation", Some(&params), None, None)
                    .await
                    .to_channel(command.channel_id)
                    .build_interaction_message_followup()
                    .await;

                let _ = command.create_followup(ctx.clone(), response).await;

                Ok(())
            } else {
                Err(ModmailError::Thread(NotAThreadChannel))
            }
        })
    }
}
