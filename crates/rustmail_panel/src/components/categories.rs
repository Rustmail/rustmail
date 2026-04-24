use crate::components::forbidden::Forbidden403;
use crate::i18n::yew::use_translation;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub discord_category_id: String,
    pub position: i64,
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategorySettingsDto {
    pub enabled: bool,
    pub selection_timeout_s: i64,
}

#[derive(Debug, Clone, Serialize)]
struct CreateCategoryRequest {
    name: String,
    description: Option<String>,
    emoji: Option<String>,
    discord_category_id: String,
}

#[derive(Debug, Clone, Serialize, Default)]
struct UpdateCategoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CategoryRolesDto {
    pub role_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CategoryRoleRequest {
    role_id: String,
}

async fn fetch_categories() -> Result<Vec<CategoryDto>, String> {
    let resp = Request::get("/api/categories")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != 200 {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, body));
    }
    resp.json::<Vec<CategoryDto>>()
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_settings() -> Result<CategorySettingsDto, String> {
    let resp = Request::get("/api/categories/settings")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != 200 {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, body));
    }
    resp.json::<CategorySettingsDto>()
        .await
        .map_err(|e| e.to_string())
}

#[function_component(CategoriesPage)]
pub fn categories_page() -> Html {
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
        if !perms.contains(&PanelPermission::ManageCategories) {
            return html! {
                <Forbidden403 required_permission={i18n.t("navbar.categories")} />
            };
        }
    } else {
        return html! {
            <div class="flex items-center justify-center min-h-[70vh]">
                <div class="text-gray-400 animate-pulse">{i18n.t("panel.forbidden.checking_permissions")}</div>
            </div>
        };
    }

    let categories = use_state(|| Vec::<CategoryDto>::new());
    let settings = use_state(|| None::<CategorySettingsDto>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let show_create_modal = use_state(|| false);

    let reload = {
        let categories = categories.clone();
        let settings = settings.clone();
        let loading = loading.clone();
        let error = error.clone();
        let i18n = i18n.clone();
        Callback::from(move |_| {
            let categories = categories.clone();
            let settings = settings.clone();
            let loading = loading.clone();
            let error = error.clone();
            let i18n = i18n.clone();
            spawn_local(async move {
                loading.set(true);
                let cats = fetch_categories().await;
                let s = fetch_settings().await;
                match (cats, s) {
                    (Ok(c), Ok(s)) => {
                        categories.set(c);
                        settings.set(Some(s));
                        error.set(None);
                    }
                    (Err(e), _) | (_, Err(e)) => error.set(Some(format!(
                        "{}: {}",
                        i18n.t("panel.categories.error_load"),
                        e
                    ))),
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

    let on_toggle_enabled = {
        let settings = settings.clone();
        let reload = reload.clone();
        Callback::from(move |_| {
            let current = match (*settings).clone() {
                Some(s) => s,
                None => return,
            };
            let new_settings = CategorySettingsDto {
                enabled: !current.enabled,
                selection_timeout_s: current.selection_timeout_s,
            };
            let reload = reload.clone();
            spawn_local(async move {
                if let Ok(req) = Request::put("/api/categories/settings").json(&new_settings) {
                    let _ = req.send().await;
                }
                reload.emit(());
            });
        })
    };

    let on_timeout_change = {
        let settings = settings.clone();
        let reload = reload.clone();
        Callback::from(move |e: Event| {
            let current = match (*settings).clone() {
                Some(s) => s,
                None => return,
            };
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input
                .value()
                .parse::<i64>()
                .unwrap_or(current.selection_timeout_s);
            if val < 30 {
                return;
            }
            let new_settings = CategorySettingsDto {
                enabled: current.enabled,
                selection_timeout_s: val,
            };
            let reload = reload.clone();
            spawn_local(async move {
                if let Ok(req) = Request::put("/api/categories/settings").json(&new_settings) {
                    let _ = req.send().await;
                }
                reload.emit(());
            });
        })
    };

    let on_toggle_category = {
        let reload = reload.clone();
        Callback::from(move |(id, enabled): (String, bool)| {
            let body = UpdateCategoryRequest {
                enabled: Some(enabled),
                ..Default::default()
            };
            let reload = reload.clone();
            spawn_local(async move {
                let url = format!("/api/categories/{}", id);
                if let Ok(req) = Request::patch(&url).json(&body) {
                    let _ = req.send().await;
                }
                reload.emit(());
            });
        })
    };

    let on_delete_category = {
        let reload = reload.clone();
        Callback::from(move |id: String| {
            let reload = reload.clone();
            spawn_local(async move {
                let url = format!("/api/categories/{}", id);
                let _ = Request::delete(&url).send().await;
                reload.emit(());
            });
        })
    };

    let on_create_click = {
        let show_create_modal = show_create_modal.clone();
        Callback::from(move |_| show_create_modal.set(true))
    };

    let on_close_modal = {
        let show_create_modal = show_create_modal.clone();
        Callback::from(move |_| show_create_modal.set(false))
    };

    let on_created = {
        let show_create_modal = show_create_modal.clone();
        let reload = reload.clone();
        Callback::from(move |_| {
            show_create_modal.set(false);
            reload.emit(());
        })
    };

    html! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <h1 class="text-3xl font-bold text-white">{i18n.t("panel.categories.title")}</h1>
                <button
                    onclick={on_create_click}
                    class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition"
                >
                    {i18n.t("panel.categories.create")}
                </button>
            </div>

            {
                if let Some(s) = (*settings).clone() {
                    html! {
                        <div class="bg-slate-800 rounded-lg p-6 border border-slate-700 space-y-4">
                            <h2 class="text-xl font-semibold text-white">{i18n.t("panel.categories.settings")}</h2>
                            <div class="flex items-center justify-between">
                                <div>
                                    <p class="text-gray-300">{i18n.t("panel.categories.feature_enabled")}</p>
                                    <p class="text-xs text-gray-500">{i18n.t("panel.categories.feature_enabled_help")}</p>
                                </div>
                                <button
                                    onclick={on_toggle_enabled}
                                    class={classes!(
                                        "px-4", "py-2", "rounded-md", "text-sm", "transition",
                                        if s.enabled {
                                            "bg-green-900/30 border border-green-500 text-green-200 hover:bg-green-900/50"
                                        } else {
                                            "bg-gray-900/30 border border-gray-500 text-gray-400 hover:bg-gray-900/50"
                                        }
                                    )}
                                >
                                    { if s.enabled { i18n.t("panel.categories.state_enabled") } else { i18n.t("panel.categories.state_disabled") } }
                                </button>
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-300 mb-2">
                                    {i18n.t("panel.categories.timeout_label")}
                                </label>
                                <input
                                    type="number"
                                    min="30"
                                    value={s.selection_timeout_s.to_string()}
                                    onchange={on_timeout_change}
                                    class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                />
                                <p class="text-xs text-gray-500 mt-1">{i18n.t("panel.categories.timeout_help")}</p>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            {
                if *loading {
                    html! {
                        <div class="text-center text-gray-400 py-8">
                            <p class="animate-pulse">{i18n.t("panel.categories.loading")}</p>
                        </div>
                    }
                } else if let Some(err) = (*error).clone() {
                    html! {
                        <div class="bg-red-900/20 border border-red-500 text-red-200 p-4 rounded-md">{err}</div>
                    }
                } else if categories.is_empty() {
                    html! {
                        <div class="bg-slate-800 rounded-lg p-8 text-center">
                            <p class="text-gray-400">{i18n.t("panel.categories.no_categories")}</p>
                        </div>
                    }
                } else {
                    html! {
                        <div class="space-y-4">
                            {
                                categories.iter().map(|c| {
                                    let cat = c.clone();
                                    let cat_id = cat.id.clone();
                                    let toggle_id = cat.id.clone();
                                    let on_toggle_category = on_toggle_category.clone();
                                    let on_delete_category = on_delete_category.clone();
                                    html! {
                                        <CategoryCard
                                            key={cat_id}
                                            category={cat}
                                            on_toggle={Callback::from(move |enabled: bool| {
                                                on_toggle_category.emit((toggle_id.clone(), enabled));
                                            })}
                                            on_delete={on_delete_category.clone()}
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
                        <CreateCategoryModal
                            on_close={on_close_modal}
                            on_created={on_created}
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
struct CategoryCardProps {
    category: CategoryDto,
    on_toggle: Callback<bool>,
    on_delete: Callback<String>,
}

async fn fetch_category_roles(id: &str) -> Result<CategoryRolesDto, String> {
    let url = format!("/api/categories/{}/roles", id);
    let resp = Request::get(&url).send().await.map_err(|e| e.to_string())?;
    if resp.status() != 200 {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, body));
    }
    resp.json::<CategoryRolesDto>()
        .await
        .map_err(|e| e.to_string())
}

#[function_component(CategoryCard)]
fn category_card(props: &CategoryCardProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let c = &props.category;
    let enabled = c.enabled;
    let on_toggle = props.on_toggle.clone();
    let on_delete = props.on_delete.clone();
    let cat_id = c.id.clone();

    let roles = use_state(|| Vec::<String>::new());
    let roles_loaded = use_state(|| false);
    let role_error = use_state(|| None::<String>);
    let role_input_ref = use_node_ref();

    let load_roles = {
        let id = c.id.clone();
        let roles = roles.clone();
        let roles_loaded = roles_loaded.clone();
        let role_error = role_error.clone();
        Callback::from(move |_| {
            let id = id.clone();
            let roles = roles.clone();
            let roles_loaded = roles_loaded.clone();
            let role_error = role_error.clone();
            spawn_local(async move {
                match fetch_category_roles(&id).await {
                    Ok(dto) => {
                        roles.set(dto.role_ids);
                        roles_loaded.set(true);
                        role_error.set(None);
                    }
                    Err(e) => {
                        roles.set(Vec::new());
                        roles_loaded.set(true);
                        role_error.set(Some(e));
                    }
                }
            });
        })
    };

    {
        let load_roles = load_roles.clone();
        use_effect_with(c.id.clone(), move |_| {
            load_roles.emit(());
            || ()
        });
    }

    let on_add_role = {
        let id = c.id.clone();
        let roles = roles.clone();
        let role_error = role_error.clone();
        let role_input_ref = role_input_ref.clone();
        let i18n = i18n.clone();
        Callback::from(move |_| {
            let input = match role_input_ref.cast::<HtmlInputElement>() {
                Some(i) => i,
                None => return,
            };
            let raw = input.value();
            let trimmed = raw.trim().trim_start_matches("<@&").trim_end_matches('>');
            if trimmed.parse::<u64>().is_err() {
                role_error.set(Some(i18n.t("panel.categories.error_role_invalid")));
                return;
            }
            let role_id = trimmed.to_string();
            let id = id.clone();
            let roles = roles.clone();
            let role_error = role_error.clone();
            let input = input.clone();
            spawn_local(async move {
                let url = format!("/api/categories/{}/roles", id);
                let body = CategoryRoleRequest {
                    role_id: role_id.clone(),
                };
                match Request::post(&url).json(&body) {
                    Ok(req) => match req.send().await {
                        Ok(resp) if resp.status() == 200 => {
                            if let Ok(dto) = resp.json::<CategoryRolesDto>().await {
                                roles.set(dto.role_ids);
                            }
                            role_error.set(None);
                            input.set_value("");
                        }
                        Ok(resp) => {
                            let status = resp.status();
                            let body = resp.text().await.unwrap_or_default();
                            role_error.set(Some(format!("HTTP {}: {}", status, body)));
                        }
                        Err(e) => role_error.set(Some(e.to_string())),
                    },
                    Err(e) => role_error.set(Some(format!("{:?}", e))),
                }
            });
        })
    };

    let on_remove_role = {
        let id = c.id.clone();
        let roles = roles.clone();
        let role_error = role_error.clone();
        Callback::from(move |role_id: String| {
            let id = id.clone();
            let roles = roles.clone();
            let role_error = role_error.clone();
            spawn_local(async move {
                let url = format!("/api/categories/{}/roles/{}", id, role_id);
                match Request::delete(&url).send().await {
                    Ok(resp) if resp.status() == 200 => {
                        if let Ok(dto) = resp.json::<CategoryRolesDto>().await {
                            roles.set(dto.role_ids);
                        }
                        role_error.set(None);
                    }
                    Ok(resp) => {
                        let status = resp.status();
                        let body = resp.text().await.unwrap_or_default();
                        role_error.set(Some(format!("HTTP {}: {}", status, body)));
                    }
                    Err(e) => role_error.set(Some(e.to_string())),
                }
            });
        })
    };

    html! {
        <div class="bg-slate-800 rounded-lg p-6 border border-slate-700 space-y-4">
            <div class="flex justify-between items-start">
                <div class="space-y-2">
                    <h3 class="text-xl font-semibold text-white">
                        { c.emoji.clone().unwrap_or_default() }{" "}{&c.name}
                    </h3>
                    {
                        if let Some(desc) = &c.description {
                            html! { <p class="text-sm text-gray-400">{desc}</p> }
                        } else {
                            html! {}
                        }
                    }
                    <p class="text-xs text-gray-500 font-mono">
                        {i18n.t("panel.categories.discord_category_id")}{": "}{&c.discord_category_id}
                    </p>
                </div>
                <div class="flex gap-2">
                    <button
                        onclick={Callback::from(move |_| on_toggle.emit(!enabled))}
                        class={classes!(
                            "px-3", "py-1", "rounded-md", "text-sm", "transition",
                            if enabled {
                                "bg-green-900/30 border border-green-500 text-green-200 hover:bg-green-900/50"
                            } else {
                                "bg-gray-900/30 border border-gray-500 text-gray-400 hover:bg-gray-900/50"
                            }
                        )}
                    >
                        { if enabled { i18n.t("panel.categories.state_enabled") } else { i18n.t("panel.categories.state_disabled") } }
                    </button>
                    <button
                        onclick={Callback::from(move |_| on_delete.emit(cat_id.clone()))}
                        class="px-3 py-1 bg-red-900/30 border border-red-500 text-red-200 hover:bg-red-900/50 rounded-md text-sm transition"
                    >
                        {i18n.t("panel.categories.delete")}
                    </button>
                </div>
            </div>

            <div class="border-t border-slate-700 pt-4 space-y-3">
                <div class="flex items-center justify-between">
                    <p class="text-sm font-medium text-gray-300">{i18n.t("panel.categories.roles_title")}</p>
                    <p class="text-xs text-gray-500">{i18n.t("panel.categories.roles_help")}</p>
                </div>
                {
                    if let Some(err) = (*role_error).clone() {
                        html! {
                            <div class="bg-red-900/20 border border-red-500 text-red-200 p-2 rounded-md text-sm">{err}</div>
                        }
                    } else { html! {} }
                }
                {
                    if !*roles_loaded {
                        html! {
                            <p class="text-xs text-gray-500 animate-pulse">{i18n.t("panel.categories.roles_loading")}</p>
                        }
                    } else if roles.is_empty() {
                        html! {
                            <p class="text-xs text-gray-500 italic">{i18n.t("panel.categories.roles_empty")}</p>
                        }
                    } else {
                        html! {
                            <div class="flex flex-wrap gap-2">
                                {
                                    roles.iter().map(|r| {
                                        let role_id = r.clone();
                                        let on_remove_role = on_remove_role.clone();
                                        let role_id_click = role_id.clone();
                                        html! {
                                            <span class="inline-flex items-center gap-2 bg-slate-900 border border-slate-700 text-gray-200 text-xs font-mono px-3 py-1 rounded-full">
                                                {role_id.clone()}
                                                <button
                                                    onclick={Callback::from(move |_| on_remove_role.emit(role_id_click.clone()))}
                                                    class="text-red-400 hover:text-red-200"
                                                    title={i18n.t("panel.categories.role_remove")}
                                                >{"×"}</button>
                                            </span>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    }
                }
                <div class="flex gap-2">
                    <input
                        ref={role_input_ref}
                        type="text"
                        placeholder={i18n.t("panel.categories.role_input_placeholder")}
                        class="flex-1 px-3 py-1.5 bg-slate-900 border border-slate-700 rounded-md text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                    <button
                        onclick={on_add_role}
                        class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white rounded-md text-sm transition"
                    >
                        {i18n.t("panel.categories.role_add")}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CreateCategoryModalProps {
    on_close: Callback<()>,
    on_created: Callback<()>,
}

#[function_component(CreateCategoryModal)]
fn create_category_modal(props: &CreateCategoryModalProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let name_ref = use_node_ref();
    let desc_ref = use_node_ref();
    let emoji_ref = use_node_ref();
    let discord_id_ref = use_node_ref();
    let roles_ref = use_node_ref();
    let creating = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_submit = {
        let name_ref = name_ref.clone();
        let desc_ref = desc_ref.clone();
        let emoji_ref = emoji_ref.clone();
        let discord_id_ref = discord_id_ref.clone();
        let roles_ref = roles_ref.clone();
        let creating = creating.clone();
        let error = error.clone();
        let on_created = props.on_created.clone();
        let i18n_clone = i18n.clone();

        Callback::from(move |_| {
            let name = name_ref
                .cast::<HtmlInputElement>()
                .map(|i| i.value())
                .unwrap_or_default();
            let discord_id = discord_id_ref
                .cast::<HtmlInputElement>()
                .map(|i| i.value())
                .unwrap_or_default();
            let desc = desc_ref
                .cast::<HtmlInputElement>()
                .map(|i| i.value())
                .filter(|s| !s.trim().is_empty());
            let emoji = emoji_ref
                .cast::<HtmlInputElement>()
                .map(|i| i.value())
                .filter(|s| !s.trim().is_empty());
            let roles_raw = roles_ref
                .cast::<HtmlInputElement>()
                .map(|i| i.value())
                .unwrap_or_default();

            if name.trim().is_empty() {
                error.set(Some(i18n_clone.t("panel.categories.error_name_required")));
                return;
            }
            if discord_id.trim().is_empty() {
                error.set(Some(
                    i18n_clone.t("panel.categories.error_discord_id_required"),
                ));
                return;
            }

            let mut role_ids: Vec<String> = Vec::new();
            for token in roles_raw
                .split(|c: char| c == ',' || c.is_whitespace())
                .map(|s| s.trim().trim_start_matches("<@&").trim_end_matches('>'))
                .filter(|s| !s.is_empty())
            {
                if token.parse::<u64>().is_err() {
                    error.set(Some(i18n_clone.t("panel.categories.error_role_invalid")));
                    return;
                }
                role_ids.push(token.to_string());
            }

            let req = CreateCategoryRequest {
                name,
                description: desc,
                emoji,
                discord_category_id: discord_id,
            };

            let creating = creating.clone();
            let error = error.clone();
            let on_created = on_created.clone();
            let i18n_clone2 = i18n_clone.clone();
            creating.set(true);
            spawn_local(async move {
                match Request::post("/api/categories").json(&req) {
                    Ok(r) => match r.send().await {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let created = resp.json::<CategoryDto>().await;
                                if !role_ids.is_empty() {
                                    if let Ok(ref cat) = created {
                                        let url = format!("/api/categories/{}/roles", cat.id);
                                        let body = CategoryRolesDto {
                                            role_ids: role_ids.clone(),
                                        };
                                        if let Ok(req) = Request::put(&url).json(&body) {
                                            let _ = req.send().await;
                                        }
                                    }
                                }
                                error.set(None);
                                on_created.emit(());
                            } else {
                                let status = resp.status();
                                let text = resp.text().await.unwrap_or_default();
                                error.set(Some(format!(
                                    "{}: {} {}",
                                    i18n_clone2.t("panel.categories.error_create"),
                                    status,
                                    text
                                )));
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("{:?}", e)));
                        }
                    },
                    Err(e) => {
                        error.set(Some(format!("{:?}", e)));
                    }
                }
                creating.set(false);
            });
        })
    };

    html! {
        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
            <div class="bg-slate-800 rounded-lg max-w-xl w-full max-h-[90vh] overflow-y-auto">
                <div class="p-6 space-y-6">
                    <h2 class="text-2xl font-bold text-white">{i18n.t("panel.categories.modal.title")}</h2>

                    {
                        if let Some(err) = (*error).clone() {
                            html! {
                                <div class="bg-red-900/20 border border-red-500 text-red-200 p-4 rounded-md">{err}</div>
                            }
                        } else {
                            html! {}
                        }
                    }

                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">{i18n.t("panel.categories.modal.name")}</label>
                        <input
                            ref={name_ref}
                            type="text"
                            placeholder={i18n.t("panel.categories.modal.name_placeholder")}
                            class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">{i18n.t("panel.categories.modal.description")}</label>
                        <input
                            ref={desc_ref}
                            type="text"
                            class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">{i18n.t("panel.categories.modal.emoji")}</label>
                        <input
                            ref={emoji_ref}
                            type="text"
                            placeholder="🎮"
                            class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">{i18n.t("panel.categories.modal.discord_category_id")}</label>
                        <input
                            ref={discord_id_ref}
                            type="text"
                            placeholder="123456789012345678"
                            class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">{i18n.t("panel.categories.modal.roles")}</label>
                        <input
                            ref={roles_ref}
                            type="text"
                            placeholder={i18n.t("panel.categories.modal.roles_placeholder")}
                            class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                        <p class="text-xs text-gray-500 mt-1">{i18n.t("panel.categories.modal.roles_help")}</p>
                    </div>

                    <div class="flex gap-3">
                        <button
                            onclick={props.on_close.reform(|_| ())}
                            disabled={*creating}
                            class="flex-1 px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white rounded-md transition disabled:opacity-50"
                        >
                            {i18n.t("panel.categories.modal.cancel")}
                        </button>
                        <button
                            onclick={on_submit}
                            disabled={*creating}
                            class="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition disabled:opacity-50"
                        >
                            { if *creating { i18n.t("panel.categories.modal.creating") } else { i18n.t("panel.categories.modal.create") } }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
