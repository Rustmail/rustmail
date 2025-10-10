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
use crate::commands::remove_reminder::slash_command::remove_reminder::RemoveReminderCommand;
use crate::commands::remove_staff::slash_command::remove_staff::RemoveStaffCommand;
use crate::commands::reply::slash_command::reply::ReplyCommand;
use crate::commands::CommandRegistry;
use crate::config::load_config;
use crate::handlers::guild_handler::GuildHandler;
use crate::handlers::guild_interaction_handler::InteractionHandler;
use crate::handlers::guild_members_handler::GuildMembersHandler;
use crate::handlers::guild_message_reactions_handler::GuildMessageReactionsHandler;
use crate::handlers::guild_messages_handler::GuildMessagesHandler;
use crate::handlers::guild_moderation_handler::GuildModerationHandler;
use crate::handlers::ready_handler::ReadyHandler;
use crate::handlers::typing_proxy_handler::TypingProxyHandler;
use crate::{db, BotState, BotStatus};
use serenity::all::{ClientBuilder, GatewayIntents};
use serenity::cache::Settings as CacheSettings;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::{select, spawn};

pub async fn init_bot_state() -> Arc<Mutex<BotState>> {
    let pool = db::operations::init_database()
        .await
        .expect("An error occured!");
    println!("Database connected!");

    let config = load_config("config.toml");

    let bot_state = BotState {
        config,
        status: BotStatus::Stopped,
        db_pool: Some(pool),
    };

    Arc::new(Mutex::new(bot_state))
}

pub async fn run_bot(bot_state: Arc<Mutex<BotState>>, shutdown: &mut Receiver<bool>) {
    let shutdown_rx_command = shutdown.clone();
    let shutdown_rx = shutdown.clone();

    let pool = {
        let state_lock = bot_state.lock().unwrap();
        state_lock.db_pool.clone().expect("Database pool not set")
    };

    let mut config = {
        let state_lock = bot_state.lock().unwrap();
        state_lock.config.clone().expect("Config not set")
    };

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

    let mut registry = CommandRegistry::new(shutdown_rx_command);
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
    registry.register_command(RemoveReminderCommand);

    let registry = Arc::new(registry);

    let mut client: serenity::Client = ClientBuilder::new(config.bot.token.clone(), intents)
        .cache_settings(cache_settings)
        .event_handler(ReadyHandler::new(
            &config,
            registry.clone(),
            Arc::new(shutdown_rx.clone()),
        ))
        .event_handler(GuildMessagesHandler::new(&config, shutdown_rx.clone()))
        .event_handler(TypingProxyHandler::new(&config))
        .event_handler(GuildMembersHandler::new(&config))
        .event_handler(GuildMessageReactionsHandler::new(&config))
        .event_handler(GuildModerationHandler::new(&config))
        .event_handler(InteractionHandler::new(
            &config,
            registry.clone(),
            shutdown_rx,
        ))
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

    let shard_manager = client.shard_manager.clone();

    let discord_task = spawn(async move {
        if let Err(e) = client.start().await {
            println!("Failed to initialize client: {e}");
        }
    });

    select! {
        _ = shutdown.changed() => {
            println!("Shutdown signal received, shutting down...");
            shard_manager.shutdown_all().await;
        }
        _ = discord_task => {
            println!("Discord task ended.");
        }
    }

    println!("Bot has been shut down.");
}
