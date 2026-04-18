use crate::components::forbidden::Forbidden403;
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
    pub banned_by: Option<String>,
    pub ban_reason: Option<String>,
    pub roles_unknown: bool,
}

#[function_component(BansPage)]
pub fn bans_page() -> Html {
    let (i18n, _set_language) = use_translation();

    let permissions = use_state(|| None::<Vec<PanelPermission>>);
    {
        let permissions = permissions.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/user/permissions").send().await {
                    if let Ok(perms) = resp.json::<Vec<PanelPermission>>().await {
                        permissions.set(Some(perms));
                    }
                }
            });
            || ()
        });
    }

    if let Some(perms) = (*permissions).as_ref() {
        if !perms.contains(&PanelPermission::ViewBans) {
            return html! {
                <Forbidden403 required_permission={i18n.t("navbar.bans")} />
            };
        }
    } else {
        return html! {
            <div class="flex items-center justify-center min-h-[70vh]">
                <div class="text-gray-400 animate-pulse">{i18n.t("panel.forbidden.checking_permissions")}</div>
            </div>
        };
    }

    let banned_users = use_state(|| Vec::<BannedUserDto>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let reload = {
        let banned_users = banned_users.clone();
        let loading = loading.clone();
        let error = error.clone();
        let i18n = i18n.clone();
        Callback::from(move |_| {
            let banned_users = banned_users.clone();
            let loading = loading.clone();
            let error = error.clone();
            let i18n = i18n.clone();
            spawn_local(async move {
                loading.set(true);
                match Request::get("/api/admin/bans").send().await {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            if let Ok(users) = resp.json::<Vec<BannedUserDto>>().await {
                                banned_users.set(users);
                                error.set(None);
                            } else {
                                error.set(Some(i18n.t("panel.bans.error_parse")));
                            }
                        } else {
                            error.set(Some(format!(
                                "{}: {}",
                                i18n.t("panel.bans.error_load"),
                                resp.status()
                            )));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("{}: {}", i18n.t("panel.bans.error_load"), e)));
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

    html! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <h1 class="text-3xl font-bold text-white">{i18n.t("panel.bans.title")}</h1>
                <button
                    onclick={reload.clone().reform(|_| ())}
                    class="p-2 bg-slate-800 hover:bg-slate-700 text-gray-300 rounded-full transition"
                    title={i18n.t("panel.bans.reload")}
                >
                    <svg class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                    </svg>
                </button>
            </div>

            {
                if *loading {
                    html! {
                        <div class="text-center text-gray-400 py-8">
                            <p class="animate-pulse">{i18n.t("panel.bans.loading")}</p>
                        </div>
                    }
                } else if let Some(err) = (*error).clone() {
                    html! {
                        <div class="bg-red-900/20 border border-red-500 text-red-200 p-4 rounded-md">{err}</div>
                    }
                } else if banned_users.is_empty() {
                    html! {
                        <div class="bg-slate-800 rounded-lg p-8 text-center border border-slate-700">
                            <p class="text-gray-400">{i18n.t("panel.bans.no_bans")}</p>
                        </div>
                    }
                } else {
                    html! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                            {
                                banned_users.iter().map(|user| {
                                    html! {
                                        <BanCard key={user.user_id.clone()} user={user.clone()} />
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    }
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct BanCardProps {
    user: BannedUserDto,
}

#[function_component(BanCard)]
fn ban_card(props: &BanCardProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let u = &props.user;

    let banned_at = chrono::DateTime::from_timestamp(u.banned_at, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| u.banned_at.to_string());

    let joined_at = u
        .joined_at
        .and_then(|t| {
            chrono::DateTime::from_timestamp(t, 0).map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        })
        .unwrap_or_else(|| i18n.t("panel.bans.unknown"));

    html! {
        <div class="bg-slate-800 rounded-lg p-5 border border-slate-700 hover:border-slate-500 transition flex flex-col h-full">
            <div class="flex items-center space-x-4 mb-4">
                <img
                    src={u.avatar_url.clone().unwrap_or_else(|| "https://cdn.discordapp.com/embed/avatars/0.png".to_string())}
                    alt="Avatar"
                    class="h-12 w-12 rounded-full bg-slate-900 border border-slate-700"
                />
                <div class="overflow-hidden">
                    <h3 class="text-lg font-semibold text-white truncate">
                        { u.nickname.clone().unwrap_or_else(|| u.global_name.clone().unwrap_or_else(|| u.username.clone())) }
                    </h3>
                    <p class="text-xs text-gray-500 font-mono truncate">{&u.user_id}</p>
                </div>
            </div>

            <div class="flex-grow space-y-3">
                <div class="grid grid-cols-2 gap-2 text-sm">
                    <div>
                        <p class="text-gray-500">{i18n.t("panel.bans.label_username")}</p>
                        <p class="text-gray-300 truncate">{&u.username}</p>
                    </div>
                    <div>
                        <p class="text-gray-500">{i18n.t("panel.bans.label_banned_at")}</p>
                        <p class="text-gray-300">{banned_at}</p>
                    </div>
                    <div>
                        <p class="text-gray-500">{i18n.t("panel.bans.label_joined_at")}</p>
                        <p class="text-gray-300">{joined_at}</p>
                    </div>
                    <div>
                        <p class="text-gray-500">{i18n.t("panel.bans.label_banned_by")}</p>
                        <p class="text-gray-300 truncate">{u.banned_by.clone().unwrap_or_else(|| i18n.t("panel.bans.unknown"))}</p>
                    </div>
                </div>

                <div>
                    <p class="text-sm text-gray-500 mb-1">{i18n.t("panel.bans.label_reason")}</p>
                    <div class="bg-slate-900 rounded p-2 text-sm text-gray-300 min-h-[3rem] italic border border-slate-700/50">
                        { u.ban_reason.clone().unwrap_or_else(|| i18n.t("panel.bans.no_reason")) }
                    </div>
                </div>

                <div>
                    <p class="text-sm text-gray-500 mb-1">{i18n.t("panel.bans.label_roles")}</p>
                    <div class="flex flex-wrap gap-1">
                        {
                            if u.roles_unknown {
                                html! { <span class="px-2 py-0.5 bg-yellow-900/30 text-yellow-500 border border-yellow-800 rounded text-xs">{i18n.t("panel.bans.roles_unknown")}</span> }
                            } else if u.roles.is_empty() {
                                html! { <span class="text-xs text-gray-600 italic">{i18n.t("panel.bans.no_roles")}</span> }
                            } else {
                                u.roles.iter().map(|role| {
                                    html! {
                                        <span class="px-2 py-0.5 bg-blue-900/20 text-blue-400 border border-blue-800 rounded text-xs">
                                            {role}
                                        </span>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
