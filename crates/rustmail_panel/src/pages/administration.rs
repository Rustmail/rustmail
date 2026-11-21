use crate::components::forbidden::{CheckingPermissions, Forbidden403};
use crate::components::sidebar::AppShell;
use crate::components::ui::{
    Avatar, Button, ButtonSize, ButtonVariant, CardHeader, Checkbox, Chip, EmptyState, Field, Icon,
    IconName, Loading, Modal, Segmented, Select,
};
use crate::hooks::{use_user_avatar, use_user_permissions};
use crate::i18n::yew::use_translation;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Subject {
    kind: String,
    id: String,
}

async fn fetch_permissions() -> Vec<PermissionEntry> {
    match Request::get("/api/admin/permissions").send().await {
        Ok(resp) => resp
            .json::<Vec<PermissionEntry>>()
            .await
            .unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

async fn grant(kind: &str, id: &str, permission: &PanelPermission) -> bool {
    let body = serde_json::json!({
        "subject_type": kind,
        "subject_id": id,
        "permission": permission,
    });
    match Request::post("/api/admin/permissions")
        .header("Content-Type", "application/json")
        .body(body.to_string())
    {
        Ok(req) => req.send().await.map(|r| r.ok()).unwrap_or(false),
        Err(_) => false,
    }
}

async fn revoke(id: i64) -> bool {
    Request::delete(&format!("/api/admin/permissions/{}", id))
        .send()
        .await
        .map(|r| r.ok())
        .unwrap_or(false)
}

#[function_component(Administration)]
pub fn administration() -> Html {
    let (i18n, _set_language) = use_translation();
    let user_permissions = use_user_permissions();
    let avatar = use_user_avatar();

    let shell = |content: Html| -> Html {
        html! {
            <AppShell permissions={user_permissions.clone().unwrap_or_default()} avatar_url={avatar.clone()}>
                {content}
            </AppShell>
        }
    };

    match &user_permissions {
        None => shell(html! { <CheckingPermissions /> }),
        Some(perms) if !perms.contains(&PanelPermission::ManagePermissions) => {
            shell(html! { <Forbidden403 required_permission={i18n.t("navbar.administration")} /> })
        }
        Some(_) => shell(html! { <AdministrationContent /> }),
    }
}

#[function_component(AdministrationContent)]
fn administration_content() -> Html {
    let (i18n, _set_language) = use_translation();
    let members = use_state(Vec::<Member>::new);
    let roles = use_state(Vec::<Role>::new);
    let entries = use_state(Vec::<PermissionEntry>::new);
    let loading = use_state(|| true);

    let subject_kind = use_state(|| AttrValue::from("role"));
    let subject_id = use_state(String::new);
    let selected = use_state(Vec::<PanelPermission>::new);
    let granting = use_state(|| false);
    let adding_to = use_state(|| None::<Subject>);
    let confirm_remove = use_state(|| None::<(Subject, String)>);

    {
        let members = members.clone();
        let roles = roles.clone();
        let entries = entries.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/admin/members").send().await
                    && let Ok(data) = resp.json::<Vec<Member>>().await
                {
                    members.set(data);
                }
                if let Ok(resp) = Request::get("/api/admin/roles").send().await
                    && let Ok(data) = resp.json::<Vec<Role>>().await
                {
                    roles.set(data);
                }
                entries.set(fetch_permissions().await);
                loading.set(false);
            });
            || ()
        });
    }

    if *loading {
        return html! { <Loading label={i18n.t("panel.administration.loading")} class="min-h-[60vh]" /> };
    }

    let display_name = {
        let members = (*members).clone();
        let roles = (*roles).clone();
        move |subject: &Subject| -> String {
            if subject.kind == "user" {
                members
                    .iter()
                    .find(|m| m.user_id == subject.id)
                    .map(|m| m.username.clone())
                    .unwrap_or_else(|| subject.id.clone())
            } else {
                roles
                    .iter()
                    .find(|r| r.role_id == subject.id)
                    .map(|r| format!("@{}", r.name))
                    .unwrap_or_else(|| subject.id.clone())
            }
        }
    };

    let mut grouped: BTreeMap<Subject, Vec<(i64, PanelPermission)>> = BTreeMap::new();
    for entry in entries.iter() {
        grouped
            .entry(Subject {
                kind: entry.subject_type.clone(),
                id: entry.subject_id.clone(),
            })
            .or_default()
            .push((entry.id, entry.permission.clone()));
    }

    let on_kind = {
        let subject_kind = subject_kind.clone();
        let subject_id = subject_id.clone();
        Callback::from(move |kind: AttrValue| {
            subject_kind.set(kind);
            subject_id.set(String::new());
        })
    };

    let subject_options: Vec<(AttrValue, AttrValue)> = {
        let placeholder = if *subject_kind == "user" {
            i18n.t("panel.administration.select_user")
        } else {
            i18n.t("panel.administration.select_role")
        };
        let mut options = vec![(AttrValue::from(""), AttrValue::from(placeholder))];
        if *subject_kind == "user" {
            options.extend(members.iter().map(|m| {
                (
                    AttrValue::from(m.user_id.clone()),
                    AttrValue::from(m.username.clone()),
                )
            }));
        } else {
            options.extend(roles.iter().map(|r| {
                (
                    AttrValue::from(r.role_id.clone()),
                    AttrValue::from(format!("@{}", r.name)),
                )
            }));
        }
        options
    };

    let on_subject = {
        let subject_id = subject_id.clone();
        Callback::from(move |value: String| subject_id.set(value))
    };

    let on_grant = {
        let subject_kind = subject_kind.clone();
        let subject_id = subject_id.clone();
        let selected = selected.clone();
        let entries = entries.clone();
        let granting = granting.clone();
        Callback::from(move |_| {
            let kind = subject_kind.to_string();
            let id = (*subject_id).clone();
            let perms = (*selected).clone();
            if id.is_empty() || perms.is_empty() {
                return;
            }
            let selected = selected.clone();
            let entries = entries.clone();
            let granting = granting.clone();
            granting.set(true);
            spawn_local(async move {
                for perm in &perms {
                    grant(&kind, &id, perm).await;
                }
                entries.set(fetch_permissions().await);
                selected.set(Vec::new());
                granting.set(false);
            });
        })
    };

    let permission_tiles = PanelPermission::ALL.iter().map(|perm| {
        let checked = selected.contains(perm);
        let toggle = {
            let selected = selected.clone();
            let perm = perm.clone();
            Callback::from(move |_| {
                let mut next = (*selected).clone();
                if let Some(pos) = next.iter().position(|p| *p == perm) {
                    next.remove(pos);
                } else {
                    next.push(perm.clone());
                }
                selected.set(next);
            })
        };
        let toggle_box = {
            let toggle = toggle.clone();
            Callback::from(move |_: bool| toggle.emit(()))
        };
        let frame = if checked {
            "border-line-strong bg-surface-raised"
        } else {
            "border-line hover:bg-surface-raised/50"
        };
        html! {
            <div
                key={perm.key()}
                role="button"
                class={classes!("flex", "cursor-pointer", "items-start", "gap-2.5", "rounded-xl", "border", "p-3", "transition-colors", "duration-150", frame)}
                onclick={toggle.reform(|_: MouseEvent| ())}
            >
                <span class="pt-0.5">
                    <Checkbox checked={checked} onchange={toggle_box} label={i18n.t(&perm.label_key())} />
                </span>
                <span class="flex min-w-0 flex-col gap-0.5">
                    <span class="text-sm font-medium text-fg">{i18n.t(&perm.label_key())}</span>
                    <span class="text-xs text-fg-muted">{i18n.t(&perm.description_key())}</span>
                </span>
            </div>
        }
    });

    let selected_count = selected.len();
    let grant_label = if selected_count == 0 {
        i18n.t("panel.administration.grant_button")
    } else {
        i18n.t("panel.administration.grant_button_count")
            .replace("{count}", &selected_count.to_string())
    };

    let rows = grouped.iter().map(|(subject, perms)| {
        let name = display_name(subject);
        let is_user = subject.kind == "user";
        let avatar_url = if is_user {
            members
                .iter()
                .find(|m| m.user_id == subject.id)
                .and_then(|m| m.avatar.clone())
                .filter(|a| a.starts_with("http"))
                .map(AttrValue::from)
        } else {
            None
        };
        let identity = if is_user {
            html! { <Avatar name={name.clone()} url={avatar_url} size={32} /> }
        } else {
            html! {
                <span class="inline-flex size-8 shrink-0 items-center justify-center rounded-full bg-surface-raised text-fg-secondary">
                    <Icon name={IconName::Users} />
                </span>
            }
        };

        let chips = perms.iter().map(|(id, perm)| {
            let on_remove = {
                let entries = entries.clone();
                let id = *id;
                Callback::from(move |_| {
                    let entries = entries.clone();
                    spawn_local(async move {
                        revoke(id).await;
                        entries.set(fetch_permissions().await);
                    });
                })
            };
            html! {
                <Chip key={*id} on_remove={on_remove} remove_label={i18n.t("panel.administration.remove")}>
                    {i18n.t(&perm.label_key())}
                </Chip>
            }
        });

        let missing: Vec<PanelPermission> = PanelPermission::ALL
            .iter()
            .filter(|p| !perms.iter().any(|(_, have)| have == *p))
            .cloned()
            .collect();
        let is_adding = adding_to.as_ref() == Some(subject);
        let toggle_add = {
            let adding_to = adding_to.clone();
            let subject = subject.clone();
            Callback::from(move |_| {
                if adding_to.as_ref() == Some(&subject) {
                    adding_to.set(None);
                } else {
                    adding_to.set(Some(subject.clone()));
                }
            })
        };
        let add_list = if is_adding {
            html! {
                <div class="flex w-full flex-wrap items-center gap-1.5 pt-1">
                    <span class="mr-1 text-xs text-fg-muted">{i18n.t("panel.administration.available_permissions")}</span>
                    { for missing.iter().map(|perm| {
                        let onclick = {
                            let entries = entries.clone();
                            let adding_to = adding_to.clone();
                            let subject = subject.clone();
                            let perm = perm.clone();
                            Callback::from(move |_| {
                                let entries = entries.clone();
                                let adding_to = adding_to.clone();
                                let subject = subject.clone();
                                let perm = perm.clone();
                                spawn_local(async move {
                                    grant(&subject.kind, &subject.id, &perm).await;
                                    entries.set(fetch_permissions().await);
                                    adding_to.set(None);
                                });
                            })
                        };
                        html! {
                            <button type="button" key={perm.key()} class="inline-flex h-6 items-center gap-1 rounded-md border border-dashed border-line-strong px-2 text-xs text-fg-secondary transition-colors hover:bg-surface-raised hover:text-fg" {onclick}>
                                <Icon name={IconName::Plus} size={12} />
                                {i18n.t(&perm.label_key())}
                            </button>
                        }
                    }) }
                </div>
            }
        } else {
            html! {}
        };

        let ask_remove = {
            let confirm_remove = confirm_remove.clone();
            let subject = subject.clone();
            let name = name.clone();
            Callback::from(move |_| confirm_remove.set(Some((subject.clone(), name.clone()))))
        };

        html! {
            <div key={format!("{}:{}", subject.kind, subject.id)} class="flex flex-col gap-3 border-t border-line px-6 py-4 md:flex-row md:items-start md:gap-4">
                <div class="flex w-56 shrink-0 items-center gap-2.5">
                    {identity}
                    <div class="flex min-w-0 flex-col">
                        <span class="truncate text-sm font-medium text-fg">{name.clone()}</span>
                        <span class="text-xs text-fg-muted">
                            {if is_user { i18n.t("panel.administration.user") } else { i18n.t("panel.administration.role") }}
                        </span>
                    </div>
                </div>
                <div class="flex min-w-0 flex-1 flex-wrap items-center gap-1.5">
                    { for chips }
                    if !missing.is_empty() {
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Sm} icon={IconName::Plus} onclick={toggle_add}>
                            {i18n.t("panel.administration.add")}
                        </Button>
                    }
                    {add_list}
                </div>
                <Button variant={ButtonVariant::Ghost} size={ButtonSize::Sm} icon={IconName::TrashTwo} onclick={ask_remove}>
                    {i18n.t("panel.administration.remove_all")}
                </Button>
            </div>
        }
    });

    let remove_modal = if let Some((subject, name)) = (*confirm_remove).clone() {
        let close = {
            let confirm_remove = confirm_remove.clone();
            Callback::from(move |_| confirm_remove.set(None))
        };
        let confirm = {
            let confirm_remove = confirm_remove.clone();
            let entries = entries.clone();
            let ids: Vec<i64> = grouped
                .get(&subject)
                .map(|p| p.iter().map(|(id, _)| *id).collect())
                .unwrap_or_default();
            Callback::from(move |_| {
                let confirm_remove = confirm_remove.clone();
                let entries = entries.clone();
                let ids = ids.clone();
                spawn_local(async move {
                    for id in ids {
                        revoke(id).await;
                    }
                    entries.set(fetch_permissions().await);
                    confirm_remove.set(None);
                });
            })
        };
        html! {
            <Modal
                title={i18n.t("panel.administration.remove_all_title").replace("{name}", &name)}
                description={i18n.t("panel.administration.remove_all_help")}
                on_close={close.clone()}
                width={440}
                footer={html! {
                    <>
                        <Button variant={ButtonVariant::Ghost} onclick={close.reform(|_: MouseEvent| ())}>{i18n.t("panel.administration.cancel")}</Button>
                        <Button variant={ButtonVariant::Danger} icon={IconName::TrashTwo} onclick={confirm}>{i18n.t("panel.administration.remove_all")}</Button>
                    </>
                }}
            />
        }
    } else {
        html! {}
    };

    html! {
        <>
            <crate::components::ui::PageHeader
                title={i18n.t("panel.administration.title")}
                description={i18n.t("panel.administration.description")}
            />

            <section class="card flex flex-col gap-5 p-6">
                <CardHeader
                    title={i18n.t("panel.administration.grant_permission")}
                    description={i18n.t("panel.administration.grant_help")}
                />
                <div class="flex flex-wrap items-end gap-4">
                    <div class="flex flex-col gap-1.5">
                        <span class="label">{i18n.t("panel.administration.subject_type")}</span>
                        <Segmented
                            options={vec![
                                (AttrValue::from("user"), AttrValue::from(i18n.t("panel.administration.user"))),
                                (AttrValue::from("role"), AttrValue::from(i18n.t("panel.administration.role"))),
                            ]}
                            value={(*subject_kind).clone()}
                            onchange={on_kind}
                        />
                    </div>
                    <Field
                        label={if *subject_kind == "user" { i18n.t("panel.administration.user") } else { i18n.t("panel.administration.role") }}
                        class="w-full max-w-xs"
                    >
                        <Select value={AttrValue::from((*subject_id).clone())} options={subject_options} onchange={on_subject} />
                    </Field>
                </div>
                <div class="flex flex-col gap-2">
                    <span class="label">{i18n.t("panel.administration.select_permissions")}</span>
                    <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 xl:grid-cols-4">
                        { for permission_tiles }
                    </div>
                </div>
                <div class="flex items-center justify-end gap-3">
                    if selected_count > 0 {
                        <span class="text-[13px] text-fg-muted">
                            {i18n.t("panel.administration.selected").replace("{count}", &selected_count.to_string())}
                        </span>
                    }
                    <Button
                        variant={ButtonVariant::Primary}
                        loading={*granting}
                        disabled={subject_id.is_empty() || selected_count == 0}
                        onclick={on_grant}
                    >
                        {grant_label}
                    </Button>
                </div>
            </section>

            <section class="card overflow-hidden">
                <div class="flex items-center gap-2 px-6 py-5">
                    <h2 class="text-base font-semibold text-fg">{i18n.t("panel.administration.active_permissions")}</h2>
                    <span class="text-[13px] text-fg-muted">
                        {i18n.t("panel.administration.subjects").replace("{count}", &grouped.len().to_string())}
                    </span>
                </div>
                if grouped.is_empty() {
                    <div class="border-t border-line">
                        <EmptyState icon={IconName::Shield} title={i18n.t("panel.administration.no_permissions")} />
                    </div>
                } else {
                    { for rows }
                }
            </section>
            {remove_modal}
        </>
    }
}
