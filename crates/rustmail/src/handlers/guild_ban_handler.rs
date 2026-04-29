use crate::db::repr::{BannedUser, TrackedMember};
use crate::prelude::config::*;
use crate::prelude::db::*;
use chrono::Utc;
use serenity::all::audit_log::Action;
use serenity::all::{Context, EventHandler, GuildId, Member, MemberAction, User, UserId};
use serenity::async_trait;
use sqlx::SqlitePool;

pub struct GuildBanHandler {
    pub config: Config,
}

impl GuildBanHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    fn pool(&self) -> Option<&SqlitePool> {
        self.config.db_pool.as_ref()
    }

    fn should_track(&self, guild_id: GuildId) -> bool {
        self.config.bot.is_community_guild(guild_id.get())
    }
}

fn member_to_tracked(member: &Member, now: i64) -> TrackedMember {
    TrackedMember {
        guild_id: member.guild_id.to_string(),
        user_id: member.user.id.to_string(),
        username: member.user.name.clone(),
        global_name: member.user.global_name.as_ref().map(|s| s.to_string()),
        nickname: member.nick.clone(),
        avatar_url: Some(member.user.face()),
        roles: member.roles.iter().map(|r| r.to_string()).collect(),
        joined_at: member.joined_at.map(|t| t.unix_timestamp()),
        first_seen_at: now,
        last_seen_at: now,
    }
}

async fn record_member(pool: &SqlitePool, member: &Member) {
    let now = Utc::now().timestamp();
    let tracked = member_to_tracked(member, now);
    if let Err(e) = upsert_tracked_member(&tracked, pool).await {
        eprintln!("Failed to track member {}: {:?}", member.user.id, e);
    }
}

async fn fetch_ban_audit(
    ctx: &Context,
    guild_id: GuildId,
    target_user_id: u64,
) -> (Option<String>, Option<String>) {
    let entries = match guild_id
        .audit_logs(
            &ctx.http,
            Some(Action::Member(MemberAction::BanAdd)),
            None,
            None,
            Some(10),
        )
        .await
    {
        Ok(logs) => logs,
        Err(_) => return (None, None),
    };

    for entry in entries.entries.iter() {
        if let Some(target) = entry.target_id {
            if target.get() == target_user_id {
                return (Some(entry.user_id.to_string()), entry.reason.clone());
            }
        }
    }
    (None, None)
}

#[async_trait]
impl EventHandler for GuildBanHandler {
    async fn guild_member_addition(&self, _ctx: Context, new_member: Member) {
        if !self.should_track(new_member.guild_id) {
            return;
        }
        let Some(pool) = self.pool() else {
            return;
        };
        record_member(pool, &new_member).await;
    }

    async fn guild_member_update(
        &self,
        _ctx: Context,
        _old: Option<Member>,
        new: Option<Member>,
        _event: serenity::all::GuildMemberUpdateEvent,
    ) {
        let Some(new_member) = new else {
            return;
        };
        if !self.should_track(new_member.guild_id) {
            return;
        }
        let Some(pool) = self.pool() else {
            return;
        };
        record_member(pool, &new_member).await;
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        if !self.should_track(guild_id) {
            return;
        }
        let Some(pool) = self.pool() else {
            return;
        };

        let guild_id_str = guild_id.to_string();
        let user_id_str = banned_user.id.to_string();
        let now = Utc::now().timestamp();

        let tracked = get_tracked_member(&guild_id_str, &user_id_str, pool)
            .await
            .unwrap_or(None);

        let (banned_by, ban_reason) = fetch_ban_audit(&ctx, guild_id, banned_user.id.get()).await;

        let record = match tracked {
            Some(t) => BannedUser {
                guild_id: t.guild_id,
                user_id: t.user_id,
                username: t.username,
                global_name: t.global_name,
                nickname: t.nickname,
                avatar_url: t.avatar_url,
                roles: t.roles,
                joined_at: t.joined_at,
                banned_at: now,
                banned_by,
                ban_reason,
                roles_unknown: false,
            },
            None => BannedUser {
                guild_id: guild_id_str.clone(),
                user_id: user_id_str.clone(),
                username: banned_user.name.clone(),
                global_name: banned_user.global_name.as_ref().map(|s| s.to_string()),
                nickname: None,
                avatar_url: Some(banned_user.face()),
                roles: Vec::new(),
                joined_at: None,
                banned_at: now,
                banned_by,
                ban_reason,
                roles_unknown: true,
            },
        };

        match save_banned_user(&record, pool).await {
            Ok(()) => {
                if let Err(e) = delete_tracked_member(&guild_id_str, &user_id_str, pool).await {
                    eprintln!(
                        "Failed to delete tracked member {} from guild {} after saving banned user: {:?}",
                        banned_user.id, guild_id, e
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to save banned user {}: {:?}", banned_user.id, e);
            }
        }
    }
}

pub async fn backfill_tracked_members(
    ctx: &Context,
    config: &Config,
    shutdown: &mut tokio::sync::watch::Receiver<bool>,
) {
    const PAGE_LIMIT: u64 = 1000;

    let Some(pool) = config.db_pool.as_ref() else {
        return;
    };

    let guild_id = GuildId::new(config.bot.get_community_guild_id());
    let now = Utc::now().timestamp();

    let mut after: Option<UserId> = None;
    let mut total: usize = 0;

    loop {
        if *shutdown.borrow() {
            println!("Tracked member backfill cancelled due to shutdown.");
            return;
        }

        let page = match guild_id.members(&ctx.http, Some(PAGE_LIMIT), after).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to fetch guild members for backfill: {:?}", e);
                return;
            }
        };

        if page.is_empty() {
            break;
        }

        let last_id = page.last().map(|m| m.user.id);
        let page_len = page.len();

        let tracked_batch: Vec<TrackedMember> =
            page.iter().map(|m| member_to_tracked(m, now)).collect();

        match bulk_upsert_tracked_members(&tracked_batch, pool).await {
            Ok(()) => total += page_len,
            Err(e) => {
                eprintln!(
                    "Failed to backfill tracked members page in guild {}: {:?}",
                    guild_id, e
                );
            }
        }

        if page_len < PAGE_LIMIT as usize {
            break;
        }

        after = last_id;
    }

    println!(
        "Backfilled {} tracked members for community guild {}",
        total, guild_id
    );
}
