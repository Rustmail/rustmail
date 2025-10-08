use crate::commands::CommandRegistry;
use crate::commands::add_reminder::slash_command::add_reminder::AddReminderCommand;
use crate::commands::add_staff::slash_command::add_staff::AddStaffCommand;
use crate::commands::alert::slash_command::alert::AlertCommand;
use crate::commands::close::slash_command::close::CloseCommand;
use crate::commands::delete::slash_command::delete::DeleteCommand;
use crate::commands::edit::slash_command::edit::EditCommand;
use crate::commands::force_close::slash_command::force_close::ForceCloseCommand;
use crate::commands::help::slash_command::help::HelpCommand;
use crate::commands::id::slash_command::id::IdCommand;
use crate::commands::move_thread::slash_command::move_thread::MoveCommand;
use crate::commands::new_thread::slash_command::new_thread::NewThreadCommand;
use crate::commands::recover::slash_command::recover::RecoverCommand;
use crate::commands::remove_staff::slash_command::remove_staff::RemoveStaffCommand;
use crate::commands::reply::slash_command::reply::ReplyCommand;
use crate::handlers::guild_handler::GuildHandler;
use crate::handlers::guild_interaction_handler::InteractionHandler;
use crate::handlers::guild_message_reactions_handler::GuildMessageReactionsHandler;
use crate::handlers::guild_moderation_handler::GuildModerationHandler;
use crate::handlers::typing_proxy_handler::TypingProxyHandler;
use config::load_config;
use handlers::{
    guild_members_handler::GuildMembersHandler, guild_messages_handler::GuildMessagesHandler,
    ready_handler::ReadyHandler,
};
use serenity::all::{ClientBuilder, GatewayIntents};
use serenity::cache::Settings as CacheSettings;
use std::process;
use std::sync::Arc;
use std::time::Duration;

mod commands;
mod config;
mod db;
mod errors;
mod features;
mod handlers;
mod i18n;
mod modules;
mod utils;

pub struct Database;

#[tokio::main]
async fn main() {
    let pool = db::operations::init_database()
        .await
        .expect("An error occured!");
    println!("Database connected!");

    let mut config = load_config("config.toml");
    config.db_pool = Some(pool.clone());

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_TYPING
        | GatewayIntents::DIRECT_MESSAGE_TYPING
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MODERATION;

    let mut cache_settings = CacheSettings::default();
    cache_settings.max_messages = 10_000;
    cache_settings.time_to_live = Duration::from_secs(6 * 60 * 60);

    let mut registry = CommandRegistry::new();
    registry.register_command(AddStaffCommand);
    registry.register_command(AlertCommand);
    registry.register_command(CloseCommand);
    registry.register_command(DeleteCommand);
    registry.register_command(EditCommand);
    registry.register_command(ForceCloseCommand);
    registry.register_command(HelpCommand);
    registry.register_command(IdCommand);
    registry.register_command(MoveCommand);
    registry.register_command(NewThreadCommand);
    registry.register_command(RecoverCommand);
    registry.register_command(RemoveStaffCommand);
    registry.register_command(ReplyCommand);
    registry.register_command(AddReminderCommand);

    let registry = Arc::new(registry);

    let mut client: serenity::Client = ClientBuilder::new(config.bot.token.clone(), intents)
        .cache_settings(cache_settings)
        .event_handler(ReadyHandler::new(&config, registry.clone()))
        .event_handler(GuildMessagesHandler::new(&config))
        .event_handler(TypingProxyHandler::new(&config))
        .event_handler(GuildMembersHandler::new(&config))
        .event_handler(GuildMessageReactionsHandler::new(&config))
        .event_handler(GuildModerationHandler::new(&config))
        .event_handler(InteractionHandler::new(&config, registry.clone()))
        .event_handler(GuildHandler::new(&config))
        .await
        .expect("Failed to create client.");

    if let Err(e) = config.validate_servers(&client.http).await {
        eprintln!("Configuration validation error: {}", e);
        eprintln!(
            "Check that the server IDs are correct and that the bot has access to the servers."
        );
        process::exit(1);
    }

    println!("Configuration successfully validated!!");
    if config.bot.is_dual_mode() {
        println!(
            "Mode: Dual server (Community: {}, Staff: {})",
            config.bot.get_community_guild_id(),
            config.bot.get_staff_guild_id()
        );
    } else {
        println!(
            "Mode: Mono server (ID: {})",
            config.bot.get_community_guild_id()
        );
    }

    if let Err(e) = client.start().await {
        println!("Failed to initialize client: {e}");
    }
}
