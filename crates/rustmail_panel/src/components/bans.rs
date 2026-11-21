use crate::components::forbidden::{CheckingPermissions, Forbidden403};
use crate::components::ui::{
    Avatar, Badge, Button, EmptyState, IconName, Loading, Notice, PageHeader, Tone,
};
use crate::hooks::use_user_permissions;
use crate::i18n::yew::use_translation;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BannedUserDto {
    pub user_id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: Option<i64>,
    pub banned_at: i64,
    pub banned_by_id: Option<String>,
    pub banned_by_name: Option<String>,
    pub ban_reason: Option<String>,
    pub roles_unknown: bool,
}

fn format_day(timestamp: i64) -> Option<String> {
    chrono::DateTime::from_timestamp(timestamp, 0).map(|dt| dt.format("%-d %b %Y").to_string())
}

#[function_component(BansPage)]
pub fn bans_page() -> Html {
    let (i18n, _set_language) = use_translation();
    match use_user_permissions() {
        None => html! { <CheckingPermissions /> },
        Some(perms) if !perms.contains(&PanelPermission::ViewBans) => {
            html! { <Forbidden403 required_permission={i18n.t("navbar.bans")} /> }
        }
        Some(_) => html! { <BansContent /> },
    }
}

#[function_component(BansContent)]
fn bans_content() -> Html {
    let (i18n, _set_language) = use_translation();
    let banned_users = use_state(Vec::<BannedUserDto>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let reload = {
        let banned_users = banned_users.clone();
        let loading = loading.clone();
        let error = error.clone();
        let i18n = i18n.clone();
        Callback::from(move |_: ()| {
            let banned_users = banned_users.clone();
            let loading = loading.clone();
            let error = error.clone();
            let i18n = i18n.clone();
            spawn_local(async move {
                loading.set(true);
                match Request::get("/api/admin/bans").send().await {
                    Ok(resp) if resp.status() == 200 => {
                        match resp.json::<Vec<BannedUserDto>>().await {
                            Ok(users) => {
                                banned_users.set(users);
                                error.set(None);
                            }
                            Err(_) => error.set(Some(i18n.t("panel.bans.error_parse"))),
                        }
                    }
                    Ok(resp) => error.set(Some(format!(
                        "{}: {}",
                        i18n.t("panel.bans.error_load"),
                        resp.status()
                    ))),
                    Err(e) => {
                        error.set(Some(format!("{}: {}", i18n.t("panel.bans.error_load"), e)))
                    }
                }
                loading.set(false);
            });
        })
    };

    {
        let reload = reload.clone();
        use_effect_with((), move |_| {
            reload.emit(());
            || ()
        });
    }

    let description = if banned_users.len() == 1 {
        i18n.t("panel.bans.description_one")
    } else {
        i18n.t("panel.bans.description")
            .replace("{count}", &banned_users.len().to_string())
    };

    let body = if *loading {
        html! { <Loading label={i18n.t("panel.bans.loading")} class="min-h-[40vh]" /> }
    } else if let Some(err) = (*error).clone() {
        html! { <Notice>{err}</Notice> }
    } else if banned_users.is_empty() {
        html! {
            <section class="card">
                <EmptyState
                    icon={IconName::Ban}
                    title={i18n.t("panel.bans.no_bans")}
                    description={i18n.t("panel.bans.no_bans_help")}
                />
            </section>
        }
    } else {
        html! {
            <section class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
                { for banned_users.iter().map(|user| html! {
                    <BanCard key={user.user_id.clone()} user={user.clone()} />
                }) }
            </section>
        }
    };

    html! {
        <>
            <PageHeader
                title={i18n.t("panel.bans.title")}
                description={if *loading || error.is_some() { None } else { Some(AttrValue::from(description)) }}
            >
                <Button icon={IconName::RefreshCw} disabled={*loading} onclick={reload.reform(|_: MouseEvent| ())}>
                    {i18n.t("panel.bans.reload")}
                </Button>
            </PageHeader>
            {body}
        </>
    }
}

#[derive(Properties, PartialEq)]
struct BanCardProps {
    user: BannedUserDto,
}

#[function_component(BanCard)]
fn ban_card(props: &BanCardProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let user = &props.user;
    let unknown = i18n.t("panel.bans.unknown");

    let display_name = user
        .nickname
        .clone()
        .or_else(|| user.global_name.clone())
        .unwrap_or_else(|| user.username.clone());
    let banned_at = format_day(user.banned_at).unwrap_or_else(|| user.banned_at.to_string());
    let joined_at = user
        .joined_at
        .and_then(format_day)
        .unwrap_or_else(|| unknown.clone());
    let banned_by = user
        .banned_by_name
        .clone()
        .or_else(|| user.banned_by_id.clone())
        .unwrap_or_else(|| unknown.clone());
    let reason = user.ban_reason.clone().filter(|r| !r.trim().is_empty());

    let detail = |label: String, value: String| {
        html! {
            <div class="flex min-w-0 flex-col gap-0.5">
                <span class="text-xs text-fg-muted">{label}</span>
                <span class="truncate text-[13px] text-fg">{value}</span>
            </div>
        }
    };

    let roles = if user.roles_unknown {
        html! { <Badge tone={Tone::Warning}>{i18n.t("panel.bans.roles_unknown")}</Badge> }
    } else if user.roles.is_empty() {
        html! { <span class="text-xs text-fg-muted">{i18n.t("panel.bans.no_roles")}</span> }
    } else {
        user.roles
            .iter()
            .map(|role| {
                html! {
                    <span class="inline-flex h-6 items-center gap-0.5 rounded-md border border-line bg-surface-raised px-2 text-xs text-fg">
                        <span class="text-fg-muted">{"@"}</span>
                        {role.clone()}
                    </span>
                }
            })
            .collect::<Html>()
    };

    html! {
        <article class="card flex flex-col gap-4 p-5">
            <div class="flex items-center gap-3">
                <Avatar
                    name={display_name.clone()}
                    url={user.avatar_url.clone().map(AttrValue::from)}
                    size={40}
                    member={true}
                />
                <div class="flex min-w-0 flex-col">
                    <span class="truncate text-sm font-medium text-fg">{display_name}</span>
                    <span class="truncate font-mono text-xs text-fg-muted">{user.user_id.clone()}</span>
                </div>
            </div>
            <div class="grid grid-cols-2 gap-x-4 gap-y-3">
                {detail(i18n.t("panel.bans.label_username"), user.username.clone())}
                {detail(i18n.t("panel.bans.label_banned_by"), banned_by)}
                {detail(i18n.t("panel.bans.label_banned_at"), banned_at)}
                {detail(i18n.t("panel.bans.label_joined_at"), joined_at)}
            </div>
            <div class="flex flex-col gap-1 rounded-lg bg-surface-raised p-3">
                <span class="text-xs text-fg-muted">{i18n.t("panel.bans.label_reason")}</span>
                if let Some(reason) = reason {
                    <p class="break-words text-[13px] text-fg">{reason}</p>
                } else {
                    <p class="text-[13px] text-fg-muted">{i18n.t("panel.bans.no_reason")}</p>
                }
            </div>
            <div class="mt-auto flex flex-wrap items-center gap-1.5">{roles}</div>
        </article>
    }
}
