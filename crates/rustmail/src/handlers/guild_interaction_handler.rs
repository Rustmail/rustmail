use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::features::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use crate::utils::{MessageBuilder, defer_response};
use serenity::all::{Context, EventHandler, Interaction, Permissions};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::watch::Receiver;

#[derive(Clone)]
pub struct InteractionHandler {
    pub config: Arc<Config>,
    pub registry: Arc<CommandRegistry>,
    pub shutdown: Arc<Receiver<bool>>,
    pub pagination: PaginationStore,
    pub maintenance_mode: Arc<AtomicBool>,
}

impl InteractionHandler {
    pub fn new(
        config: &Config,
        registry: Arc<CommandRegistry>,
        shutdown: Receiver<bool>,
        pagination: PaginationStore,
        maintenance_mode: Arc<AtomicBool>,
    ) -> Self {
        Self {
            config: Arc::new(config.clone()),
            registry,
            shutdown: Arc::new(shutdown),
            pagination,
            maintenance_mode,
        }
    }
}

async fn is_interaction_user_maintenance_exempt(
    ctx: &Context,
    guild_id: serenity::all::GuildId,
    user_id: serenity::all::UserId,
    config: &Config,
) -> bool {
    if config.bot.panel_super_admin_users.contains(&user_id.get()) {
        return true;
    }

    let member = match guild_id.member(&ctx.http, user_id).await {
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

    if guild.owner_id == user_id {
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

#[async_trait::async_trait]
impl EventHandler for InteractionHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Component(mut comp) => {
                if let Err(..) =
                    handle_feature_component_interaction(&ctx, &self.config, &comp).await
                {
                    return;
                }
                if let Err(..) =
                    handle_thread_component_interaction(&ctx, &self.config, &mut comp).await
                {
                    return;
                }
                if let Err(..) = handle_command_component_interaction(
                    &ctx,
                    &self.config,
                    &mut comp,
                    self.pagination.clone(),
                )
                .await
                {
                    return;
                }
            }
            Interaction::Modal(mut modal) => {
                if let Err(..) =
                    handle_thread_modal_interaction(&ctx, &self.config, &mut modal).await
                {
                    return;
                }
            }
            Interaction::Command(command) => {
                if self.maintenance_mode.load(Ordering::Relaxed) {
                    if let Some(guild_id) = command.guild_id {
                        if !is_interaction_user_maintenance_exempt(
                            &ctx,
                            guild_id,
                            command.user.id,
                            &self.config,
                        )
                        .await
                        {
                            defer_response(&ctx, &command).await.ok();

                            let _ = MessageBuilder::system_message(&ctx, &self.config)
                                .translated_content(
                                    "status.maintenance_mode_active",
                                    None,
                                    Some(command.user.id),
                                    command.guild_id.map(|g| g.get()),
                                )
                                .await
                                .send_interaction_followup(&command, true)
                                .await;
                            return;
                        }
                    }
                }

                let ctx = ctx.clone();
                let command = command.clone();
                let options = command.data.options().clone();
                let config = self.config.clone();

                if let Some(handler) = self.registry.get(command.data.name.as_str()) {
                    let result = handler
                        .run(&ctx, &command, &options, &config, Arc::new(self.clone()))
                        .await;

                    if let Err(e) = result {
                        if let Some(error_handler) = &self.config.error_handler {
                            let _ = error_handler
                                .reply_to_command_with_error(&ctx, &command, &e)
                                .await;
                        }
                    }
                } else {
                    eprintln!("Command {} not found", command.data.name);
                }
            }
            _ => {}
        }
    }
}
