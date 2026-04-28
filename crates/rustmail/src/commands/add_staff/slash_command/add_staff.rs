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
    CreateCommandOption, GuildId, ResolvedOption, RoleId, UserId,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct AddStaffCommand;

impl RegistrableCommand for AddStaffCommand {
    fn name(&self) -> &'static str {
        "addmod"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
            get_translated_message(config, "help.add_staff", None, None, None, None).await
        }.boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();
        let name = self.name();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.add_staff_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            let target_desc = get_translated_message(
                &config,
                "slash_command.add_staff_target_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new(name).description(cmd_desc).add_option(
                    CreateCommandOption::new(CommandOptionType::Mentionable, "target", target_desc)
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

            defer_response(&ctx, &command).await?;

            if !thread_exists_by_channel(command.channel_id, pool).await {
                return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
            }

            let target = resolve_target(&command)?;

            match target {
                AddTarget::User(user_id) => add_single_user(&ctx, &config, &command, user_id).await,
                AddTarget::Role(role_id) => add_role(&ctx, &config, &command, role_id).await,
            }
        })
    }
}

fn resolve_target(command: &CommandInteraction) -> ModmailResult<AddTarget> {
    let opt = command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "target")
        .ok_or_else(|| {
            ModmailError::Command(CommandError::InvalidArguments("target".to_string()))
        })?;

    let id = match &opt.value {
        CommandDataOptionValue::Mentionable(id) => id.get(),
        CommandDataOptionValue::User(user_id) => return Ok(AddTarget::User(*user_id)),
        CommandDataOptionValue::Role(role_id) => return Ok(AddTarget::Role(*role_id)),
        _ => {
            return Err(ModmailError::Command(CommandError::InvalidArguments(
                "target".to_string(),
            )));
        }
    };

    if command.data.resolved.users.contains_key(&UserId::new(id)) {
        Ok(AddTarget::User(UserId::new(id)))
    } else if command.data.resolved.roles.contains_key(&RoleId::new(id)) {
        Ok(AddTarget::Role(RoleId::new(id)))
    } else {
        Err(ModmailError::Command(CommandError::InvalidArguments(
            "target".to_string(),
        )))
    }
}

async fn add_single_user(
    ctx: &Context,
    config: &Config,
    command: &CommandInteraction,
    user_id: UserId,
) -> ModmailResult<()> {
    add_user_to_channel(ctx, command.channel_id, user_id).await?;

    let mut params = HashMap::new();
    params.insert("user".to_string(), format!("<@{}>", user_id));

    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content("add_staff.add_success", Some(&params), None, None)
        .await
        .to_channel(command.channel_id)
        .send_interaction_followup(command, true)
        .await;

    Ok(())
}

async fn add_role(
    ctx: &Context,
    config: &Config,
    command: &CommandInteraction,
    role_id: RoleId,
) -> ModmailResult<()> {
    let guild_id = command
        .guild_id
        .unwrap_or_else(|| GuildId::new(config.bot.get_staff_guild_id()));

    if role_id.get() == guild_id.get() {
        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("add_staff.role_everyone_forbidden", None, None, None)
            .await
            .to_channel(command.channel_id)
            .send_interaction_followup(command, true)
            .await;
        return Ok(());
    }

    let role_mention = format!("<@&{}>", role_id);
    let members = members_with_role(ctx, guild_id, role_id).await?;

    if members.is_empty() {
        let mut params = HashMap::new();
        params.insert("role".to_string(), role_mention);
        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("add_staff.role_no_members", Some(&params), None, None)
            .await
            .to_channel(command.channel_id)
            .send_interaction_followup(command, true)
            .await;
        return Ok(());
    }

    if members.len() > MAX_ROLE_MEMBERS_PER_ADD {
        let mut params = HashMap::new();
        params.insert("role".to_string(), role_mention);
        params.insert("count".to_string(), members.len().to_string());
        params.insert("max".to_string(), MAX_ROLE_MEMBERS_PER_ADD.to_string());
        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("add_staff.role_too_many", Some(&params), None, None)
            .await
            .to_channel(command.channel_id)
            .send_interaction_followup(command, true)
            .await;
        return Ok(());
    }

    let total = members.len();
    let outcome = add_role_members_to_channel(ctx, command.channel_id, members).await;

    let key = if outcome.failed.is_empty() {
        "add_staff.role_add_success"
    } else {
        "add_staff.role_add_partial"
    };

    let mut params = HashMap::new();
    params.insert("role".to_string(), role_mention);
    params.insert("count".to_string(), outcome.added.len().to_string());
    params.insert("added".to_string(), outcome.added.len().to_string());
    params.insert("total".to_string(), total.to_string());
    params.insert("failed".to_string(), outcome.failed.len().to_string());

    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content(key, Some(&params), None, None)
        .await
        .to_channel(command.channel_id)
        .send_interaction_followup(command, true)
        .await;

    Ok(())
}
