use crate::commands::add_staff::slash_command::add_staff;
use crate::commands::alert::slash_command::alert;
use crate::commands::close::slash_command::close;
use crate::commands::delete::slash_command::delete;
use crate::commands::edit::slash_command::edit;
use crate::commands::force_close::slash_command::force_close;
use crate::commands::id::slash_command::id;
use crate::commands::move_thread::slash_command::move_thread;
use crate::commands::new_thread::slash_command::new_thread;
use crate::commands::recover::slash_command::recover;
use crate::commands::remove_staff::slash_command::remove_staff;
use crate::commands::reply::slash_command::reply;
use crate::config::Config;
use crate::errors::{CommandError, ModmailError};
use crate::features::handle_feature_component_interaction;
use crate::modules::threads::{
    handle_thread_component_interaction, handle_thread_modal_interaction,
};
use serenity::all::{Context, EventHandler, Interaction};
use serenity::async_trait;

#[derive(Clone)]
pub struct InteractionHandler {
    pub config: Config,
}

impl InteractionHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
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
            }
            Interaction::Modal(mut modal) => {
                if let Err(..) =
                    handle_thread_modal_interaction(&ctx, &self.config, &mut modal).await
                {
                    return;
                }
            }
            Interaction::Command(command) => {
                let command_return = match command.data.name.as_str() {
                    "id" => id::run(&ctx, &command, &command.data.options(), &self.config).await,
                    "move" => {
                        move_thread::run(&ctx, &command, &command.data.options(), &self.config)
                            .await
                    }
                    "new_thread" => {
                        new_thread::run(&ctx, &command, &command.data.options(), &self.config).await
                    }
                    "close" => {
                        close::run(&ctx, &command, &command.data.options(), &self.config).await
                    }
                    "edit" => {
                        edit::run(&ctx, &command, &command.data.options(), &self.config).await
                    }
                    "add_staff" => {
                        add_staff::run(&ctx, &command, &command.data.options(), &self.config).await
                    }
                    "remove_staff" => {
                        remove_staff::run(&ctx, &command, &command.data.options(), &self.config)
                            .await
                    }
                    "alert" => {
                        alert::run(&ctx, &command, &command.data.options(), &self.config).await
                    }
                    "force_close" => {
                        force_close::run(&ctx, &command, &command.data.options(), &self.config)
                            .await
                    }
                    "reply" => {
                        reply::run(&ctx, &command, &command.data.options(), &self.config).await
                    }
                    "delete" => {
                        delete::run(&ctx, &command, &command.data.options(), &self.config).await
                    }
                    "recover" => {
                        recover::run(&ctx, &command, &command.data.options(), &self.config).await
                    }
                    _ => Err(ModmailError::Command(CommandError::UnknownSlashCommand(
                        command.data.name.clone(),
                    ))),
                };

                if let Err(error) = command_return {
                    if let Some(error_handler) = &self.config.error_handler {
                        let _ = error_handler
                            .reply_to_command_with_error(&ctx, &command, &error)
                            .await;
                    } else {
                        eprintln!("Command error: {}", error);
                    }
                }
            }
            _ => {}
        }
    }
}
