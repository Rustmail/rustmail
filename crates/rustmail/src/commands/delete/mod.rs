pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::{
    CommandOptionType, CommandType, CreateCommand, CreateCommandOption, MessageId, UserId,
};
use std::collections::HashMap;

pub struct DeleteCommand;

impl Command for DeleteCommand {
    type Args = DeleteArgs;

    fn name(&self) -> &'static str {
        "delete"
    }

    fn ephemeral(&self) -> bool {
        true
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.delete", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("delete")
                .description(translate("slash_command.delete_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Number,
                        "message_id",
                        translate("slash_command.delete_message_id_argument_description").await,
                    )
                    .required(true),
                ),
            CreateCommand::new("delete").kind(CommandType::Message),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<DeleteArgs> {
        parse_delete_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<DeleteArgs> {
        parse_delete_slash(options)
    }

    async fn run(
        &self,
        args: DeleteArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let ctx = invocation.ctx();

        let ticket = current_ticket(invocation, pool).await?;
        let number = match args {
            DeleteArgs::Number(number) => number,
            DeleteArgs::Message(message_id) => {
                let message = get_thread_message_by_message_id(&message_id.to_string(), pool)
                    .await
                    .map_err(|_| message_not_found(""))?;
                if message.thread_id != ticket.id {
                    return Err(message_not_found(""));
                }
                message
                    .message_number
                    .ok_or_else(|| message_not_found(""))?
            }
        };

        let user_id = UserId::new(ticket.user_id as u64);
        let ids = get_message_ids_by_number(number, user_id, &ticket.id, pool)
            .await
            .ok_or_else(|| message_not_found(&number.to_string()))?;

        if let Some(id) = parse_message_id(&ids.inbox_message_id)
            && let Err(e) = invocation.channel_id().delete_message(&ctx.http, id).await
        {
            eprintln!("Failed to delete inbox message: {e}");
            return Err(ModmailError::Command(CommandError::DiscordDeleteFailed));
        }

        if let Some(id) = parse_message_id(&ids.dm_message_id) {
            let deleted = match user_id.create_dm_channel(ctx).await {
                Ok(dm) => dm.id.delete_message(&ctx.http, id).await,
                Err(e) => Err(e),
            };
            if let Err(e) = deleted {
                eprintln!("Failed to delete DM message: {e}");
            }
        }

        if let Some(id) = ids.dm_message_id.as_ref().or(ids.inbox_message_id.as_ref()) {
            delete_message(id, pool)
                .await
                .map_err(|e| ModmailError::Database(DatabaseError::QueryFailed(e.to_string())))?;
        }
        if let Err(e) =
            update_message_numbers_after_deletion(&ticket.channel_id, number, pool).await
        {
            eprintln!("Failed to update message numbers: {e}");
        }

        match invocation.message() {
            Some(message) => {
                let _ = message.delete(&ctx.http).await;
            }
            None => {
                let params = HashMap::from([("number".to_string(), number.to_string())]);
                invocation
                    .reply_translated(config, "delete.success", Some(&params))
                    .await?;
            }
        }
        Ok(())
    }
}

fn parse_message_id(id: &Option<String>) -> Option<MessageId> {
    id.as_deref()?.parse().ok().map(MessageId::new)
}
