use crate::commands::force_close::common::delete_channel;
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::threads::{is_a_ticket_channel, is_orphaned_thread_channel};
use crate::errors::DatabaseError::QueryFailed;
use crate::errors::ThreadError::{NotAThreadChannel, UserStillInServer};
use crate::errors::{common, ModmailError, ModmailResult};
use crate::handlers::guild_interaction_handler::InteractionHandler;
use crate::i18n::get_translated_message;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::sync::Arc;

pub struct ForceCloseCommand;

#[async_trait::async_trait]
impl RegistrableCommand for ForceCloseCommand {
    fn name(&self) -> &'static str {
        "force_close"
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.force_close_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new("force_close").description(cmd_desc)]
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
                .ok_or_else(common::database_connection_failed)?;

            if !is_a_ticket_channel(command.channel_id, db_pool).await {
                match command.channel_id.to_channel(&ctx.http).await {
                    Ok(channel) => {
                        let guild_channel = match channel.guild() {
                            Some(guild_channel) => guild_channel,
                            None => {
                                return Err(ModmailError::Thread(NotAThreadChannel));
                            }
                        };

                        if let Some(category_id) = guild_channel.parent_id {
                            if category_id == config.thread.inbox_category_id {
                                delete_channel(&ctx, command.channel_id).await?;
                            } else {
                                return Err(ModmailError::Thread(NotAThreadChannel));
                            }
                        }
                    }
                    Err(_) => {
                        return Err(ModmailError::Thread(NotAThreadChannel));
                    }
                }
            }

            match is_orphaned_thread_channel(command.channel_id, db_pool).await {
                Ok(res) => {
                    if !res {
                        return Err(ModmailError::Thread(UserStillInServer));
                    }
                    delete_channel(&ctx, command.channel_id).await
                }
                Err(..) => Err(ModmailError::Database(QueryFailed(
                    "Failed to check if thread channel is orphaned".to_string(),
                ))),
            }
        })
    }
}
