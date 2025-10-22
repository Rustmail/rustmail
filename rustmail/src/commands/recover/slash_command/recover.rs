use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::errors::{ModmailResult, common};
use crate::i18n::get_translated_message;
use crate::modules::message_recovery::recover_missing_messages;
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch::Receiver;
use crate::types::logs::PaginationStore;

pub struct RecoverCommand;

#[async_trait::async_trait]
impl RegistrableCommand for RecoverCommand {
    fn name(&self) -> &'static str {
        "recover"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.recover_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new("recover").description(cmd_desc)]
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
            let _ = config
                .db_pool
                .as_ref()
                .ok_or_else(common::database_connection_failed)?;

            defer_response(&ctx, &command).await?;

            let mut params = HashMap::new();
            params.insert("user".to_string(), command.user.name.clone());

            let response = MessageBuilder::system_message(&ctx, &config)
                .translated_content(
                    "recovery.started",
                    Some(&params),
                    Some(command.user.id),
                    command.guild_id.map(|g| g.get()),
                )
                .await
                .to_channel(command.channel_id)
                .build_interaction_message_followup()
                .await;

            let _ = command.create_followup(&ctx.http, response).await;

            let ctx_clone = ctx.clone();
            let config_clone = config.clone();
            let channel_id = command.channel_id;
            let command_clone = command.clone();

            tokio::spawn(async move {
                let recovery_results = recover_missing_messages(&ctx_clone, &config_clone).await;

                let total_recovered: u32 = recovery_results.iter().map(|r| r.recovered_count).sum();
                let successful_threads = recovery_results.iter().filter(|r| r.success).count();
                let failed_threads = recovery_results.len() - successful_threads;

                let mut params = HashMap::new();
                params.insert("total".to_string(), total_recovered.to_string());
                params.insert("threads".to_string(), successful_threads.to_string());
                params.insert("failed".to_string(), failed_threads.to_string());

                let response = MessageBuilder::system_message(&ctx_clone, &config_clone)
                    .translated_content("recovery.summary", Some(&params), None, None)
                    .await
                    .to_channel(channel_id)
                    .build_interaction_message_followup()
                    .await;

                let _ = command_clone
                    .create_followup(&ctx_clone.http, response)
                    .await;
            });

            Ok(())
        })
    }
}
