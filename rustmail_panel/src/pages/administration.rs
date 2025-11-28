use crate::components::navbar::RustmailNavbar;
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
    let members = use_state(|| Vec::<Member>::new());
    let roles = use_state(|| Vec::<Role>::new());
    let permissions = use_state(|| Vec::<PermissionEntry>::new());
    let loading = use_state(|| true);
    let avatar = use_state(|| None::<String>);

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

    let avatar_url = (*avatar).clone().unwrap_or_default();

    html! {
        <>
            <RustmailNavbar avatar_url={avatar_url.clone()} />
            <section class="pt-24 min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                <div class="container mx-auto p-6 max-w-7xl">
                    <h1 class="text-3xl font-bold mb-6 text-white">{"Administration"}</h1>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
                <div class="bg-slate-800 rounded-lg shadow-lg p-6 border border-slate-700">
                    <h2 class="text-2xl font-semibold mb-4 text-white">{"Members"}</h2>
                    <div class="space-y-2 max-h-96 overflow-y-auto">
                        {members.iter().map(|member| html! {
                            <div key={member.user_id.clone()} class="flex items-center space-x-3 p-3 bg-slate-900 rounded border border-slate-700">
                                <span class="font-medium text-gray-200">{&member.username}</span>
                                <span class="text-gray-400 text-sm">{"#"}{&member.discriminator}</span>
                            </div>
                        }).collect::<Html>()}
                    </div>
                </div>

                <div class="bg-slate-800 rounded-lg shadow-lg p-6 border border-slate-700">
                    <h2 class="text-2xl font-semibold mb-4 text-white">{"Roles"}</h2>
                    <div class="space-y-2 max-h-96 overflow-y-auto">
                        {roles.iter().map(|role| html! {
                            <div key={role.role_id.clone()} class="flex items-center space-x-3 p-3 bg-slate-900 rounded border border-slate-700">
                                <div
                                    class="w-4 h-4 rounded-full"
                                    style={format!("background-color: #{:06x}", role.color)}
                                />
                                <span class="font-medium text-gray-200">{&role.name}</span>
                            </div>
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>

            <div class="bg-slate-800 rounded-lg shadow-lg p-6 border border-slate-700">
                <h2 class="text-2xl font-semibold mb-4 text-white">{"Panel Permissions"}</h2>
                <div class="overflow-x-auto">
                    <table class="min-w-full divide-y divide-slate-700">
                        <thead class="bg-slate-900">
                            <tr>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Type"}</th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Subject ID"}</th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Permission"}</th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Granted By"}</th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Actions"}</th>
                            </tr>
                        </thead>
                        <tbody class="bg-slate-800 divide-y divide-slate-700">
                            {permissions.iter().map(|perm| {
                                let perm_id = perm.id;
                                let permissions_clone = permissions.clone();
                                let revoke_callback = Callback::from(move |_| {
                                    let permissions = permissions_clone.clone();
                                    spawn_local(async move {
                                        if Request::delete(&format!("/api/admin/permissions/{}", perm_id)).send().await.is_ok() {
                                            if let Ok(resp) = Request::get("/api/admin/permissions").send().await {
                                                if let Ok(data) = resp.json::<Vec<PermissionEntry>>().await {
                                                    permissions.set(data);
                                                }
                                            }
                                        }
                                    });
                                });

                                html! {
                                    <tr key={perm.id}>
                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">{&perm.subject_type}</td>
                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-400 font-mono">{&perm.subject_id}</td>
                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">{perm.permission.to_display()}</td>
                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-400 font-mono">{&perm.granted_by}</td>
                                        <td class="px-6 py-4 whitespace-nowrap text-sm">
                                            <button
                                                onclick={revoke_callback}
                                                class="text-red-400 hover:text-red-300 transition"
                                            >
                                                {"Revoke"}
                                            </button>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                </div>
            </div>
                </div>
            </section>
        </>
    }
}
