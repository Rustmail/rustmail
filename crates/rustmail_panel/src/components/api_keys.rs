use crate::components::forbidden::{CheckingPermissions, Forbidden403};
use crate::components::ui::{
    Badge, Button, ButtonSize, ButtonVariant, Checkbox, EmptyState, Field, IconName, Loading,
    Modal, Notice, NoticeTone, PageHeader, Tone, input_value,
};
use crate::hooks::use_user_permissions;
use crate::i18n::yew::use_translation;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};
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

    pub fn label_key(&self) -> &'static str {
        match self {
            Permission::CreateTicket => "panel.apikeys.permission.create_ticket",
            Permission::ReadTickets => "panel.apikeys.permission.read_tickets",
            Permission::UpdateTicket => "panel.apikeys.permission.update_ticket",
            Permission::DeleteTicket => "panel.apikeys.permission.delete_ticket",
            Permission::ReadConfig => "panel.apikeys.permission.read_config",
            Permission::UpdateConfig => "panel.apikeys.permission.update_config",
            Permission::ManageBot => "panel.apikeys.permission.manage_bot",
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

enum KeysLoadError {
    Parse,
    Status(u16),
    Network(String),
}

async fn fetch_keys() -> Result<Vec<ApiKeyListItem>, KeysLoadError> {
    let resp = Request::get("/api/apikeys")
        .send()
        .await
        .map_err(|e| KeysLoadError::Network(format!("{:?}", e)))?;
    if resp.status() != 200 {
        return Err(KeysLoadError::Status(resp.status()));
    }
    resp.json::<Vec<ApiKeyListItem>>()
        .await
        .map_err(|_| KeysLoadError::Parse)
}

fn format_day(timestamp: i64) -> Option<String> {
    chrono::DateTime::from_timestamp(timestamp, 0).map(|dt| dt.format("%-d %b %Y").to_string())
}

async fn write_to_clipboard(text: &str) -> bool {
    let navigator = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("navigator"));
    let Ok(clipboard) =
        navigator.and_then(|n| js_sys::Reflect::get(&n, &JsValue::from_str("clipboard")))
    else {
        return false;
    };
    let Ok(write_text) = js_sys::Reflect::get(&clipboard, &JsValue::from_str("writeText")) else {
        return false;
    };
    let Ok(write_text) = write_text.dyn_into::<js_sys::Function>() else {
        return false;
    };
    let Ok(promise) = write_text.call1(&clipboard, &JsValue::from_str(text)) else {
        return false;
    };
    let Ok(promise) = promise.dyn_into::<js_sys::Promise>() else {
        return false;
    };
    JsFuture::from(promise).await.is_ok()
}

#[function_component(ApiKeysPage)]
pub fn api_keys_page() -> Html {
    let (i18n, _set_language) = use_translation();
    match use_user_permissions() {
        None => html! { <CheckingPermissions /> },
        Some(perms) if !perms.contains(&PanelPermission::ManageApiKeys) => {
            html! { <Forbidden403 required_permission={i18n.t("navbar.apikeys")} /> }
        }
        Some(_) => html! { <ApiKeysContent /> },
    }
}

#[function_component(ApiKeysContent)]
fn api_keys_content() -> Html {
    let (i18n, _set_language) = use_translation();
    let api_keys = use_state(Vec::<ApiKeyListItem>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let show_create_modal = use_state(|| false);
    let created_key = use_state(|| None::<String>);
    let pending_revoke = use_state(|| None::<ApiKeyListItem>);
    let revoking = use_state(|| false);

    {
        let api_keys = api_keys.clone();
        let loading = loading.clone();
        let error = error.clone();
        let i18n = i18n.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                match fetch_keys().await {
                    Ok(keys) => {
                        api_keys.set(keys);
                        error.set(None);
                    }
                    Err(KeysLoadError::Parse) => {
                        error.set(Some(i18n.t("panel.apikeys.error_parse")))
                    }
                    Err(KeysLoadError::Status(status)) => error.set(Some(format!(
                        "{}: {}",
                        i18n.t("panel.apikeys.error_load"),
                        status
                    ))),
                    Err(KeysLoadError::Network(e)) => error.set(Some(format!(
                        "{}: {}",
                        i18n.t("panel.apikeys.error_network"),
                        e
                    ))),
                }
                loading.set(false);
            });
            || ()
        });
    }

    let open_create = {
        let show_create_modal = show_create_modal.clone();
        Callback::from(move |_: MouseEvent| show_create_modal.set(true))
    };

    let close_modals = {
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
                if let Ok(keys) = fetch_keys().await {
                    api_keys.set(keys);
                }
            });
        })
    };

    let header = html! {
        <PageHeader
            title={i18n.t("panel.apikeys.title")}
            description={i18n.t("panel.apikeys.description")}
        >
            <Button variant={ButtonVariant::Primary} icon={IconName::Plus} onclick={open_create.clone()}>
                {i18n.t("panel.apikeys.create")}
            </Button>
        </PageHeader>
    };

    let body = if *loading {
        html! { <Loading label={i18n.t("panel.apikeys.loading")} class="min-h-[40vh]" /> }
    } else if let Some(err) = (*error).clone() {
        html! { <Notice>{err}</Notice> }
    } else if api_keys.is_empty() {
        html! {
            <section class="card">
                <EmptyState
                    icon={IconName::KeyRound}
                    title={i18n.t("panel.apikeys.no_keys")}
                    description={i18n.t("panel.apikeys.no_keys_help")}
                >
                    <Button icon={IconName::Plus} onclick={open_create}>{i18n.t("panel.apikeys.create")}</Button>
                </EmptyState>
            </section>
        }
    } else {
        let now = chrono::Utc::now().timestamp();
        html! {
            <section class="card overflow-hidden p-0">
                <div class="overflow-x-auto">
                    <table class="w-full min-w-[760px] text-left">
                        <thead>
                            <tr class="h-9 text-xs text-fg-muted">
                                <th class="px-5 font-normal">{i18n.t("panel.apikeys.column_name")}</th>
                                <th class="px-5 font-normal">{i18n.t("panel.apikeys.column_key")}</th>
                                <th class="px-5 font-normal">{i18n.t("panel.apikeys.column_permissions")}</th>
                                <th class="px-5 font-normal">{i18n.t("panel.apikeys.last_used")}</th>
                                <th class="px-5 font-normal">{i18n.t("panel.apikeys.column_status")}</th>
                                <th class="px-5" />
                            </tr>
                        </thead>
                        <tbody>
                            { for api_keys.iter().map(|key| {
                                let ask_revoke = {
                                    let pending_revoke = pending_revoke.clone();
                                    let key = key.clone();
                                    Callback::from(move |_: MouseEvent| pending_revoke.set(Some(key.clone())))
                                };
                                html! { <ApiKeyRow key={key.id} api_key={key.clone()} {now} on_revoke={ask_revoke} /> }
                            }) }
                        </tbody>
                    </table>
                </div>
            </section>
        }
    };

    let revoke_modal = (*pending_revoke).clone().map(|key| {
        let close = {
            let pending_revoke = pending_revoke.clone();
            Callback::from(move |_| pending_revoke.set(None))
        };
        let confirm = {
            let pending_revoke = pending_revoke.clone();
            let revoking = revoking.clone();
            let api_keys = api_keys.clone();
            let id = key.id;
            Callback::from(move |_: MouseEvent| {
                let pending_revoke = pending_revoke.clone();
                let revoking = revoking.clone();
                let api_keys = api_keys.clone();
                revoking.set(true);
                spawn_local(async move {
                    let url = format!("/api/apikeys/{}/revoke", id);
                    if let Ok(resp) = Request::post(&url).send().await
                        && resp.status() == 204
                        && let Ok(keys) = fetch_keys().await
                    {
                        api_keys.set(keys);
                    }
                    revoking.set(false);
                    pending_revoke.set(None);
                });
            })
        };
        html! {
            <Modal
                title={i18n.t("panel.apikeys.revoke_title").replace("{name}", &key.name)}
                description={i18n.t("panel.apikeys.revoke_help")}
                on_close={close.clone()}
                width={440}
                footer={html! {
                    <>
                        <Button variant={ButtonVariant::Ghost} onclick={close.reform(|_: MouseEvent| ())}>
                            {i18n.t("panel.apikeys.modal.cancel")}
                        </Button>
                        <Button variant={ButtonVariant::Danger} loading={*revoking} onclick={confirm}>
                            {i18n.t("panel.apikeys.revoke")}
                        </Button>
                    </>
                }}
            />
        }
    });

    let key_modal = match ((*created_key).clone(), *show_create_modal) {
        (Some(key), _) => html! { <KeyCreatedModal api_key={key} on_close={close_modals} /> },
        (None, true) => {
            html! { <CreateApiKeyModal on_close={close_modals} on_created={on_key_created} /> }
        }
        (None, false) => html! {},
    };

    html! {
        <>
            {header}
            {body}
            {key_modal}
            {revoke_modal.unwrap_or_default()}
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ApiKeyRowProps {
    api_key: ApiKeyListItem,
    now: i64,
    on_revoke: Callback<MouseEvent>,
}

#[function_component(ApiKeyRow)]
fn api_key_row(props: &ApiKeyRowProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let key = &props.api_key;

    let created = format_day(key.created_at).unwrap_or_else(|| i18n.t("panel.apikeys.unknown"));
    let last_used = match key.last_used_at {
        None => i18n.t("panel.apikeys.never"),
        Some(ts) => {
            let elapsed = (props.now - ts).max(0);
            match elapsed {
                0..60 => i18n.t("panel.apikeys.just_now"),
                60..3600 => i18n
                    .t("panel.apikeys.minutes_ago")
                    .replace("{count}", &(elapsed / 60).to_string()),
                3600..86400 => i18n
                    .t("panel.apikeys.hours_ago")
                    .replace("{count}", &(elapsed / 3600).to_string()),
                86400..172800 => i18n.t("panel.apikeys.yesterday"),
                _ => format_day(ts).unwrap_or_else(|| i18n.t("panel.apikeys.unknown")),
            }
        }
    };

    let shown_permissions = key.permissions.iter().take(2);
    let hidden_count = key.permissions.len().saturating_sub(2);
    let name_tone = if key.is_active {
        "text-fg"
    } else {
        "text-fg-muted"
    };

    html! {
        <tr class="border-t border-line">
            <td class="px-5 py-2.5">
                <div class="flex min-w-0 flex-col">
                    <span class={classes!("truncate", "text-sm", "font-medium", name_tone)}>{key.name.clone()}</span>
                    <span class="text-xs text-fg-muted">
                        {i18n.t("panel.apikeys.created").replace("{date}", &created)}
                    </span>
                </div>
            </td>
            <td class="px-5 py-2.5 font-mono text-[13px] text-fg-secondary">{key.key_preview.clone()}</td>
            <td class="px-5 py-2.5">
                <div class="flex items-center gap-1.5">
                    { for shown_permissions.map(|perm| html! { <Badge>{i18n.t(perm.label_key())}</Badge> }) }
                    if hidden_count > 0 {
                        <span class="tnum font-mono text-xs text-fg-muted">{format!("+{}", hidden_count)}</span>
                    }
                </div>
            </td>
            <td class="whitespace-nowrap px-5 py-2.5 text-[13px] text-fg-secondary">{last_used}</td>
            <td class="px-5 py-2.5">
                if key.is_active {
                    <Badge tone={Tone::Positive}>{i18n.t("panel.apikeys.active")}</Badge>
                } else {
                    <Badge>{i18n.t("panel.apikeys.revoked")}</Badge>
                }
            </td>
            <td class="px-5 py-2.5 text-right">
                if key.is_active {
                    <Button variant={ButtonVariant::Ghost} size={ButtonSize::Sm} onclick={props.on_revoke.clone()}>
                        {i18n.t("panel.apikeys.revoke")}
                    </Button>
                }
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
struct CreateApiKeyModalProps {
    on_close: Callback<()>,
    on_created: Callback<String>,
}

#[function_component(CreateApiKeyModal)]
fn create_api_key_modal(props: &CreateApiKeyModalProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let name = use_state(String::new);
    let selected_permissions = use_state(Vec::<Permission>::new);
    let creating = use_state(|| false);
    let name_error = use_state(|| None::<String>);
    let permissions_error = use_state(|| None::<String>);
    let submit_error = use_state(|| None::<String>);

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

    let on_name = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| name.set(input_value(&e)))
    };

    let on_create = {
        let name = name.clone();
        let selected_permissions = selected_permissions.clone();
        let creating = creating.clone();
        let name_error = name_error.clone();
        let permissions_error = permissions_error.clone();
        let submit_error = submit_error.clone();
        let on_created = props.on_created.clone();
        let i18n = i18n.clone();
        Callback::from(move |_: MouseEvent| {
            let missing_name = name.trim().is_empty();
            let missing_permissions = selected_permissions.is_empty();
            name_error.set(missing_name.then(|| i18n.t("panel.apikeys.modal.error_name_required")));
            permissions_error.set(
                missing_permissions
                    .then(|| i18n.t("panel.apikeys.modal.error_permission_required")),
            );
            if missing_name || missing_permissions {
                return;
            }

            let request = CreateApiKeyRequest {
                name: (*name).clone(),
                permissions: (*selected_permissions).clone(),
                expires_at: None,
            };
            let creating = creating.clone();
            let submit_error = submit_error.clone();
            let on_created = on_created.clone();
            let i18n = i18n.clone();
            creating.set(true);
            spawn_local(async move {
                let sent = match Request::post("/api/apikeys").json(&request) {
                    Ok(req) => req.send().await.map_err(|e| format!("{:?}", e)),
                    Err(e) => Err(format!("{:?}", e)),
                };
                match sent {
                    Ok(resp) if resp.status() == 200 => {
                        match resp.json::<CreateApiKeyResponse>().await {
                            Ok(response) => {
                                submit_error.set(None);
                                on_created.emit(response.api_key);
                            }
                            Err(_) => submit_error
                                .set(Some(i18n.t("panel.apikeys.modal.error_parse_response"))),
                        }
                    }
                    Ok(resp) => submit_error.set(Some(format!(
                        "{}: {}",
                        i18n.t("panel.apikeys.modal.error_create"),
                        resp.status()
                    ))),
                    Err(e) => submit_error.set(Some(format!(
                        "{}: {}",
                        i18n.t("panel.apikeys.error_network"),
                        e
                    ))),
                }
                creating.set(false);
            });
        })
    };

    let close = props.on_close.clone();

    html! {
        <Modal
            title={i18n.t("panel.apikeys.modal.title_create")}
            description={i18n.t("panel.apikeys.modal.description")}
            on_close={close.clone()}
            footer={html! {
                <>
                    <Button variant={ButtonVariant::Ghost} disabled={*creating} onclick={close.reform(|_: MouseEvent| ())}>
                        {i18n.t("panel.apikeys.modal.cancel")}
                    </Button>
                    <Button variant={ButtonVariant::Primary} loading={*creating} onclick={on_create}>
                        {i18n.t("panel.apikeys.modal.create_button")}
                    </Button>
                </>
            }}
        >
            <div class="flex flex-col gap-4">
                if let Some(err) = (*submit_error).clone() {
                    <Notice>{err}</Notice>
                }
                <Field label={i18n.t("panel.apikeys.modal.name")} error={(*name_error).clone().map(AttrValue::from)}>
                    <input
                        type="text"
                        class="input"
                        placeholder={i18n.t("panel.apikeys.modal.name_placeholder")}
                        value={(*name).clone()}
                        oninput={on_name}
                    />
                </Field>
                <div class="flex flex-col gap-1.5">
                    <span class="label">{i18n.t("panel.apikeys.modal.permissions")}</span>
                    <div class="overflow-hidden rounded-lg border border-line">
                        { for Permission::all().into_iter().enumerate().map(|(index, perm)| {
                            let checked = selected_permissions.contains(&perm);
                            let label = i18n.t(perm.label_key());
                            let on_row = {
                                let toggle = toggle_permission.clone();
                                let perm = perm.clone();
                                Callback::from(move |_: MouseEvent| toggle.emit(perm.clone()))
                            };
                            let on_box = {
                                let toggle = toggle_permission.clone();
                                let perm = perm.clone();
                                Callback::from(move |_: bool| toggle.emit(perm.clone()))
                            };
                            let divider = if index == 0 { "" } else { "border-t border-line" };
                            html! {
                                <div
                                    key={perm.label_key()}
                                    role="button"
                                    class={classes!("flex", "h-[33px]", "cursor-pointer", "items-center", "gap-2.5", "bg-surface-raised/40", "px-3", "transition-colors", "duration-150", "hover:bg-surface-raised", divider)}
                                    onclick={on_row}
                                >
                                    <Checkbox {checked} onchange={on_box} label={label.clone()} />
                                    <span class="text-[13px] text-fg">{label}</span>
                                </div>
                            }
                        }) }
                    </div>
                    if let Some(err) = (*permissions_error).clone() {
                        <span class="text-xs text-negative">{err}</span>
                    }
                </div>
            </div>
        </Modal>
    }
}

#[derive(Properties, PartialEq)]
struct KeyCreatedModalProps {
    api_key: String,
    on_close: Callback<()>,
}

#[function_component(KeyCreatedModal)]
fn key_created_modal(props: &KeyCreatedModalProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let copied = use_state(|| false);

    let on_copy = {
        let copied = copied.clone();
        let api_key = props.api_key.clone();
        Callback::from(move |_: MouseEvent| {
            let copied = copied.clone();
            let api_key = api_key.clone();
            spawn_local(async move {
                if write_to_clipboard(&api_key).await {
                    copied.set(true);
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                    copied.set(false);
                }
            });
        })
    };

    let close = props.on_close.clone();

    html! {
        <Modal
            title={i18n.t("panel.apikeys.modal.title_created")}
            description={i18n.t("panel.apikeys.modal.created_description")}
            on_close={close.clone()}
            footer={html! {
                <Button variant={ButtonVariant::Primary} onclick={close.reform(|_: MouseEvent| ())}>
                    {i18n.t("panel.apikeys.modal.close")}
                </Button>
            }}
        >
            <div class="flex flex-col gap-4">
                <div class="flex h-11 items-center gap-2 rounded-lg border border-line-strong bg-surface-sunken pl-3 pr-1.5">
                    <code
                        class="min-w-0 flex-1 select-all truncate font-mono text-[13px] text-fg"
                        aria-label={i18n.t("panel.apikeys.modal.your_key")}
                    >
                        {props.api_key.clone()}
                    </code>
                    <Button
                        size={ButtonSize::Sm}
                        icon={if *copied { IconName::Check } else { IconName::Copy }}
                        onclick={on_copy}
                    >
                        {if *copied { i18n.t("panel.apikeys.modal.copied") } else { i18n.t("panel.apikeys.modal.copy") }}
                    </Button>
                </div>
                <Notice tone={NoticeTone::Warning}>{i18n.t("panel.apikeys.modal.warning_scope")}</Notice>
            </div>
        </Modal>
    }
}
