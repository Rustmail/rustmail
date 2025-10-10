use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::get_thread_by_channel_id;
use crate::db::threads::is_a_ticket_channel;
use crate::errors::ThreadError::{NotAThreadChannel, ThreadNotFound};
use crate::errors::{DatabaseError, ModmailError, ModmailResult};
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{CommandInteraction, Context, ResolvedOption};
use serenity::builder::CreateCommand;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub struct IdCommand;

#[async_trait::async_trait]
impl RegistrableCommand for IdCommand {
    fn name(&self) -> &'static str {
        "id"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.id_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new("id").description(cmd_desc)]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        _options: &[ResolvedOption<'_>],
        config: &Config,
        _shutdown: Arc<Receiver<bool>>,
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

            if !is_a_ticket_channel(command.channel_id, pool).await {
                return Err(ModmailError::Thread(NotAThreadChannel));
            }

            let thread = match get_thread_by_channel_id(&command.channel_id.to_string(), pool).await
            {
                Some(thread) => thread,
                None => {
                    return Err(ModmailError::Thread(ThreadNotFound));
                }
            };

            let mut params = std::collections::HashMap::new();
            params.insert("user".to_string(), format!("<@{}>", thread.user_id));
            params.insert(
                "id".to_string(),
                format!("||{}||", thread.user_id.to_string()),
            );

            let response = MessageBuilder::system_message(&ctx, &config)
                .translated_content("id.show_id", Some(&params), None, None)
                .await
                .to_channel(command.channel_id)
                .build_interaction_message_followup()
                .await;

            let _ = command.create_followup(&ctx.http, response).await;

            Ok(())
        })
    }
}
