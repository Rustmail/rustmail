pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId, RoleId,
};
use std::collections::HashMap;

pub struct ReminderSubscriptionCommand;

impl Command for ReminderSubscriptionCommand {
    type Args = ReminderSubscriptionArgs;

    fn name(&self) -> &'static str {
        "reminder_subscription"
    }

    fn surfaces(&self) -> Surfaces {
        Surfaces::SlashOnly
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.reminder_subscription", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("reminder_subscription")
                .description(translate("slash_command.reminder_subscribe_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "action",
                        translate("slash_command.reminder_action_argument").await,
                    )
                    .required(true)
                    .add_string_choice("subscribe", "subscribe")
                    .add_string_choice("unsubscribe", "unsubscribe"),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Role,
                        "role",
                        translate("slash_command.reminder_role_argument").await,
                    )
                    .required(true),
                ),
        ]
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<ReminderSubscriptionArgs> {
        parse_reminder_subscription_slash(options)
    }

    async fn run(
        &self,
        args: ReminderSubscriptionArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        change_reminder_subscription(args, invocation, config).await
    }
}

pub async fn change_reminder_subscription(
    args: ReminderSubscriptionArgs,
    invocation: &Invocation,
    config: &Config,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;
    let ctx = invocation.ctx();
    let guild_id = GuildId::new(config.bot.get_staff_guild_id());

    let roles = guild_roles(ctx, guild_id).await?;
    let role_id = args.role.find(&roles).ok_or_else(|| {
        ModmailError::Command(CommandError::ReminderRoleNotFound(args.role.label()))
    })?;
    let role_name = roles[&role_id].clone();

    let member = guild_id
        .member(ctx, invocation.user_id())
        .await
        .map_err(|_| ModmailError::Discord(DiscordError::UserNotFound))?;
    if !member.roles.contains(&role_id) {
        return Err(ModmailError::Command(CommandError::ReminderRoleRequired(
            role_name,
        )));
    }

    let guild = guild_id.get() as i64;
    let user = invocation.user_id().get() as i64;
    let role = role_id.get() as i64;

    let key = match args.subscription {
        Subscription::Subscribe => {
            if !delete_reminder_optout(guild, user, role, pool).await? {
                return Err(ModmailError::Command(
                    CommandError::ReminderAlreadySubscribed(role_name),
                ));
            }
            "reminder_subscription.subscribed"
        }
        Subscription::Unsubscribe => {
            if is_user_opted_out(guild, user, role, pool).await? {
                return Err(ModmailError::Command(
                    CommandError::ReminderAlreadyUnsubscribed(role_name),
                ));
            }
            insert_reminder_optout(guild, user, role, pool).await?;
            "reminder_subscription.unsubscribed"
        }
    };

    let params = HashMap::from([("role".to_string(), role_name)]);
    invocation
        .reply_translated(config, key, Some(&params))
        .await?;
    Ok(())
}

pub async fn guild_roles(
    ctx: &Context,
    guild_id: GuildId,
) -> ModmailResult<HashMap<RoleId, String>> {
    if let Some(guild) = ctx.cache.guild(guild_id) {
        return Ok(guild
            .roles
            .iter()
            .map(|(id, role)| (*id, role.name.clone()))
            .collect());
    }
    let roles = guild_id.roles(&ctx.http).await.map_err(|_| {
        ModmailError::Discord(DiscordError::ApiError("Guild not found".to_string()))
    })?;
    Ok(roles
        .into_iter()
        .map(|(id, role)| (id, role.name))
        .collect())
}
