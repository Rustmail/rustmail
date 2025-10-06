use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::errors::ModmailResult;
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};

pub struct HelpCommand;

#[async_trait::async_trait]
impl RegistrableCommand for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
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

            vec![CreateCommand::new("help").description(cmd_desc)]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        options: &[ResolvedOption<'_>],
        config: &Config,
    ) -> BoxFuture<ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let help_message = "# Available commands:\n\n\
                **!add_staff <staff_id>** - Add a staff to an hidden ticket\n\
                **!remove_staff <staff_id>** - Remove a staff from a ticket\n\
                **!alert** - Alert staff in the current thread when a new user answer arrives \n\
                **!help** - Show this help message\n\
                **!reply <message>** - Reply to a ticket\n\
                **!annonreply <message>** - Reply anonymously to a ticket\n\
                **!close [reason]** - Close the current thread with an optional reason\n\
                **!delete [reason]** - Delete the current thread with an optional reason\n\
                **!hide** - Make the current thread hidden to non-staff members\n\
                **!new_thread <user_id>** - Create a new ticket with a specific user\n\
                **!alert [cancel]** - Set or cancel an alert for staff in the current thread\n\
                **!force_close** - Force close the current thread if it's orphaned\n\
                **!id** - Show the user ID associated with the current thread";

            defer_response(&ctx, &command).await?;

            let response = MessageBuilder::system_message(&ctx, &config)
                .content(help_message)
                .to_channel(command.channel_id)
                .build_interaction_message_followup()
                .await;

            let _ = command.create_followup(&ctx.http, response).await;

            Ok(())
        })
    }
}
