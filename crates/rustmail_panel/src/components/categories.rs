use crate::components::forbidden::{CheckingPermissions, Forbidden403};
use crate::components::ui::{
    Badge, Button, ButtonSize, ButtonVariant, Chip, EmptyState, Field, Icon, IconButton, IconName,
    Loading, Modal, Notice, PageHeader, Switch, input_value,
};
use crate::hooks::use_user_permissions;
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

async fn save_settings(settings: CategorySettingsDto) {
    if let Ok(req) = Request::put("/api/categories/settings").json(&settings) {
        let _ = req.send().await;
    }
}

fn parse_role_id(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_start_matches("<@&").trim_end_matches('>');
    trimmed.parse::<u64>().ok().map(|_| trimmed.to_string())
}

#[function_component(CategoriesPage)]
pub fn categories_page() -> Html {
    let (i18n, _set_language) = use_translation();
    match use_user_permissions() {
        None => html! { <CheckingPermissions /> },
        Some(perms) if !perms.contains(&PanelPermission::ManageCategories) => {
            html! { <Forbidden403 required_permission={i18n.t("navbar.categories")} /> }
        }
        Some(_) => html! { <CategoriesContent /> },
    }
}

#[function_component(CategoriesContent)]
fn categories_content() -> Html {
    let (i18n, _set_language) = use_translation();
    let categories = use_state(Vec::<CategoryDto>::new);
    let settings = use_state(|| None::<CategorySettingsDto>);
    let loaded = use_state(|| false);
    let error = use_state(|| None::<String>);
    let show_create_modal = use_state(|| false);
    let pending_delete = use_state(|| None::<CategoryDto>);

    let reload = {
        let categories = categories.clone();
        let settings = settings.clone();
        let loaded = loaded.clone();
        let error = error.clone();
        let i18n = i18n.clone();
        Callback::from(move |_| {
            let categories = categories.clone();
            let settings = settings.clone();
            let loaded = loaded.clone();
            let error = error.clone();
            let i18n = i18n.clone();
            spawn_local(async move {
                match (fetch_categories().await, fetch_settings().await) {
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
                loaded.set(true);
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
        Callback::from(move |enabled: bool| {
            let Some(current) = (*settings).clone() else {
                return;
            };
            let reload = reload.clone();
            spawn_local(async move {
                save_settings(CategorySettingsDto {
                    enabled,
                    selection_timeout_s: current.selection_timeout_s,
                })
                .await;
                reload.emit(());
            });
        })
    };

    let on_timeout_change = {
        let settings = settings.clone();
        let reload = reload.clone();
        Callback::from(move |e: Event| {
            let Some(current) = (*settings).clone() else {
                return;
            };
            let input: HtmlInputElement = e.target_unchecked_into();
            let seconds = input
                .value()
                .parse::<i64>()
                .unwrap_or(current.selection_timeout_s);
            if seconds < 30 {
                return;
            }
            let reload = reload.clone();
            spawn_local(async move {
                save_settings(CategorySettingsDto {
                    enabled: current.enabled,
                    selection_timeout_s: seconds,
                })
                .await;
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

    let open_create = {
        let show_create_modal = show_create_modal.clone();
        Callback::from(move |_: MouseEvent| show_create_modal.set(true))
    };

    let close_create = {
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

    if !*loaded {
        return html! { <Loading label={i18n.t("panel.categories.loading")} class="min-h-[60vh]" /> };
    }

    let header = html! {
        <PageHeader
            title={i18n.t("panel.categories.title")}
            description={i18n.t("panel.categories.description")}
        >
            <Button variant={ButtonVariant::Primary} icon={IconName::Plus} onclick={open_create.clone()}>
                {i18n.t("panel.categories.create")}
            </Button>
        </PageHeader>
    };

    let picker_card = (*settings).clone().map(|s| {
        html! {
            <section class="card flex flex-col px-5 py-1">
                <div class="flex items-center gap-4 py-4">
                    <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                        <span class="text-sm font-medium text-fg">{i18n.t("panel.categories.feature_enabled")}</span>
                        <span class="help">{i18n.t("panel.categories.feature_enabled_help")}</span>
                    </div>
                    <Switch
                        checked={s.enabled}
                        onchange={on_toggle_enabled.clone()}
                        label={i18n.t("panel.categories.feature_enabled")}
                    />
                </div>
                <div class="flex items-center gap-4 border-t border-line py-4">
                    <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                        <span class="text-sm font-medium text-fg">{i18n.t("panel.categories.timeout_label")}</span>
                        <span class="help">{i18n.t("panel.categories.timeout_help")}</span>
                    </div>
                    <div class="relative w-[120px] shrink-0">
                        <input
                            type="number"
                            min="30"
                            class="input input-mono tnum pr-11"
                            aria-label={i18n.t("panel.categories.timeout_label")}
                            value={s.selection_timeout_s.to_string()}
                            onchange={on_timeout_change.clone()}
                        />
                        <span class="pointer-events-none absolute inset-y-0 right-3 flex items-center font-mono text-xs text-fg-muted">
                            {i18n.t("panel.categories.timeout_unit")}
                        </span>
                    </div>
                </div>
            </section>
        }
    });

    let enabled_count = categories.iter().filter(|c| c.enabled).count();
    let count_label = if categories.len() == 1 {
        i18n.t("panel.categories.count_one")
    } else {
        i18n.t("panel.categories.count")
            .replace("{count}", &categories.len().to_string())
    };

    let list = if categories.is_empty() {
        html! {
            <section class="card">
                <EmptyState
                    icon={IconName::Folder}
                    title={i18n.t("panel.categories.no_categories")}
                    description={i18n.t("panel.categories.no_categories_help")}
                >
                    <Button icon={IconName::Plus} onclick={open_create}>
                        {i18n.t("panel.categories.create")}
                    </Button>
                </EmptyState>
            </section>
        }
    } else {
        html! {
            <section class="flex flex-col gap-3">
                <div class="flex items-baseline gap-2">
                    <h2 class="text-base font-semibold text-fg">{count_label}</h2>
                    <span class="text-[13px] text-fg-muted">
                        {i18n.t("panel.categories.enabled_count").replace("{count}", &enabled_count.to_string())}
                    </span>
                </div>
                { for categories.iter().map(|category| {
                    let on_toggle = {
                        let on_toggle_category = on_toggle_category.clone();
                        let id = category.id.clone();
                        Callback::from(move |enabled: bool| on_toggle_category.emit((id.clone(), enabled)))
                    };
                    let on_delete = {
                        let pending_delete = pending_delete.clone();
                        let category = category.clone();
                        Callback::from(move |_| pending_delete.set(Some(category.clone())))
                    };
                    html! {
                        <CategoryCard key={category.id.clone()} category={category.clone()} {on_toggle} {on_delete} />
                    }
                }) }
            </section>
        }
    };

    let delete_modal = (*pending_delete).clone().map(|category| {
        let close = {
            let pending_delete = pending_delete.clone();
            Callback::from(move |_| pending_delete.set(None))
        };
        let confirm = {
            let pending_delete = pending_delete.clone();
            let reload = reload.clone();
            let id = category.id.clone();
            Callback::from(move |_: MouseEvent| {
                let pending_delete = pending_delete.clone();
                let reload = reload.clone();
                let id = id.clone();
                spawn_local(async move {
                    let url = format!("/api/categories/{}", id);
                    let _ = Request::delete(&url).send().await;
                    pending_delete.set(None);
                    reload.emit(());
                });
            })
        };
        html! {
            <Modal
                title={i18n.t("panel.categories.delete_title").replace("{name}", &category.name)}
                description={i18n.t("panel.categories.delete_help")}
                on_close={close.clone()}
                width={440}
                footer={html! {
                    <>
                        <Button variant={ButtonVariant::Ghost} onclick={close.reform(|_: MouseEvent| ())}>
                            {i18n.t("panel.categories.cancel")}
                        </Button>
                        <Button variant={ButtonVariant::Danger} icon={IconName::TrashTwo} onclick={confirm}>
                            {i18n.t("panel.categories.delete")}
                        </Button>
                    </>
                }}
            />
        }
    });

    html! {
        <>
            {header}
            if let Some(err) = (*error).clone() {
                <Notice>{err}</Notice>
            }
            {picker_card.unwrap_or_default()}
            {list}
            if *show_create_modal {
                <CreateCategoryModal on_close={close_create} {on_created} />
            }
            {delete_modal.unwrap_or_default()}
        </>
    }
}

#[derive(Properties, PartialEq)]
struct CategoryCardProps {
    category: CategoryDto,
    on_toggle: Callback<bool>,
    on_delete: Callback<()>,
}

#[function_component(CategoryCard)]
fn category_card(props: &CategoryCardProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let category = &props.category;

    let roles = use_state(Vec::<String>::new);
    let roles_loaded = use_state(|| false);
    let role_error = use_state(|| None::<String>);
    let adding_role = use_state(|| false);
    let role_input = use_state(String::new);

    {
        let roles = roles.clone();
        let roles_loaded = roles_loaded.clone();
        let role_error = role_error.clone();
        use_effect_with(category.id.clone(), move |id| {
            let id = id.clone();
            spawn_local(async move {
                match fetch_category_roles(&id).await {
                    Ok(dto) => {
                        roles.set(dto.role_ids);
                        role_error.set(None);
                    }
                    Err(e) => {
                        roles.set(Vec::new());
                        role_error.set(Some(e));
                    }
                }
                roles_loaded.set(true);
            });
            || ()
        });
    }

    let submit_role = {
        let id = category.id.clone();
        let roles = roles.clone();
        let role_error = role_error.clone();
        let role_input = role_input.clone();
        let adding_role = adding_role.clone();
        let i18n = i18n.clone();
        Callback::from(move |_: ()| {
            let Some(role_id) = parse_role_id(&role_input) else {
                role_error.set(Some(i18n.t("panel.categories.error_role_invalid")));
                return;
            };
            let url = format!("/api/categories/{}/roles", id);
            let roles = roles.clone();
            let role_error = role_error.clone();
            let role_input = role_input.clone();
            let adding_role = adding_role.clone();
            spawn_local(async move {
                let body = CategoryRoleRequest { role_id };
                match Request::post(&url).json(&body) {
                    Ok(req) => match req.send().await {
                        Ok(resp) if resp.status() == 200 => {
                            if let Ok(dto) = resp.json::<CategoryRolesDto>().await {
                                roles.set(dto.role_ids);
                            }
                            role_error.set(None);
                            role_input.set(String::new());
                            adding_role.set(false);
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
        let id = category.id.clone();
        let roles = roles.clone();
        let role_error = role_error.clone();
        Callback::from(move |role_id: String| {
            let url = format!("/api/categories/{}/roles/{}", id, role_id);
            let roles = roles.clone();
            let role_error = role_error.clone();
            spawn_local(async move {
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

    let toggle_adding = {
        let adding_role = adding_role.clone();
        let role_input = role_input.clone();
        let role_error = role_error.clone();
        Callback::from(move |_: MouseEvent| {
            adding_role.set(!*adding_role);
            role_input.set(String::new());
            role_error.set(None);
        })
    };

    let on_role_input = {
        let role_input = role_input.clone();
        Callback::from(move |e: InputEvent| role_input.set(input_value(&e)))
    };

    let on_role_keydown = {
        let submit_role = submit_role.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                submit_role.emit(());
            }
        })
    };

    let tile = match category.emoji.clone().filter(|e| !e.trim().is_empty()) {
        Some(emoji) => html! { <span class="text-base leading-none">{emoji}</span> },
        None => html! { <Icon name={IconName::Folder} /> },
    };

    let name_tone = if category.enabled {
        "text-fg"
    } else {
        "text-fg-muted"
    };

    let role_chips = if !*roles_loaded {
        html! {
            <span class="inline-flex items-center gap-1.5 text-xs text-fg-muted">
                <Icon name={IconName::LoaderCircle} size={12} class="animate-spin" />
                {i18n.t("panel.categories.roles_loading")}
            </span>
        }
    } else if roles.is_empty() {
        html! { <span class="text-xs text-fg-muted">{i18n.t("panel.categories.roles_empty")}</span> }
    } else {
        roles
            .iter()
            .map(|role_id| {
                let on_remove = {
                    let on_remove_role = on_remove_role.clone();
                    let role_id = role_id.clone();
                    Callback::from(move |_: MouseEvent| on_remove_role.emit(role_id.clone()))
                };
                html! {
                    <Chip key={role_id.clone()} {on_remove} remove_label={i18n.t("panel.categories.role_remove")}>
                        <span class="text-fg-muted">{"@"}</span>
                        <span class="font-mono">{role_id.clone()}</span>
                    </Chip>
                }
            })
            .collect::<Html>()
    };

    html! {
        <article class="card flex flex-col gap-3 p-5">
            <div class="flex items-start gap-3">
                <span class="inline-flex size-9 shrink-0 items-center justify-center rounded-lg bg-surface-raised text-fg-secondary">
                    {tile}
                </span>
                <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                    <div class="flex items-center gap-2">
                        <span class={classes!("truncate", "text-sm", "font-medium", name_tone)}>{category.name.clone()}</span>
                        if !category.enabled {
                            <Badge>{i18n.t("panel.categories.state_disabled")}</Badge>
                        }
                    </div>
                    if let Some(description) = category.description.clone().filter(|d| !d.trim().is_empty()) {
                        <p class="text-[13px] text-fg-secondary">{description}</p>
                    }
                </div>
                <div class="flex shrink-0 items-center gap-3">
                    <span class="hidden font-mono text-xs text-fg-muted md:inline">
                        {i18n.t("panel.categories.discord_category").replace("{id}", &category.discord_category_id)}
                    </span>
                    <Switch
                        checked={category.enabled}
                        onchange={props.on_toggle.clone()}
                        label={i18n.t("panel.categories.toggle")}
                    />
                    <IconButton
                        icon={IconName::TrashTwo}
                        label={i18n.t("panel.categories.delete")}
                        onclick={props.on_delete.reform(|_: MouseEvent| ())}
                    />
                </div>
            </div>
            <div class="flex flex-col gap-2 pl-12">
                <div class="flex flex-wrap items-center gap-1.5">
                    <span class="mr-1 text-xs text-fg-muted">{i18n.t("panel.categories.roles_title")}</span>
                    {role_chips}
                    if !*adding_role {
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Sm} icon={IconName::Plus} onclick={toggle_adding.clone()}>
                            {i18n.t("panel.categories.role_add_open")}
                        </Button>
                    }
                </div>
                if *adding_role {
                    <div class="flex flex-wrap items-center gap-2">
                        <input
                            type="text"
                            class="input input-mono h-8 w-64"
                            placeholder={i18n.t("panel.categories.role_input_placeholder")}
                            aria-label={i18n.t("panel.categories.role_input_placeholder")}
                            value={(*role_input).clone()}
                            oninput={on_role_input}
                            onkeydown={on_role_keydown}
                        />
                        <Button size={ButtonSize::Sm} onclick={submit_role.reform(|_: MouseEvent| ())}>
                            {i18n.t("panel.categories.role_add")}
                        </Button>
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Sm} onclick={toggle_adding}>
                            {i18n.t("panel.categories.cancel")}
                        </Button>
                    </div>
                }
                if let Some(err) = (*role_error).clone() {
                    <span class="text-xs text-negative">{err}</span>
                }
            </div>
        </article>
    }
}

#[derive(Clone, PartialEq, Default)]
struct CreateFormErrors {
    name: Option<String>,
    discord_category_id: Option<String>,
    roles: Option<String>,
    submit: Option<String>,
}

#[derive(Properties, PartialEq)]
struct CreateCategoryModalProps {
    on_close: Callback<()>,
    on_created: Callback<()>,
}

#[function_component(CreateCategoryModal)]
fn create_category_modal(props: &CreateCategoryModalProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let name = use_state(String::new);
    let description = use_state(String::new);
    let emoji = use_state(String::new);
    let discord_category_id = use_state(String::new);
    let roles = use_state(String::new);
    let creating = use_state(|| false);
    let errors = use_state(CreateFormErrors::default);

    let bind = |state: &UseStateHandle<String>| {
        let state = state.clone();
        Callback::from(move |e: InputEvent| state.set(input_value(&e)))
    };

    let on_submit = {
        let name = name.clone();
        let description = description.clone();
        let emoji = emoji.clone();
        let discord_category_id = discord_category_id.clone();
        let roles = roles.clone();
        let creating = creating.clone();
        let errors = errors.clone();
        let on_created = props.on_created.clone();
        let i18n = i18n.clone();
        Callback::from(move |_: MouseEvent| {
            let mut found = CreateFormErrors::default();
            if name.trim().is_empty() {
                found.name = Some(i18n.t("panel.categories.error_name_required"));
            }
            if discord_category_id.trim().is_empty() {
                found.discord_category_id =
                    Some(i18n.t("panel.categories.error_discord_id_required"));
            }
            let mut role_ids: Vec<String> = Vec::new();
            for token in roles
                .split(|c: char| c == ',' || c.is_whitespace())
                .map(|s| s.trim().trim_start_matches("<@&").trim_end_matches('>'))
                .filter(|s| !s.is_empty())
            {
                if token.parse::<u64>().is_err() {
                    found.roles = Some(i18n.t("panel.categories.error_role_invalid"));
                    break;
                }
                role_ids.push(token.to_string());
            }
            if found != CreateFormErrors::default() {
                errors.set(found);
                return;
            }
            errors.set(CreateFormErrors::default());

            let request = CreateCategoryRequest {
                name: (*name).clone(),
                description: Some((*description).clone()).filter(|s| !s.trim().is_empty()),
                emoji: Some((*emoji).clone()).filter(|s| !s.trim().is_empty()),
                discord_category_id: (*discord_category_id).clone(),
            };

            let creating = creating.clone();
            let errors = errors.clone();
            let on_created = on_created.clone();
            let i18n = i18n.clone();
            creating.set(true);
            spawn_local(async move {
                let submit_error = match Request::post("/api/categories").json(&request) {
                    Ok(req) => match req.send().await {
                        Ok(resp) if resp.status() == 200 => {
                            let created = resp.json::<CategoryDto>().await;
                            if !role_ids.is_empty()
                                && let Ok(category) = &created
                            {
                                let url = format!("/api/categories/{}/roles", category.id);
                                let body = CategoryRolesDto {
                                    role_ids: role_ids.clone(),
                                };
                                if let Ok(req) = Request::put(&url).json(&body) {
                                    let _ = req.send().await;
                                }
                            }
                            None
                        }
                        Ok(resp) => {
                            let status = resp.status();
                            let text = resp.text().await.unwrap_or_default();
                            Some(format!(
                                "{}: {} {}",
                                i18n.t("panel.categories.modal.error_create"),
                                status,
                                text
                            ))
                        }
                        Err(e) => Some(format!("{:?}", e)),
                    },
                    Err(e) => Some(format!("{:?}", e)),
                };
                creating.set(false);
                match submit_error {
                    None => on_created.emit(()),
                    Some(message) => errors.set(CreateFormErrors {
                        submit: Some(message),
                        ..Default::default()
                    }),
                }
            });
        })
    };

    let close = props.on_close.clone();

    html! {
        <Modal
            title={i18n.t("panel.categories.modal.title")}
            description={i18n.t("panel.categories.modal.subtitle")}
            on_close={close.clone()}
            footer={html! {
                <>
                    <Button variant={ButtonVariant::Ghost} disabled={*creating} onclick={close.reform(|_: MouseEvent| ())}>
                        {i18n.t("panel.categories.modal.cancel")}
                    </Button>
                    <Button variant={ButtonVariant::Primary} loading={*creating} onclick={on_submit}>
                        {i18n.t("panel.categories.modal.create")}
                    </Button>
                </>
            }}
        >
            <div class="flex flex-col gap-4">
                if let Some(err) = errors.submit.clone() {
                    <Notice>{err}</Notice>
                }
                <Field label={i18n.t("panel.categories.modal.name")} error={errors.name.clone().map(AttrValue::from)}>
                    <input
                        type="text"
                        class="input"
                        placeholder={i18n.t("panel.categories.modal.name_placeholder")}
                        value={(*name).clone()}
                        oninput={bind(&name)}
                    />
                </Field>
                <Field label={i18n.t("panel.categories.modal.description")} help={i18n.t("panel.categories.modal.optional")}>
                    <input
                        type="text"
                        class="input"
                        value={(*description).clone()}
                        oninput={bind(&description)}
                    />
                </Field>
                <div class="flex items-start gap-3">
                    <Field label={i18n.t("panel.categories.modal.emoji")} help={i18n.t("panel.categories.modal.optional")} class="w-[100px] shrink-0">
                        <input
                            type="text"
                            class="input"
                            placeholder={i18n.t("panel.categories.modal.emoji_placeholder")}
                            value={(*emoji).clone()}
                            oninput={bind(&emoji)}
                        />
                    </Field>
                    <Field
                        label={i18n.t("panel.categories.modal.discord_category_id")}
                        error={errors.discord_category_id.clone().map(AttrValue::from)}
                        class="flex-1"
                    >
                        <input
                            type="text"
                            inputmode="numeric"
                            class="input input-mono"
                            placeholder="123456789012345678"
                            value={(*discord_category_id).clone()}
                            oninput={bind(&discord_category_id)}
                        />
                    </Field>
                </div>
                <Field
                    label={i18n.t("panel.categories.modal.roles")}
                    help={i18n.t("panel.categories.modal.roles_help")}
                    error={errors.roles.clone().map(AttrValue::from)}
                >
                    <input
                        type="text"
                        class="input input-mono"
                        placeholder={i18n.t("panel.categories.modal.roles_placeholder")}
                        value={(*roles).clone()}
                        oninput={bind(&roles)}
                    />
                </Field>
            </div>
        </Modal>
    }
}
