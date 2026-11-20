use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::CreateCommand;

pub struct ForceCloseCommand;

impl Command for ForceCloseCommand {
    type Args = ();

    fn name(&self) -> &'static str {
        "force_close"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["fc"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.force_close", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let description = get_translated_message(
            config,
            "slash_command.force_close_command_description",
            None,
            None,
            None,
            None,
        )
        .await;

        vec![CreateCommand::new("force_close").description(description)]
    }

    fn parse_text(&self, _args: &str) -> ModmailResult<()> {
        Ok(())
    }

    fn parse_slash(&self, _options: &SlashOptions) -> ModmailResult<()> {
        Ok(())
    }

    async fn run(&self, _args: (), invocation: &Invocation, config: &Config) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let ctx = invocation.ctx();
        let channel_id = invocation.channel_id();

        if is_a_ticket_channel(channel_id, pool).await {
            let orphaned = is_orphaned_thread_channel(channel_id, pool)
                .await
                .map_err(|e| ModmailError::Database(DatabaseError::QueryFailed(e.to_string())))?;
            if !orphaned {
                return Err(ModmailError::Thread(ThreadError::UserStillInServer));
            }
        } else {
            let category_id = channel_id
                .to_channel(ctx)
                .await
                .ok()
                .and_then(|channel| channel.guild())
                .and_then(|channel| channel.parent_id);
            if category_id.map(|id| id.get()) != Some(config.thread.inbox_category_id) {
                return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
            }
        }

        channel_id
            .delete(ctx)
            .await
            .map_err(|e| ModmailError::Discord(DiscordError::ApiError(e.to_string())))?;
        Ok(())
    }
}
