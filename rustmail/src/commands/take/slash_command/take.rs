use crate::modules::update_thread_status_ui;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{ChannelId, CommandInteraction, Context, CreateCommand, ResolvedOption};
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

                tokio::spawn({
                    let config = config.clone();
                    let db_pool = db_pool.clone();

                    async move {
                        let mut ticket_status = match get_thread_status(&thread.id, &db_pool).await
                        {
                            Some(status) => status,
                            None => {
                                return;
                            }
                        };
                        ticket_status.taken_by = Some(command.user.id.to_string());
                        let _ = update_thread_status_db(&thread.id, &ticket_status, &db_pool).await;

                        tokio::spawn({
                            let ctx = ctx.clone();
                            async move {
                                let _ = update_thread_status_ui(&ctx, &ticket_status).await;
                            }
                        });

                        let mut params = std::collections::HashMap::new();
                        params.insert("staff".to_string(), format!("<@{}>", command.user.id));

                        let response = MessageBuilder::system_message(&ctx, &config)
                            .translated_content("take.confirmation", Some(&params), None, None)
                            .await
                            .to_channel(command.channel_id)
                            .build_interaction_message_followup()
                            .await;

                        let _ = command.create_followup(ctx.clone(), response).await;
                    }
                });

                Ok(())
            } else {
                Err(ModmailError::Thread(ThreadError::NotAThreadChannel))
            }
        })
    }
}
