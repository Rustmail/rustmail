use crate::components::forbidden::Forbidden403;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use i18nrs::yew::use_translation;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    CreateTicket,
    ReadTickets,
    UpdateTicket,
    DeleteTicket,
    ReadConfig,
    UpdateConfig,
    ManageBot,
}

impl Permission {
    pub fn all() -> Vec<Permission> {
        vec![
            Permission::CreateTicket,
            Permission::ReadTickets,
            Permission::UpdateTicket,
            Permission::DeleteTicket,
            Permission::ReadConfig,
            Permission::UpdateConfig,
            Permission::ManageBot,
        ]
    }

    pub fn to_display_string(&self) -> &str {
        match self {
            Permission::CreateTicket => "Create Ticket",
            Permission::ReadTickets => "Read Tickets",
            Permission::UpdateTicket => "Update Ticket",
            Permission::DeleteTicket => "Delete Ticket",
            Permission::ReadConfig => "Read Config",
            Permission::UpdateConfig => "Update Config",
            Permission::ManageBot => "Manage Bot",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyListItem {
    pub id: i64,
    pub name: String,
    pub permissions: Vec<Permission>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub last_used_at: Option<i64>,
    pub is_active: bool,
    pub key_preview: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: Vec<Permission>,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
    pub id: i64,
    pub name: String,
    pub permissions: Vec<Permission>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

#[function_component(ApiKeysPage)]
pub fn api_keys_page() -> Html {
    let (i18n, _set_language) = use_translation();
    let api_keys = use_state(|| Vec::<ApiKeyListItem>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let show_create_modal = use_state(|| false);
    let created_key = use_state(|| None::<String>);

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
        if !perms.contains(&PanelPermission::ManageApiKeys) {
            return html! {
                <Forbidden403 required_permission={i18n.t("navbar.apikeys")} />
            };
        }
    } else {
        return html! {
            <div class="flex items-center justify-center min-h-[70vh]">
                <div class="text-gray-400 animate-pulse">{i18n.t("panel.forbidden.checking_permissions")}</div>
            </div>
        };
    }

    {
        let api_keys = api_keys.clone();
        let loading = loading.clone();
        let error = error.clone();
        let i18n_clone = i18n.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                match Request::get("/api/apikeys").send().await {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            if let Ok(keys) = resp.json::<Vec<ApiKeyListItem>>().await {
                                api_keys.set(keys);
                                error.set(None);
                            } else {
                                error.set(Some(i18n_clone.t("panel.apikeys.error_parse")));
                            }
                        } else {
                            error.set(Some(format!("{}: {}", i18n_clone.t("panel.apikeys.error_load"), resp.status())));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("{}: {:?}", i18n_clone.t("panel.apikeys.error_network"), e)));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_create_click = {
        let show_create_modal = show_create_modal.clone();
        Callback::from(move |_| {
            show_create_modal.set(true);
        })
    };

    let on_close_modal = {
        let show_create_modal = show_create_modal.clone();
        let created_key = created_key.clone();
        Callback::from(move |_| {
            show_create_modal.set(false);
            created_key.set(None);
        })
    };

    let on_key_created = {
        let api_keys = api_keys.clone();
        let created_key = created_key.clone();
        Callback::from(move |key: String| {
            created_key.set(Some(key));
            let api_keys = api_keys.clone();
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/apikeys").send().await {
                    if resp.status() == 200 {
                        if let Ok(keys) = resp.json::<Vec<ApiKeyListItem>>().await {
                            api_keys.set(keys);
                        }
                    }
                }
            });
        })
    };

    let on_revoke = {
        let api_keys = api_keys.clone();
        Callback::from(move |id: i64| {
            let api_keys = api_keys.clone();
            spawn_local(async move {
                let url = format!("/api/apikeys/{}/revoke", id);
                if let Ok(resp) = Request::post(&url).send().await {
                    if resp.status() == 204 {
                        if let Ok(resp) = Request::get("/api/apikeys").send().await {
                            if resp.status() == 200 {
                                if let Ok(keys) = resp.json::<Vec<ApiKeyListItem>>().await {
                                    api_keys.set(keys);
                                }
                            }
                        }
                    }
                }
            });
        })
    };

    html! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <h1 class="text-3xl font-bold text-white">{i18n.t("panel.apikeys.title")}</h1>
                <button
                    onclick={on_create_click}
                    class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition"
                >
                    {i18n.t("panel.apikeys.create")}
                </button>
            </div>

            {
                if *loading {
                    html! {
                        <div class="text-center text-gray-400 py-8">
                            <p class="animate-pulse">{i18n.t("panel.apikeys.loading")}</p>
                        </div>
                    }
                } else if let Some(err) = (*error).clone() {
                    html! {
                        <div class="bg-red-900/20 border border-red-500 text-red-200 p-4 rounded-md">
                            {err}
                        </div>
                    }
                } else if api_keys.is_empty() {
                    html! {
                        <div class="bg-slate-800 rounded-lg p-8 text-center">
                            <p class="text-gray-400">{i18n.t("panel.apikeys.no_keys")}</p>
                        </div>
                    }
                } else {
                    html! {
                        <div class="space-y-4">
                            {
                                api_keys.iter().map(|key| {
                                    let key_id = key.id;
                                    let on_revoke = on_revoke.clone();
                                    html! {
                                        <ApiKeyCard
                                            key={key.id}
                                            api_key={key.clone()}
                                            on_revoke={Callback::from(move |_| on_revoke.emit(key_id))}
                                        />
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    }
                }
            }

            {
                if *show_create_modal {
                    html! {
                        <CreateApiKeyModal
                            on_close={on_close_modal}
                            on_created={on_key_created}
                            created_key={(*created_key).clone()}
                        />
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ApiKeyCardProps {
    pub api_key: ApiKeyListItem,
    pub on_revoke: Callback<()>,
}

#[function_component(ApiKeyCard)]
fn api_key_card(props: &ApiKeyCardProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let key = &props.api_key;
    let created_date = chrono::DateTime::from_timestamp(key.created_at, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| i18n.t("panel.apikeys.unknown"));

    let last_used = key
        .last_used_at
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| i18n.t("panel.apikeys.never"));

    html! {
        <div class="bg-slate-800 rounded-lg p-6 border border-slate-700">
            <div class="flex justify-between items-start mb-4">
                <div>
                    <h3 class="text-xl font-semibold text-white">{&key.name}</h3>
                    <p class="text-sm text-gray-400 font-mono">{&key.key_preview}</p>
                </div>
                <div class="flex gap-2">
                    {
                        if key.is_active {
                            html! {
                                <>
                                    <span class="px-3 py-1 bg-green-900/30 border border-green-500 text-green-200 rounded-full text-sm">
                                        {i18n.t("panel.apikeys.active")}
                                    </span>
                                    <button
                                        onclick={props.on_revoke.reform(|_| ())}
                                        class="px-3 py-1 bg-red-900/30 border border-red-500 text-red-200 hover:bg-red-900/50 rounded-md text-sm transition"
                                    >
                                        {i18n.t("panel.apikeys.revoke")}
                                    </button>
                                </>
                            }
                        } else {
                            html! {
                                <span class="px-3 py-1 bg-gray-900/30 border border-gray-500 text-gray-400 rounded-full text-sm">
                                    {i18n.t("panel.apikeys.revoked")}
                                </span>
                            }
                        }
                    }
                </div>
            </div>

            <div class="space-y-2 text-sm">
                <div class="flex gap-2 flex-wrap">
                    {
                        key.permissions.iter().map(|perm| {
                            html! {
                                <span class="px-2 py-1 bg-blue-900/30 border border-blue-500 text-blue-200 rounded text-xs">
                                    {perm.to_display_string()}
                                </span>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <p class="text-gray-400">{i18n.t("panel.apikeys.created")}{" "}{created_date}</p>
                <p class="text-gray-400">{i18n.t("panel.apikeys.last_used")}{" "}{last_used}</p>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CreateApiKeyModalProps {
    pub on_close: Callback<()>,
    pub on_created: Callback<String>,
    pub created_key: Option<String>,
}

#[function_component(CreateApiKeyModal)]
fn create_api_key_modal(props: &CreateApiKeyModalProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let name_ref = use_node_ref();
    let selected_permissions = use_state(|| Vec::<Permission>::new());
    let creating = use_state(|| false);
    let error = use_state(|| None::<String>);

    let toggle_permission = {
        let selected_permissions = selected_permissions.clone();
        Callback::from(move |perm: Permission| {
            let mut perms = (*selected_permissions).clone();
            if perms.contains(&perm) {
                perms.retain(|p| p != &perm);
            } else {
                perms.push(perm);
            }
            selected_permissions.set(perms);
        })
    };

    let on_create = {
        let name_ref = name_ref.clone();
        let selected_permissions = selected_permissions.clone();
        let creating = creating.clone();
        let error = error.clone();
        let on_created = props.on_created.clone();
        let i18n_clone = i18n.clone();

        Callback::from(move |_| {
            let name = name_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();

            if name.trim().is_empty() {
                error.set(Some(i18n_clone.t("panel.apikeys.modal.error_name_required")));
                return;
            }

            if selected_permissions.is_empty() {
                error.set(Some(i18n_clone.t("panel.apikeys.modal.error_permission_required")));
                return;
            }

            let request = CreateApiKeyRequest {
                name,
                permissions: (*selected_permissions).clone(),
                expires_at: None,
            };

            let creating = creating.clone();
            let error = error.clone();
            let on_created = on_created.clone();
            let i18n_clone2 = i18n_clone.clone();

            creating.set(true);
            spawn_local(async move {
                match Request::post("/api/apikeys")
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            if let Ok(response) = resp.json::<CreateApiKeyResponse>().await {
                                on_created.emit(response.api_key);
                                error.set(None);
                            } else {
                                error.set(Some(i18n_clone2.t("panel.apikeys.modal.error_parse_response")));
                            }
                        } else {
                            error.set(Some(format!("{}: {}", i18n_clone2.t("panel.apikeys.modal.error_create"), resp.status())));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("{}: {:?}", i18n_clone2.t("panel.apikeys.error_network"), e)));
                    }
                }
                creating.set(false);
            });
        })
    };

    html! {
        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
            <div class="bg-slate-800 rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto">
                <div class="p-6 space-y-6">
                    {
                        if let Some(key) = &props.created_key {
                            html! {
                                <>
                                    <h2 class="text-2xl font-bold text-white">{i18n.t("panel.apikeys.modal.title_created")}</h2>
                                    <div class="bg-yellow-900/20 border border-yellow-500 text-yellow-200 p-4 rounded-md">
                                        <p class="font-semibold mb-2">{i18n.t("panel.apikeys.modal.warning_title")}</p>
                                        <p class="text-sm">{i18n.t("panel.apikeys.modal.warning_message")}</p>
                                    </div>
                                    <div class="bg-slate-900 p-4 rounded-md">
                                        <p class="text-xs text-gray-400 mb-2">{i18n.t("panel.apikeys.modal.your_key")}</p>
                                        <code class="text-green-400 font-mono text-sm break-all">{key}</code>
                                    </div>
                                    <button
                                        onclick={props.on_close.reform(|_| ())}
                                        class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition"
                                    >
                                        {i18n.t("panel.apikeys.modal.close")}
                                    </button>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <h2 class="text-2xl font-bold text-white">{i18n.t("panel.apikeys.modal.title_create")}</h2>

                                    {
                                        if let Some(err) = (*error).clone() {
                                            html! {
                                                <div class="bg-red-900/20 border border-red-500 text-red-200 p-4 rounded-md">
                                                    {err}
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }

                                    <div>
                                        <label class="block text-sm font-medium text-gray-300 mb-2">{i18n.t("panel.apikeys.modal.name")}</label>
                                        <input
                                            ref={name_ref}
                                            type="text"
                                            placeholder={i18n.t("panel.apikeys.modal.name_placeholder")}
                                            class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        />
                                    </div>

                                    <div>
                                        <label class="block text-sm font-medium text-gray-300 mb-2">{i18n.t("panel.apikeys.modal.permissions")}</label>
                                        <div class="space-y-2">
                                            {
                                                Permission::all().iter().map(|perm| {
                                                    let perm_clone = perm.clone();
                                                    let is_selected = selected_permissions.contains(perm);
                                                    let toggle = toggle_permission.clone();
                                                    html! {
                                                        <label class="flex items-center space-x-3 p-3 bg-slate-900 rounded-md cursor-pointer hover:bg-slate-900/70">
                                                            <input
                                                                type="checkbox"
                                                                checked={is_selected}
                                                                onchange={move |_| toggle.emit(perm_clone.clone())}
                                                                class="w-4 h-4 text-blue-600 bg-slate-800 border-slate-600 rounded focus:ring-blue-500"
                                                            />
                                                            <span class="text-gray-300">{perm.to_display_string()}</span>
                                                        </label>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    </div>

                                    <div class="flex gap-3">
                                        <button
                                            onclick={props.on_close.reform(|_| ())}
                                            disabled={*creating}
                                            class="flex-1 px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white rounded-md transition disabled:opacity-50"
                                        >
                                            {i18n.t("panel.apikeys.modal.cancel")}
                                        </button>
                                        <button
                                            onclick={on_create}
                                            disabled={*creating}
                                            class="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition disabled:opacity-50"
                                        >
                                            {if *creating { i18n.t("panel.apikeys.modal.creating") } else { i18n.t("panel.apikeys.modal.create_button") }}
                                        </button>
                                    </div>
                                </>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
}
