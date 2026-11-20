pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, MessageId, UserId};
use std::collections::HashMap;

pub struct EditCommand;

impl Command for EditCommand {
    type Args = EditArgs;

    fn name(&self) -> &'static str {
        "edit"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["e"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.edit", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("edit")
                .description(translate("slash_command.edit_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "message_id",
                        translate("slash_command.edit_message_id_argument").await,
                    )
                    .required(true),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "message",
                        translate("slash_command.edit_message_argument").await,
                    )
                    .required(true),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<EditArgs> {
        parse_edit_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<EditArgs> {
        parse_edit_slash(options)
    }

    async fn run(
        &self,
        args: EditArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let ctx = invocation.ctx();
        let staff = invocation.user();
        let ticket = current_ticket(invocation, pool).await?;

        let ids = get_message_ids_by_number(args.number, staff.id, &ticket.id, pool)
            .await
            .ok_or_else(|| message_not_found(&args.number.to_string()))?;
        let inbox_message_id = ids
            .inbox_message_id
            .ok_or_else(|| message_not_found(&args.number.to_string()))?;
        let original = get_thread_message_by_inbox_message_id(&inbox_message_id, pool)
            .await
            .map_err(|_| permission_denied())?;
        if original.user_id != staff.id.get() as i64 {
            return Err(permission_denied());
        }

        MessageBuilder::begin_staff_reply(
            ctx,
            config,
            ticket.id.clone(),
            staff.id,
            staff.name.clone(),
            args.number as u64,
        )
        .content(args.content.clone())
        .anonymous(original.is_anonymous)
        .to_thread(invocation.channel_id())
        .to_user(UserId::new(ticket.user_id as u64))
        .edit_sent(
            parse_message_id(&inbox_message_id).ok_or_else(|| message_not_found(""))?,
            ids.dm_message_id.as_deref().and_then(parse_message_id),
        )
        .await?;
        update_message_content(&inbox_message_id, &args.content, pool).await?;

        let mut confirmed = false;
        if config.notifications.show_success_on_edit {
            invocation
                .reply(
                    MessageBuilder::system_message(ctx, config)
                        .translated_content(
                            "success.message_edited",
                            None,
                            Some(staff.id),
                            invocation.guild_id().map(|g| g.get()),
                        )
                        .await
                        .color(hex_string_to_int(&config.thread.system_message_color) as u32),
                )
                .await?;
            confirmed = true;
        }
        if config.logs.show_log_on_edit {
            let before = if original.content.is_empty() {
                "—".to_string()
            } else {
                format!("`{}`", original.content)
            };
            let params = HashMap::from([
                ("before".to_string(), before),
                ("after".to_string(), format!("`{}`", args.content)),
                (
                    "link".to_string(),
                    format!(
                        "https://discord.com/channels/{}/{}/{}",
                        config.bot.get_staff_guild_id(),
                        invocation.channel_id(),
                        inbox_message_id
                    ),
                ),
            ]);
            invocation
                .reply_translated(config, "edit.modification_from_staff", Some(&params))
                .await?;
            confirmed = true;
        }

        match invocation.message() {
            Some(message) => {
                let _ = message.delete(&ctx.http).await;
            }
            None if !confirmed => {
                invocation
                    .reply_translated(config, "success.message_edited", None)
                    .await?;
            }
            None => {}
        }
        Ok(())
    }
}

fn parse_message_id(id: &str) -> Option<MessageId> {
    id.parse().ok().map(MessageId::new)
}
