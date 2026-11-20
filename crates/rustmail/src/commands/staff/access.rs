use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::{
    Context, GuildId, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId, UserId,
};
use std::collections::HashMap;

pub const MAX_ROLE_MEMBERS: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Grant,
    Revoke,
}

impl Access {
    fn overwrite(self, user_id: UserId) -> PermissionOverwrite {
        let permissions = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES;
        let (allow, deny) = match self {
            Access::Grant => (permissions, Permissions::empty()),
            Access::Revoke => (Permissions::empty(), permissions),
        };
        PermissionOverwrite {
            allow,
            deny,
            kind: PermissionOverwriteType::Member(user_id),
        }
    }

    fn removing(self) -> bool {
        self == Access::Revoke
    }

    fn user_success_key(self) -> &'static str {
        match self {
            Access::Grant => "add_staff.add_success",
            Access::Revoke => "add_staff.remove_success",
        }
    }

    fn role_result_key(self, all_succeeded: bool) -> &'static str {
        match (self, all_succeeded) {
            (Access::Grant, true) => "add_staff.role_add_success",
            (Access::Grant, false) => "add_staff.role_add_partial",
            (Access::Revoke, true) => "add_staff.role_remove_success",
            (Access::Revoke, false) => "add_staff.role_remove_partial",
        }
    }

    fn changed_count_param(self) -> &'static str {
        match self {
            Access::Grant => "added",
            Access::Revoke => "removed",
        }
    }
}

pub async fn change_ticket_access(
    access: Access,
    target: StaffTarget,
    invocation: &Invocation,
    config: &Config,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;
    current_ticket(invocation, pool).await?;

    let ctx = invocation.ctx();
    let guild_id = invocation
        .guild_id()
        .unwrap_or_else(|| GuildId::new(config.bot.get_staff_guild_id()));

    match resolve_grantee(ctx, guild_id, target).await? {
        Grantee::Role(role_id) => {
            change_role_access(access, role_id, guild_id, invocation, config).await
        }
        Grantee::User(user_id) => {
            invocation
                .channel_id()
                .create_permission(&ctx.http, access.overwrite(user_id))
                .await?;
            let params = HashMap::from([("user".to_string(), format!("<@{user_id}>"))]);
            invocation
                .reply_translated(config, access.user_success_key(), Some(&params))
                .await?;
            Ok(())
        }
    }
}

enum Grantee {
    User(UserId),
    Role(RoleId),
}

async fn resolve_grantee(
    ctx: &Context,
    guild_id: GuildId,
    target: StaffTarget,
) -> ModmailResult<Grantee> {
    Ok(match target {
        StaffTarget::User(user_id) => Grantee::User(user_id),
        StaffTarget::Role(role_id) => Grantee::Role(role_id),
        StaffTarget::UserOrRole(id) => {
            if guild_has_role(ctx, guild_id, RoleId::new(id)).await? {
                Grantee::Role(RoleId::new(id))
            } else {
                Grantee::User(UserId::new(id))
            }
        }
    })
}

async fn change_role_access(
    access: Access,
    role_id: RoleId,
    guild_id: GuildId,
    invocation: &Invocation,
    config: &Config,
) -> ModmailResult<()> {
    let removing = access.removing();
    if role_id.get() == guild_id.get() {
        return Err(ModmailError::Command(CommandError::StaffRoleIsEveryone {
            removing,
        }));
    }

    let role = format!("<@&{role_id}>");
    let members = members_with_role(invocation.ctx(), guild_id, role_id).await?;
    if members.is_empty() {
        return Err(ModmailError::Command(CommandError::StaffRoleHasNoMembers {
            role,
            removing,
        }));
    }
    if members.len() > MAX_ROLE_MEMBERS {
        return Err(ModmailError::Command(CommandError::StaffRoleTooLarge {
            role,
            count: members.len(),
            max: MAX_ROLE_MEMBERS,
            removing,
        }));
    }

    let total = members.len();
    let changed = apply_to_members(access, invocation, members).await;
    let failed = total - changed;

    let params = HashMap::from([
        ("role".to_string(), role),
        ("count".to_string(), changed.to_string()),
        (
            access.changed_count_param().to_string(),
            changed.to_string(),
        ),
        ("total".to_string(), total.to_string()),
        ("failed".to_string(), failed.to_string()),
    ]);
    invocation
        .reply_translated(config, access.role_result_key(failed == 0), Some(&params))
        .await?;
    Ok(())
}

async fn apply_to_members(access: Access, invocation: &Invocation, members: Vec<UserId>) -> usize {
    let channel_id = invocation.channel_id();
    let mut changed = 0;
    for user_id in members {
        if channel_id
            .create_permission(&invocation.ctx().http, access.overwrite(user_id))
            .await
            .is_ok()
        {
            changed += 1;
        }
    }
    changed
}

async fn guild_has_role(ctx: &Context, guild_id: GuildId, role_id: RoleId) -> ModmailResult<bool> {
    if let Some(guild) = ctx.cache.guild(guild_id) {
        return Ok(guild.roles.contains_key(&role_id));
    }
    let guild = guild_id.to_partial_guild(&ctx.http).await.map_err(|_| {
        ModmailError::Discord(DiscordError::ApiError("Guild not found".to_string()))
    })?;
    Ok(guild.roles.contains_key(&role_id))
}

async fn members_with_role(
    ctx: &Context,
    guild_id: GuildId,
    role_id: RoleId,
) -> ModmailResult<Vec<UserId>> {
    const PAGE_LIMIT: u64 = 1000;

    let mut result = Vec::new();
    let mut after: Option<UserId> = None;

    loop {
        let page = guild_id
            .members(&ctx.http, Some(PAGE_LIMIT), after)
            .await
            .map_err(|_| {
                ModmailError::Discord(DiscordError::ApiError(
                    "Failed to fetch guild members".to_string(),
                ))
            })?;

        let last_id = page.last().map(|m| m.user.id);
        let page_len = page.len();
        result.extend(
            page.into_iter()
                .filter(|member| member.roles.contains(&role_id))
                .map(|member| member.user.id),
        );

        if page_len < PAGE_LIMIT as usize {
            break;
        }
        after = last_id;
    }

    Ok(result)
}
