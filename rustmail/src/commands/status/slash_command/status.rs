use crate::commands::status::BotStatus;
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::errors::{CommandError, ModmailError, ModmailResult};
use crate::handlers::InteractionHandler;
use crate::i18n::get_translated_message;
use crate::utils::{MessageBuilder, defer_response};
use serenity::FutureExt;
use serenity::all::{
    ActivityData, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateCommand, CreateCommandOption, Permissions, ResolvedOption,
};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::Ordering;

async fn is_interaction_user_admin_or_super_admin(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
) -> bool {
    let user_id = command.user.id.get();

    if config.bot.panel_super_admin_users.contains(&user_id) {
        return true;
    }

    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return false,
    };

    let member = match guild_id.member(&ctx.http, command.user.id).await {
        Ok(m) => m,
        Err(_) => return false,
    };

    if !config.bot.panel_super_admin_roles.is_empty() {
        for role_id in &member.roles {
            if config.bot.panel_super_admin_roles.contains(&role_id.get()) {
                return true;
            }
        }
    }

    let guild = match guild_id.to_partial_guild(&ctx.http).await {
        Ok(g) => g,
        Err(_) => return false,
    };

    if guild.owner_id == command.user.id {
        return true;
    }

    member.roles.iter().any(|role_id| {
        guild
            .roles
            .get(role_id)
            .map(|role| role.permissions.contains(Permissions::ADMINISTRATOR))
            .unwrap_or(false)
    })
}

pub struct StatusCommand;

impl RegistrableCommand for StatusCommand {
    fn name(&self) -> &'static str {
        "status"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
            get_translated_message(
                config,
                "slash_command.status_command_help",
                None,
                None,
                None,
                None,
            )
            .await
        }
        .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.status_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let mode_desc = get_translated_message(
                &config,
                "slash_command.mode_arg_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let online_mode = get_translated_message(
                &config,
                "slash_command.online_status_mode",
                None,
                None,
                None,
                None,
            )
            .await;
            let idle_mode = get_translated_message(
                &config,
                "slash_command.idle_status_mode",
                None,
                None,
                None,
                None,
            )
            .await;
            let dnd_mode = get_translated_message(
                &config,
                "slash_command.do_not_disturb_status_mode",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("status")
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::String, "mode", mode_desc)
                            .add_string_choice(online_mode, "online")
                            .add_string_choice(idle_mode, "idle")
                            .add_string_choice(dnd_mode, "dnd")
                            .add_string_choice("Invisible", "invisible")
                            .add_string_choice("Maintenance", "maintenance")
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
        handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            defer_response(&ctx, &command).await?;

            let mode = command.data.options.iter().find_map(|opt| {
                if opt.name == "mode" {
                    if let CommandDataOptionValue::String(s) = &opt.value {
                        return Some(s.clone());
                    }
                }
                None
            });

            let mode = match mode {
                Some(m) => m,
                None => return Ok(()),
            };

            let bot_status = match BotStatus::from_str(&mode) {
                Ok(status) => status,
                Err(_) => return Ok(()),
            };

            if bot_status == BotStatus::Maintenance {
                let is_allowed =
                    is_interaction_user_admin_or_super_admin(&ctx, &command, &config).await;
                if !is_allowed {
                    return Err(ModmailError::Command(
                        CommandError::MaintenanceModeNotAllowed,
                    ));
                }
            }

            let message_key = match bot_status {
                BotStatus::Online => {
                    handler.maintenance_mode.store(false, Ordering::Relaxed);
                    ctx.set_activity(Some(ActivityData::playing(&config.bot.status)));
                    ctx.online();
                    "status.status_online"
                }
                BotStatus::Idle => {
                    ctx.idle();
                    "status.status_idle"
                }
                BotStatus::Dnd => {
                    ctx.dnd();
                    "status.status_dnd"
                }
                BotStatus::Invisible => {
                    ctx.invisible();
                    "status.status_invisible"
                }
                BotStatus::Maintenance => {
                    handler.maintenance_mode.store(true, Ordering::Relaxed);
                    let maintenance_status = get_translated_message(
                        &config,
                        "status.maintenance_activity",
                        None,
                        None,
                        None,
                        None,
                    )
                    .await;
                    ctx.set_activity(Some(ActivityData::playing(&maintenance_status)));
                    ctx.dnd();
                    "status.status_maintenance"
                }
            };

            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content(message_key, None, None, None)
                .await
                .send_interaction_followup(&command, true)
                .await;

            Ok(())
        })
    }
}
