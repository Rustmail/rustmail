use crate::prelude::config::Config;
use rustmail_types::api::panel_permissions::PanelPermission;
use serenity::all::{GuildId, Http, UserId};
use sqlx::{Pool, Row, Sqlite, query};
use std::sync::Arc;

pub fn get_panel_permissions_cache(
) -> Arc<moka::future::Cache<String, Vec<PanelPermission>>> {
    use std::sync::OnceLock;
    static CACHE: OnceLock<Arc<moka::future::Cache<String, Vec<PanelPermission>>>> =
        OnceLock::new();
    CACHE
        .get_or_init(|| {
            Arc::new(
                moka::future::Cache::builder()
                    .time_to_live(std::time::Duration::from_secs(60))
                    .build(),
            )
        })
        .clone()
}

pub async fn is_super_admin(
    user_id: &str,
    config: &Config,
    guild_id: u64,
    bot_http: Arc<Http>,
) -> bool {
    let user_id_num = match user_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => return false,
    };

    if config.bot.panel_super_admin_users.contains(&user_id_num) {
        return true;
    }

    if !config.bot.panel_super_admin_roles.is_empty() {
        let guild_id_obj = GuildId::new(guild_id);
        let user_id_obj = UserId::new(user_id_num);

        if let Ok(member) = guild_id_obj.member(bot_http, user_id_obj).await {
            for role_id in &member.roles {
                if config
                    .bot
                    .panel_super_admin_roles
                    .contains(&role_id.get())
                {
                    return true;
                }
            }
        }
    }

    false
}

pub async fn get_user_panel_permissions(
    user_id: &str,
    config: &Config,
    guild_id: u64,
    bot_http: Arc<Http>,
    db_pool: &Pool<Sqlite>,
) -> Vec<PanelPermission> {
    let cache = get_panel_permissions_cache();
    if let Some(perms) = cache.get(user_id).await {
        return perms;
    }

    let mut permissions = Vec::new();

    if is_super_admin(user_id, config, guild_id, bot_http.clone()).await {
        permissions = vec![
            PanelPermission::ViewPanel,
            PanelPermission::ManageBot,
            PanelPermission::ManageConfig,
            PanelPermission::ManageTickets,
            PanelPermission::ManageApiKeys,
            PanelPermission::ManagePermissions,
        ];
        cache
            .insert(user_id.to_string(), permissions.clone())
            .await;
        return permissions;
    }

    if crate::api::utils::is_admin_or_owner(user_id, guild_id, bot_http.clone()).await {
        permissions = vec![
            PanelPermission::ViewPanel,
            PanelPermission::ManageBot,
            PanelPermission::ManageConfig,
            PanelPermission::ManageTickets,
        ];
    }

    if let Ok(rows) = query(
        "SELECT permission FROM panel_permissions WHERE subject_type = 'user' AND subject_id = ?",
    )
    .bind(user_id)
    .fetch_all(db_pool)
    .await
    {
        for row in rows {
            if let Ok(perm_str) = row.try_get::<String, _>("permission") {
                if let Some(perm) = PanelPermission::from_str(&perm_str) {
                    if !permissions.contains(&perm) {
                        permissions.push(perm);
                    }
                }
            }
        }
    }

    let user_id_num = match user_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            cache
                .insert(user_id.to_string(), permissions.clone())
                .await;
            return permissions;
        }
    };

    let guild_id_obj = GuildId::new(guild_id);
    let user_id_obj = UserId::new(user_id_num);

    if let Ok(member) = guild_id_obj.member(bot_http, user_id_obj).await {
        for role_id in &member.roles {
            let role_id_str = role_id.get().to_string();
            if let Ok(rows) = query(
                "SELECT permission FROM panel_permissions WHERE subject_type = 'role' AND subject_id = ?",
            )
            .bind(&role_id_str)
            .fetch_all(db_pool)
            .await
            {
                for row in rows {
                    if let Ok(perm_str) = row.try_get::<String, _>("permission") {
                        if let Some(perm) = PanelPermission::from_str(&perm_str) {
                            if !permissions.contains(&perm) {
                                permissions.push(perm);
                            }
                        }
                    }
                }
            }
        }
    }

    cache
        .insert(user_id.to_string(), permissions.clone())
        .await;
    permissions
}

pub async fn has_panel_permission(
    user_id: &str,
    permission: PanelPermission,
    config: &Config,
    guild_id: u64,
    bot_http: Arc<Http>,
    db_pool: &Pool<Sqlite>,
) -> bool {
    let permissions =
        get_user_panel_permissions(user_id, config, guild_id, bot_http, db_pool).await;
    permissions.contains(&permission)
}
