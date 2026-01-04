use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, GuildId, ResolvedOption, RoleId,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ReminderSubscriptionCommand;

#[async_trait::async_trait]
impl RegistrableCommand for ReminderSubscriptionCommand {
    fn name(&self) -> &'static str {
        "reminder_subscription"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
            get_translated_message(config, "help.reminder_subscription", None, None, None, None)
                .await
        }
        .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();
        let name = self.name();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.reminder_subscribe_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let action_desc = get_translated_message(
                &config,
                "slash_command.reminder_action_argument",
                None,
                None,
                None,
                None,
            )
            .await;
            let role_desc = get_translated_message(
                &config,
                "slash_command.reminder_role_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new(name)
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::String, "action", action_desc)
                            .required(true)
                            .add_string_choice("subscribe", "subscribe")
                            .add_string_choice("unsubscribe", "unsubscribe"),
                    )
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::Role, "role", role_desc)
                            .required(true),
                    ),
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

            let _ = defer_response(&ctx, &command).await;

            let mut action: Option<String> = None;
            let mut role_id: Option<RoleId> = None;

            for option in &command.data.options {
                match option.name.as_str() {
                    "action" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            action.replace(val.clone());
                        }
                    }
                    "role" => {
                        if let CommandDataOptionValue::Role(val) = &option.value {
                            role_id.replace(*val);
                        }
                    }
                    _ => {}
                }
            }

            let action =
                action.ok_or_else(|| ModmailError::Command(CommandError::MissingArguments))?;

            let role_id =
                role_id.ok_or_else(|| ModmailError::Command(CommandError::MissingArguments))?;

            let is_subscribe = action == "subscribe";

            // Get the guild
            let guild_id = config.bot.get_staff_guild_id();
            let guild_id_obj = GuildId::new(guild_id);
            let guild = guild_id_obj
                .to_partial_guild(&ctx.http)
                .await
                .map_err(|_| {
                    ModmailError::Discord(DiscordError::ApiError("Guild not found".to_string()))
                })?;

            // Get the role name
            let role = guild.roles.get(&role_id).ok_or_else(|| {
                ModmailError::Discord(DiscordError::ApiError("Role not found".to_string()))
            })?;

            let role_name = role.name.clone();

            // Check if the user has the role
            let member = guild_id_obj
                .member(&ctx.http, command.user.id)
                .await
                .map_err(|_| ModmailError::Discord(DiscordError::UserNotFound))?;

            if !member.roles.contains(&role_id) {
                let mut params = HashMap::new();
                params.insert("role".to_string(), role_name.clone());

                let _ = MessageBuilder::system_message(&ctx, &config)
                    .translated_content(
                        "reminder_subscription.role_required",
                        Some(&params),
                        Some(command.user.id),
                        command.guild_id.map(|g| g.get()),
                    )
                    .await
                    .to_channel(command.channel_id)
                    .send_interaction_followup(&command, true)
                    .await;
                return Ok(());
            }

            // Perform the subscription/unsubscription
            let mut params = HashMap::new();
            params.insert("role".to_string(), role_name.clone());

            if is_subscribe {
                // Re-subscribe: delete the opt-out record
                let was_opted_out = delete_reminder_optout(
                    guild_id as i64,
                    command.user.id.get() as i64,
                    role_id.get() as i64,
                    pool,
                )
                .await?;

                let message_key = if was_opted_out {
                    "reminder_subscription.subscribed"
                } else {
                    "reminder_subscription.already_subscribed"
                };

                let _ = MessageBuilder::system_message(&ctx, &config)
                    .translated_content(
                        message_key,
                        Some(&params),
                        Some(command.user.id),
                        command.guild_id.map(|g| g.get()),
                    )
                    .await
                    .to_channel(command.channel_id)
                    .send_interaction_followup(&command, true)
                    .await;
            } else {
                // Unsubscribe: insert an opt-out record
                let is_already_opted_out = is_user_opted_out(
                    guild_id as i64,
                    command.user.id.get() as i64,
                    role_id.get() as i64,
                    pool,
                )
                .await?;

                if is_already_opted_out {
                    let _ = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "reminder_subscription.already_unsubscribed",
                            Some(&params),
                            Some(command.user.id),
                            command.guild_id.map(|g| g.get()),
                        )
                        .await
                        .to_channel(command.channel_id)
                        .send_interaction_followup(&command, true)
                        .await;
                } else {
                    insert_reminder_optout(
                        guild_id as i64,
                        command.user.id.get() as i64,
                        role_id.get() as i64,
                        pool,
                    )
                    .await?;

                    let _ = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "reminder_subscription.unsubscribed",
                            Some(&params),
                            Some(command.user.id),
                            command.guild_id.map(|g| g.get()),
                        )
                        .await
                        .to_channel(command.channel_id)
                        .send_interaction_followup(&command, true)
                        .await;
                }
            }

            Ok(())
        })
    }
}
