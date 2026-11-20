pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{ActivityData, CommandOptionType, Context, CreateCommand, CreateCommandOption};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct StatusCommand {
    pub maintenance_mode: Arc<AtomicBool>,
}

impl Command for StatusCommand {
    type Args = PresenceMode;

    fn name(&self) -> &'static str {
        "status"
    }

    async fn doc(&self, config: &Config) -> String {
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

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("status")
                .description(translate("slash_command.status_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "mode",
                        translate("slash_command.mode_arg_description").await,
                    )
                    .add_string_choice(
                        translate("slash_command.online_status_mode").await,
                        "online",
                    )
                    .add_string_choice(translate("slash_command.idle_status_mode").await, "idle")
                    .add_string_choice(
                        translate("slash_command.do_not_disturb_status_mode").await,
                        "dnd",
                    )
                    .add_string_choice("Invisible", "invisible")
                    .add_string_choice("Maintenance", "maintenance")
                    .required(true),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<PresenceMode> {
        parse_status_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<PresenceMode> {
        parse_status_slash(options)
    }

    async fn run(
        &self,
        mode: PresenceMode,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let ctx = invocation.ctx();

        if mode == PresenceMode::Maintenance
            && !is_bot_admin(ctx, config, invocation.guild_id(), invocation.user_id()).await
        {
            return Err(ModmailError::Command(
                CommandError::MaintenanceModeNotAllowed,
            ));
        }

        apply_presence(ctx, config, mode, &self.maintenance_mode).await;
        invocation
            .reply_translated(config, mode.confirmation_key(), None)
            .await?;
        Ok(())
    }
}

impl PresenceMode {
    fn confirmation_key(self) -> &'static str {
        match self {
            PresenceMode::Online => "status.status_online",
            PresenceMode::Idle => "status.status_idle",
            PresenceMode::Dnd => "status.status_dnd",
            PresenceMode::Invisible => "status.status_invisible",
            PresenceMode::Maintenance => "status.status_maintenance",
        }
    }
}

pub async fn apply_presence(
    ctx: &Context,
    config: &Config,
    mode: PresenceMode,
    maintenance_mode: &AtomicBool,
) {
    match mode {
        PresenceMode::Online => {
            maintenance_mode.store(false, Ordering::Relaxed);
            ctx.set_activity(Some(ActivityData::playing(&config.bot.status)));
            ctx.online();
        }
        PresenceMode::Idle => ctx.idle(),
        PresenceMode::Dnd => ctx.dnd(),
        PresenceMode::Invisible => ctx.invisible(),
        PresenceMode::Maintenance => {
            maintenance_mode.store(true, Ordering::Relaxed);
            let activity = get_translated_message(
                config,
                "status.maintenance_activity",
                None,
                None,
                None,
                None,
            )
            .await;
            ctx.set_activity(Some(ActivityData::playing(&activity)));
            ctx.dnd();
        }
    }
}
