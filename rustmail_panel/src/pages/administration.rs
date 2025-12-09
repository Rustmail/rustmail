use crate::components::forbidden::Forbidden403;
use crate::components::navbar::RustmailNavbar;
use crate::i18n::yew::use_translation;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub user_id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub role_id: String,
    pub name: String,
    pub color: u32,
    pub position: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionEntry {
    pub id: i64,
    pub subject_type: String,
    pub subject_id: String,
    pub permission: PanelPermission,
    pub granted_by: String,
    pub granted_at: i64,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct UserAvatar {
    pub avatar_url: Option<String>,
}

#[function_component(Administration)]
pub fn administration() -> Html {
    let (i18n, _set_language) = use_translation();
    let members = use_state(|| Vec::<Member>::new());
    let roles = use_state(|| Vec::<Role>::new());
    let permissions = use_state(|| Vec::<PermissionEntry>::new());
    let loading = use_state(|| true);
    let avatar = use_state(|| None::<String>);

    let selected_subject_type = use_state(|| "user".to_string());
    let selected_subject_id = use_state(|| String::new());
    let selected_permissions = use_state(|| Vec::<String>::new());
    let editing_id = use_state(|| None::<i64>);
    let adding_to_subject = use_state(|| None::<(String, String)>);

    {
        let avatar = avatar.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/user/avatar").send().await {
                    if let Ok(user_avatar) = resp.json::<UserAvatar>().await {
                        avatar.set(user_avatar.avatar_url);
                    }
                }
            });
        });
    }

    let user_permissions = use_state(|| None::<Vec<PanelPermission>>);
    {
        let user_permissions = user_permissions.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/user/permissions").send().await {
                    if let Ok(perms) = resp.json::<Vec<PanelPermission>>().await {
                        user_permissions.set(Some(perms));
                    }
                }
            });
        });
    }

    let avatar_url = (*avatar).clone().unwrap_or_default();
    let has_permission = if let Some(perms) = (*user_permissions).as_ref() {
        perms.contains(&PanelPermission::ManagePermissions)
    } else {
        false
    };

    if user_permissions.is_none() {
        return html! {
            <>
                <RustmailNavbar avatar_url={avatar_url.clone()} permissions={vec![]} />
                <section class="pt-24 min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                    <div class="flex items-center justify-center min-h-[70vh]">
                        <div class="text-gray-400 text-lg animate-pulse">{i18n.t("panel.forbidden.checking_permissions")}</div>
                    </div>
                </section>
            </>
        };
    }

    if !has_permission {
        return html! {
            <>
                <RustmailNavbar avatar_url={avatar_url.clone()} permissions={user_permissions.as_ref().unwrap().clone()} />
                <section class="pt-24 min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                    <Forbidden403 required_permission={i18n.t("navbar.administration")} />
                </section>
            </>
        };
    }

    {
        let members = members.clone();
        let roles = roles.clone();
        let permissions = permissions.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/admin/members").send().await {
                    if let Ok(data) = resp.json::<Vec<Member>>().await {
                        members.set(data);
                    }
                }

                if let Ok(resp) = Request::get("/api/admin/roles").send().await {
                    if let Ok(data) = resp.json::<Vec<Role>>().await {
                        roles.set(data);
                    }
                }

                if let Ok(resp) = Request::get("/api/admin/permissions").send().await {
                    if let Ok(data) = resp.json::<Vec<PermissionEntry>>().await {
                        permissions.set(data);
                    }
                }

                loading.set(false);
            });
        });
    }

    let perms = (*user_permissions).as_ref().unwrap().clone();

    let get_subject_display = |subject_type: &str, subject_id: &str| -> String {
        if subject_type == "user" {
            members
                .iter()
                .find(|m| m.user_id == subject_id)
                .map(|m| format!("{}#{}", m.username, m.discriminator))
                .unwrap_or_else(|| subject_id.to_string())
        } else {
            roles
                .iter()
                .find(|r| r.role_id == subject_id)
                .map(|r| r.name.clone())
                .unwrap_or_else(|| subject_id.to_string())
        }
    };

    use std::collections::HashMap;
    let mut grouped_permissions: HashMap<(String, String), Vec<(i64, PanelPermission)>> =
        HashMap::new();
    for perm in permissions.iter() {
        let key = (perm.subject_type.clone(), perm.subject_id.clone());
        grouped_permissions
            .entry(key)
            .or_insert_with(Vec::new)
            .push((perm.id, perm.permission.clone()));
    }

    html! {
        <>
            <RustmailNavbar avatar_url={avatar_url.clone()} permissions={perms.clone()} />
            <section class="pt-24 min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                {if *loading {
                    html! {
                        <div class="flex flex-col items-center justify-center min-h-[80vh]">
                            <div class="relative">
                                <div class="w-20 h-20 border-4 border-slate-700 border-t-blue-500 rounded-full animate-spin"></div>
                                <div class="absolute inset-0 w-20 h-20 border-4 border-transparent border-t-purple-500 rounded-full animate-spin" style="animation-duration: 1.5s;"></div>
                            </div>
                            <p class="mt-6 text-gray-400 text-lg font-medium animate-pulse">{i18n.t("panel.administration.loading")}</p>
                            <div class="mt-4 flex space-x-2">
                                <div class="w-2 h-2 bg-blue-500 rounded-full animate-bounce" style="animation-delay: 0ms;"></div>
                                <div class="w-2 h-2 bg-purple-500 rounded-full animate-bounce" style="animation-delay: 150ms;"></div>
                                <div class="w-2 h-2 bg-pink-500 rounded-full animate-bounce" style="animation-delay: 300ms;"></div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="container mx-auto p-6 max-w-7xl">
                            <div class="mb-8">
                                <h1 class="text-4xl font-bold text-white mb-2">{i18n.t("panel.administration.title")}</h1>
                                <p class="text-gray-400">{i18n.t("panel.administration.description")}</p>
                            </div>

            <div class="bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl shadow-2xl p-8 border border-slate-700 mb-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center">
                        <div class={format!("p-3 rounded-lg mr-4 {}", if editing_id.is_some() { "bg-orange-500/10" } else { "bg-blue-500/10" })}>
                            <svg class={format!("w-6 h-6 {}", if editing_id.is_some() { "text-orange-400" } else { "text-blue-400" })} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                {if editing_id.is_some() {
                                    html! {
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                    }
                                } else {
                                    html! {
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                                    }
                                }}
                            </svg>
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold text-white">
                                {i18n.t("panel.administration.grant_permission")}
                            </h2>
                            <p class="text-gray-400 text-sm">
                                {i18n.t("panel.administration.description")}
                            </p>
                        </div>
                    </div>
                    {if editing_id.is_some() {
                        html! {
                            <button
                                onclick={{
                                    let editing_id = editing_id.clone();
                                    let selected_subject_type = selected_subject_type.clone();
                                    let selected_subject_id = selected_subject_id.clone();
                                    let selected_permissions = selected_permissions.clone();
                                    Callback::from(move |_| {
                                        editing_id.set(None);
                                        selected_subject_type.set("user".to_string());
                                        selected_subject_id.set(String::new());
                                        selected_permissions.set(Vec::new());
                                    })
                                }}
                                class="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg transition"
                            >
                                {i18n.t("panel.administration.cancel")}
                            </button>
                        }
                    } else {
                        html! {}
                    }}
                </div>
                <div class="space-y-4">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                        <label class="block text-sm font-semibold text-gray-300 mb-2">{i18n.t("panel.administration.subject_type")}</label>
                        <select
                            class="w-full px-4 py-3 bg-slate-900/50 border border-slate-600 rounded-lg text-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                            value={(*selected_subject_type).clone()}
                            onchange={{
                                let selected_subject_type = selected_subject_type.clone();
                                let selected_subject_id = selected_subject_id.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
                                    if let Some(select) = target {
                                        selected_subject_type.set(select.value());
                                        selected_subject_id.set(String::new());
                                    }
                                })
                            }}
                        >
                            <option value="user">{format!("👤 {}", i18n.t("panel.administration.user"))}</option>
                            <option value="role">{format!("🏷️ {}", i18n.t("panel.administration.role"))}</option>
                        </select>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-gray-300 mb-2">
                            {if *selected_subject_type == "user" { i18n.t("panel.administration.user") } else { i18n.t("panel.administration.role") }}
                        </label>
                        <select
                            class="w-full px-4 py-3 bg-slate-900/50 border border-slate-600 rounded-lg text-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                            value={(*selected_subject_id).clone()}
                            onchange={{
                                let selected_subject_id = selected_subject_id.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
                                    if let Some(select) = target {
                                        selected_subject_id.set(select.value());
                                    }
                                })
                            }}
                        >
                            <option value="">{if *selected_subject_type == "user" { i18n.t("panel.administration.select_user") } else { i18n.t("panel.administration.select_role") }}</option>
                            {if *selected_subject_type == "user" {
                                members.iter().map(|member| html! {
                                    <option key={member.user_id.clone()} value={member.user_id.clone()}>
                                        {format!("{}#{}", member.username, member.discriminator)}
                                    </option>
                                }).collect::<Html>()
                            } else {
                                roles.iter().map(|role| html! {
                                    <option key={role.role_id.clone()} value={role.role_id.clone()}>
                                        {&role.name}
                                    </option>
                                }).collect::<Html>()
                            }}
                        </select>
                    </div>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-gray-300 mb-3">{i18n.t("panel.administration.select_permissions")}</label>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
                            {
                                vec![
                                    ("view_panel", i18n.t("panel.administration.permissions.view_panel")),
                                    ("manage_bot", i18n.t("panel.administration.permissions.manage_bot")),
                                    ("manage_config", i18n.t("panel.administration.permissions.manage_config")),
                                    ("manage_tickets", i18n.t("panel.administration.permissions.manage_tickets")),
                                    ("manage_api_keys", i18n.t("panel.administration.permissions.manage_api_keys")),
                                    ("manage_permissions", i18n.t("panel.administration.permissions.manage_permissions")),
                                ].iter().map(|(value, label)| {
                                let is_checked = selected_permissions.contains(&value.to_string());
                                let selected_permissions_clone = selected_permissions.clone();
                                let value_str = value.to_string();

                                html! {
                                    <label
                                        key={value.to_string()}
                                        class={format!("flex items-center p-3 rounded-lg border-2 cursor-pointer transition-all {}",
                                            if is_checked {
                                                "border-blue-500 bg-blue-500/10"
                                            } else {
                                                "border-slate-600 bg-slate-900/50 hover:border-slate-500"
                                            }
                                        )}
                                    >
                                        <input
                                            type="checkbox"
                                            checked={is_checked}
                                            onchange={{
                                                Callback::from(move |e: Event| {
                                                    let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                                    if let Some(checkbox) = target {
                                                        let mut perms = (*selected_permissions_clone).clone();
                                                        if checkbox.checked() {
                                                            if !perms.contains(&value_str) {
                                                                perms.push(value_str.clone());
                                                            }
                                                        } else {
                                                            perms.retain(|p| p != &value_str);
                                                        }
                                                        selected_permissions_clone.set(perms);
                                                    }
                                                })
                                            }}
                                            class="w-4 h-4 text-blue-600 bg-slate-800 border-slate-600 rounded focus:ring-blue-500"
                                        />
                                        <span class="ml-2 text-sm font-medium text-gray-200">{label}</span>
                                    </label>
                                }
                            }).collect::<Html>()
                            }
                        </div>
                    </div>

                    <div>
                        <button
                            disabled={(*selected_subject_id).is_empty() || (*selected_permissions).is_empty()}
                            onclick={{
                                let permissions_state = permissions.clone();
                                let subject_type = selected_subject_type.clone();
                                let subject_id = selected_subject_id.clone();
                                let perms_list = selected_permissions.clone();
                                let edit_id = editing_id.clone();
                                let reset_subject_type = selected_subject_type.clone();
                                let reset_subject_id = selected_subject_id.clone();
                                let reset_permissions = selected_permissions.clone();
                                let reset_editing = editing_id.clone();
                                Callback::from(move |_| {
                                    let permissions = permissions_state.clone();
                                    let st = (*subject_type).clone();
                                    let sid = (*subject_id).clone();
                                    let perms = (*perms_list).clone();
                                    let eid = (*edit_id).clone();
                                    let reset_st = reset_subject_type.clone();
                                    let reset_sid = reset_subject_id.clone();
                                    let reset_perms = reset_permissions.clone();
                                    let reset_edit = reset_editing.clone();

                                    if sid.is_empty() || perms.is_empty() {
                                        return;
                                    }

                                    spawn_local(async move {
                                        let mut success = true;

                                        if let Some(id) = eid {
                                            if Request::delete(&format!("/api/admin/permissions/{}", id)).send().await.is_err() {
                                                success = false;
                                            }
                                        }

                                        if success {
                                            for perm in &perms {
                                                let body = serde_json::json!({
                                                    "subject_type": st,
                                                    "subject_id": sid,
                                                    "permission": perm
                                                });

                                                let req = Request::post("/api/admin/permissions")
                                                    .header("Content-Type", "application/json")
                                                    .body(body.to_string());

                                                if let Ok(request) = req {
                                                    if request.send().await.is_err() {
                                                        success = false;
                                                        break;
                                                    }
                                                } else {
                                                    success = false;
                                                    break;
                                                }
                                            }
                                        }

                                        if success {
                                            if let Ok(resp) = Request::get("/api/admin/permissions").send().await {
                                                if let Ok(data) = resp.json::<Vec<PermissionEntry>>().await {
                                                    permissions.set(data);
                                                    if eid.is_some() {
                                                        reset_st.set("user".to_string());
                                                        reset_sid.set(String::new());
                                                        reset_perms.set(Vec::new());
                                                    } else {
                                                        reset_perms.set(Vec::new());
                                                    }
                                                    reset_edit.set(None);
                                                }
                                            }
                                        }
                                    });
                                })
                            }}
                            class={format!("w-full px-4 py-3 bg-gradient-to-r {} disabled:from-gray-600 disabled:to-gray-700 disabled:cursor-not-allowed text-white font-semibold rounded-lg transition-all duration-200 shadow-lg hover:shadow-xl flex items-center justify-center space-x-2",
                                if editing_id.is_some() { "from-orange-600 to-orange-700 hover:from-orange-700 hover:to-orange-800" } else { "from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800" }
                            )}
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                {if editing_id.is_some() {
                                    html! {
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                    }
                                } else {
                                    html! {
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                                    }
                                }}
                            </svg>
                            <span>
                                {
                                    if (*selected_permissions).is_empty() {
                                        i18n.t("panel.administration.grant_button")
                                    } else {
                                        i18n.t("panel.administration.grant_button_count").replace("{count}", &(*selected_permissions).len().to_string())
                                    }
                                }
                            </span>
                        </button>
                    </div>
                </div>
            </div>

            <div class="bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl shadow-2xl border border-slate-700 overflow-hidden">
                <div class="p-6 border-b border-slate-700">
                    <div class="flex items-center">
                        <div class="bg-purple-500/10 p-3 rounded-lg mr-4">
                            <svg class="w-6 h-6 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
                            </svg>
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold text-white">{i18n.t("panel.administration.active_permissions")}</h2>
                            <p class="text-gray-400 text-sm">{format!("{} permission(s)", permissions.len())}</p>
                        </div>
                    </div>
                </div>
                <div class="p-6">
                    {if grouped_permissions.is_empty() {
                        html! {
                            <div class="text-center py-12">
                                <svg class="mx-auto h-12 w-12 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"></path>
                                </svg>
                                <h3 class="mt-2 text-sm font-medium text-gray-300">{i18n.t("panel.administration.no_permissions")}</h3>
                                <p class="mt-1 text-sm text-gray-500">{i18n.t("panel.administration.description")}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="space-y-4">
                                {grouped_permissions.iter().map(|((subject_type, subject_id), perms_list)| {
                                    let subject_display = get_subject_display(subject_type, subject_id);
                                    let is_user = subject_type == "user";
                                    let permissions_clone = permissions.clone();
                                    let st = subject_type.clone();
                                    let sid = subject_id.clone();

                                    let is_adding = if let Some((s_type, s_id)) = adding_to_subject.as_ref() {
                                        s_type == &st && s_id == &sid
                                    } else {
                                        false
                                    };
                                    let all_perms = vec![
                                        ("view_panel", i18n.t("panel.administration.permissions.view_panel")),
                                        ("manage_bot", i18n.t("panel.administration.permissions.manage_bot")),
                                        ("manage_config", i18n.t("panel.administration.permissions.manage_config")),
                                        ("manage_tickets", i18n.t("panel.administration.permissions.manage_tickets")),
                                        ("manage_api_keys", i18n.t("panel.administration.permissions.manage_api_keys")),
                                        ("manage_permissions", i18n.t("panel.administration.permissions.manage_permissions")),
                                    ];
                                    let current_perms: Vec<String> = perms_list.iter().map(|(_, p)| {
                                        match p {
                                            PanelPermission::ViewPanel => "view_panel",
                                            PanelPermission::ManageBot => "manage_bot",
                                            PanelPermission::ManageConfig => "manage_config",
                                            PanelPermission::ManageTickets => "manage_tickets",
                                            PanelPermission::ManageApiKeys => "manage_api_keys",
                                            PanelPermission::ManagePermissions => "manage_permissions",
                                        }.to_string()
                                    }).collect();

                                    html! {
                                        <div key={format!("{}_{}", subject_type, subject_id)} class="bg-slate-800/50 rounded-lg p-5 border border-slate-700">
                                            <div class="flex items-start space-x-4">
                                                <div class={format!("p-2 rounded-lg {}", if is_user { "bg-blue-500/10" } else { "bg-purple-500/10" })}>
                                                    {if is_user {
                                                        html! {
                                                            <svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                                            </svg>
                                                        }
                                                    } else {
                                                        html! {
                                                            <svg class="w-6 h-6 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                                                            </svg>
                                                        }
                                                    }}
                                                </div>
                                                <div class="flex-1">
                                                    <div class="flex items-center space-x-2 mb-3">
                                                        <span class="font-bold text-white text-lg">{subject_display}</span>
                                                        <span class="px-2 py-0.5 text-xs rounded-full bg-slate-700 text-gray-300">
                                                            {if is_user { i18n.t("panel.administration.user") } else { i18n.t("panel.administration.role") }}
                                                        </span>
                                                    </div>
                                                    <div class="flex flex-wrap gap-2">
                                                        {perms_list.iter().map(|(perm_id, perm)| {
                                                            let pid = *perm_id;
                                                            let permissions_clone2 = permissions_clone.clone();
                                                            let remove_callback = Callback::from(move |_| {
                                                                let permissions = permissions_clone2.clone();
                                                                spawn_local(async move {
                                                                    if Request::delete(&format!("/api/admin/permissions/{}", pid)).send().await.is_ok() {
                                                                        if let Ok(resp) = Request::get("/api/admin/permissions").send().await {
                                                                            if let Ok(data) = resp.json::<Vec<PermissionEntry>>().await {
                                                                                permissions.set(data);
                                                                            }
                                                                        }
                                                                    }
                                                                });
                                                            });

                                                            html! {
                                                                <div key={pid} class="inline-flex items-center space-x-1 px-3 py-1.5 bg-green-500/10 border border-green-500/30 rounded-full text-green-400 text-sm font-medium">
                                                                    <span>{perm.to_display()}</span>
                                                                    <button
                                                                        onclick={remove_callback}
                                                                        class="ml-1 hover:bg-red-500/20 rounded-full p-0.5 transition"
                                                                    >
                                                                        <svg class="w-3.5 h-3.5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                                                        </svg>
                                                                    </button>
                                                                </div>
                                                            }
                                                        }).collect::<Html>()}

                                                        {html! {
                                                                <>
                                                                    {if is_adding {
                                                                        html! {
                                                                            <div class="w-full mt-2 p-3 bg-slate-900/80 rounded-lg border border-slate-600">
                                                                                <div class="flex items-center justify-between mb-2">
                                                                                    <span class="text-sm font-semibold text-gray-300">{i18n.t("panel.administration.available_permissions")}</span>
                                                                                    <button
                                                                                        onclick={{
                                                                                            let adding_to_subject = adding_to_subject.clone();
                                                                                            Callback::from(move |_| {
                                                                                                adding_to_subject.set(None);
                                                                                            })
                                                                                        }}
                                                                                        class="text-gray-400 hover:text-gray-300"
                                                                                    >
                                                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                                                                        </svg>
                                                                                    </button>
                                                                                </div>
                                                                                <div class="space-y-1">
                                                                                    {all_perms.iter().filter(|(value, _)| !current_perms.contains(&value.to_string())).map(|(value, label)| {
                                                                                        let permissions_clone4 = permissions_clone.clone();
                                                                                        let adding_to_subject_clone = adding_to_subject.clone();
                                                                                        let st_clone2 = st.clone();
                                                                                        let sid_clone2 = sid.clone();
                                                                                        let value_str = value.to_string();

                                                                                        html! {
                                                                                            <button
                                                                                                key={value.to_string()}
                                                                                                onclick={{
                                                                                                    Callback::from(move |_| {
                                                                                                        let permissions = permissions_clone4.clone();
                                                                                                        let adding = adding_to_subject_clone.clone();
                                                                                                        let st = st_clone2.clone();
                                                                                                        let sid = sid_clone2.clone();
                                                                                                        let perm = value_str.clone();

                                                                                                        spawn_local(async move {
                                                                                                            let body = serde_json::json!({
                                                                                                                "subject_type": st,
                                                                                                                "subject_id": sid,
                                                                                                                "permission": perm
                                                                                                            });

                                                                                                            let req = Request::post("/api/admin/permissions")
                                                                                                                .header("Content-Type", "application/json")
                                                                                                                .body(body.to_string());

                                                                                                            if let Ok(request) = req {
                                                                                                                if request.send().await.is_ok() {
                                                                                                                    if let Ok(resp) = Request::get("/api/admin/permissions").send().await {
                                                                                                                        if let Ok(data) = resp.json::<Vec<PermissionEntry>>().await {
                                                                                                                            permissions.set(data);
                                                                                                                            adding.set(None);
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        });
                                                                                                    })
                                                                                                }}
                                                                                                class="w-full text-left px-3 py-2 bg-slate-800 hover:bg-slate-700 rounded text-sm text-gray-200 transition"
                                                                                            >
                                                                                                {label}
                                                                                            </button>
                                                                                        }
                                                                                    }).collect::<Html>()}
                                                                                </div>
                                                                            </div>
                                                                        }
                                                                    } else {
                                                                        html! {
                                                                            <button
                                                                                onclick={{
                                                                                    let adding_to_subject = adding_to_subject.clone();
                                                                                    let st_clone = st.clone();
                                                                                    let sid_clone = sid.clone();
                                                                                    Callback::from(move |_| {
                                                                                        adding_to_subject.set(Some((st_clone.clone(), sid_clone.clone())));
                                                                                    })
                                                                                }}
                                                                                class="inline-flex items-center space-x-1 px-3 py-1.5 bg-blue-500/10 border border-blue-500/30 hover:bg-blue-500/20 rounded-full text-blue-400 text-sm font-medium transition"
                                                                            >
                                                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                                                                                </svg>
                                                                                <span>{i18n.t("panel.administration.add")}</span>
                                                                            </button>
                                                                        }
                                                                    }}
                                                                </>
                                                            }
                                                        }

                                                        <button
                                                            onclick={{
                                                                let permissions_clone3 = permissions_clone.clone();
                                                                let perm_ids: Vec<i64> = perms_list.iter().map(|(id, _)| *id).collect();
                                                                Callback::from(move |_| {
                                                                    let permissions = permissions_clone3.clone();
                                                                    let ids = perm_ids.clone();
                                                                    spawn_local(async move {
                                                                        let mut success = true;
                                                                        for pid in ids {
                                                                            if Request::delete(&format!("/api/admin/permissions/{}", pid)).send().await.is_err() {
                                                                                success = false;
                                                                                break;
                                                                            }
                                                                        }
                                                                        if success {
                                                                            if let Ok(resp) = Request::get("/api/admin/permissions").send().await {
                                                                                if let Ok(data) = resp.json::<Vec<PermissionEntry>>().await {
                                                                                    permissions.set(data);
                                                                                }
                                                                            }
                                                                        }
                                                                    });
                                                                })
                                                            }}
                                                            class="inline-flex items-center space-x-1 px-3 py-1.5 bg-red-500/10 border border-red-500/30 hover:bg-red-500/20 rounded-full text-red-400 text-sm font-medium transition"
                                                        >
                                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                                                            </svg>
                                                            <span>{i18n.t("panel.administration.remove_all")}</span>
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()}
                            </div>
                        }
                    }}
                </div>
            </div>
                        </div>
                    }
                }}
            </section>
        </>
    }
}
