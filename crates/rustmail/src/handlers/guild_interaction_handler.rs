use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::features::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use crate::utils::{
    MessageBuilder, defer_response, defer_response_ephemeral, is_bot_admin, is_staff_member,
    is_staff_user, responds_ephemerally,
};
use serenity::all::{Context, EventHandler, Interaction};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
pub struct InteractionHandler {
    pub config: Arc<Config>,
    pub registry: Arc<CommandRegistry>,
    pub pagination: PaginationStore,
    pub maintenance_mode: Arc<AtomicBool>,
}

impl InteractionHandler {
    pub fn new(
        config: &Config,
        registry: Arc<CommandRegistry>,
        pagination: PaginationStore,
        maintenance_mode: Arc<AtomicBool>,
    ) -> Self {
        Self {
            config: Arc::new(config.clone()),
            registry,
            pagination,
            maintenance_mode,
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for InteractionHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Component(mut comp) => {
                match handle_category_component_interaction(&ctx, &self.config, &comp).await {
                    Ok(true) => return,
                    Ok(false) => {}
                    Err(e) => {
                        eprintln!("category interaction error: {e:?}");
                        return;
                    }
                }
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
                let community_command = self.registry.is_community(command.data.name.as_str());
                let is_staff = match command.guild_id {
                    Some(_) if community_command => {
                        is_staff_user(&ctx, &self.config, command.user.id).await
                    }
                    Some(guild_id) => {
                        is_staff_member(&ctx, &self.config, guild_id, command.user.id).await
                    }
                    None => true,
                };
                if !is_staff {
                    if defer_response_ephemeral(&ctx, &command).await.is_ok()
                        && let Some(error_handler) = &self.config.error_handler
                    {
                        let _ = error_handler
                            .reply_to_command_with_error(&ctx, &command, &permission_denied())
                            .await;
                    }
                    return;
                }

                if self.maintenance_mode.load(Ordering::Relaxed)
                    && let Some(guild_id) = command.guild_id
                    && !is_bot_admin(&ctx, &self.config, Some(guild_id), command.user.id).await
                {
                    defer_response(&ctx, &command, &self.config).await.ok();

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

                let ctx = ctx.clone();
                let command = command.clone();
                let config = self.config.clone();

                match self.registry.slash(command.data.name.as_str()) {
                    Some(handler) => {
                        let ephemeral =
                            handler.ephemeral() || responds_ephemerally(&command, &config);
                        let deferred = if ephemeral {
                            defer_response_ephemeral(&ctx, &command).await
                        } else {
                            defer_response(&ctx, &command, &config).await
                        };
                        if let Err(e) = deferred {
                            eprintln!("Failed to defer command {}: {}", command.data.name, e);
                            return;
                        }

                        let slash_options = SlashOptions::from_interaction(&command);
                        let invocation = Invocation::slash(ctx, command, ephemeral);
                        if let Err(e) = handler
                            .run_slash(&slash_options, &invocation, &config)
                            .await
                        {
                            invocation.reply_error(&config, &e).await;
                        }
                    }
                    None => eprintln!("Command {} not found", command.data.name),
                }
            }
            _ => {}
        }
    }
}
