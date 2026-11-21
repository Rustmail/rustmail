mod fields;
mod form;
mod profile;
mod sections;

use crate::components::forbidden::{CheckingPermissions, Forbidden403};
use crate::components::ui::{
    Button, ButtonVariant, CardHeader, Field, IconName, Loading, Modal, Notice, NoticeTone,
    PageHeader, Select, StatusDot,
};
use crate::hooks::use_user_permissions;
use crate::i18n::yew::use_translation;
use crate::types::PanelPermission;
use form::ConfigForm;
use gloo_net::http::Request;
use profile::DiscordProfileCard;
use rustmail_types::ConfigResponse;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const RUNNING: &str = "running";
const STOPPED: &str = "stopped";

#[derive(Clone, Copy, PartialEq)]
enum BotAction {
    Start,
    Stop,
    Restart,
}

impl BotAction {
    fn path(self) -> &'static str {
        match self {
            BotAction::Start => "/api/bot/start",
            BotAction::Stop => "/api/bot/stop",
            BotAction::Restart => "/api/bot/restart",
        }
    }

    fn resulting_status(self) -> &'static str {
        match self {
            BotAction::Start | BotAction::Restart => RUNNING,
            BotAction::Stop => STOPPED,
        }
    }
}

async fn fetch_config() -> Option<ConfigResponse> {
    let resp = Request::get("/api/bot/config").send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    resp.json::<ConfigResponse>().await.ok()
}

#[function_component(ConfigurationPage)]
pub fn configuration_page() -> Html {
    let (i18n, _) = use_translation();
    match use_user_permissions() {
        None => html! { <CheckingPermissions /> },
        Some(perms) if !perms.contains(&PanelPermission::ManageConfig) => {
            html! { <Forbidden403 required_permission={i18n.t("panel.configuration.title")} /> }
        }
        Some(_) => html! { <ConfigurationContent /> },
    }
}

#[function_component(ConfigurationContent)]
fn configuration_content() -> Html {
    let (i18n, _) = use_translation();

    let bot_status = use_state(|| RUNNING.to_string());
    let presence = use_state(|| "online".to_string());
    let busy = use_state(|| false);
    let config = use_state(|| None::<ConfigResponse>);
    let config_loading = use_state(|| true);
    let form_revision = use_state(|| 0u32);
    let saving = use_state(|| false);
    let save_error = use_state(|| None::<AttrValue>);
    let save_success = use_state(|| false);
    let show_restart_modal = use_state(|| false);
    let confirm_stop = use_state(|| false);

    {
        let bot_status = bot_status.clone();
        let presence = presence.clone();
        let busy = busy.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                busy.set(true);
                if let Ok(resp) = Request::get("/api/bot/status").send().await
                    && resp.ok()
                    && let Ok(json) = resp.json::<serde_json::Value>().await
                {
                    if let Some(status) = json["status"].as_str() {
                        bot_status.set(status.to_string());
                    }
                    if let Some(value) = json["presence"].as_str() {
                        presence.set(value.to_string());
                    }
                }
                busy.set(false);
            });
            || ()
        });
    }

    {
        let config = config.clone();
        let config_loading = config_loading.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                config_loading.set(true);
                if let Some(loaded) = fetch_config().await {
                    config.set(Some(loaded));
                }
                config_loading.set(false);
            });
            || ()
        });
    }

    let run_action = {
        let bot_status = bot_status.clone();
        let busy = busy.clone();
        Callback::from(move |action: BotAction| {
            let bot_status = bot_status.clone();
            let busy = busy.clone();
            spawn_local(async move {
                busy.set(true);
                if let Ok(resp) = Request::post(action.path()).send().await
                    && resp.ok()
                {
                    bot_status.set(action.resulting_status().to_string());
                }
                busy.set(false);
            });
        })
    };

    let change_presence = {
        let presence = presence.clone();
        let busy = busy.clone();
        Callback::from(move |value: String| {
            let presence = presence.clone();
            let busy = busy.clone();
            spawn_local(async move {
                busy.set(true);
                if let Ok(req) =
                    Request::post("/api/bot/presence").json(&serde_json::json!({"status": value}))
                    && let Ok(resp) = req.send().await
                    && resp.ok()
                {
                    presence.set(value);
                }
                busy.set(false);
            });
        })
    };

    let save = {
        let config = config.clone();
        let form_revision = form_revision.clone();
        let saving = saving.clone();
        let save_error = save_error.clone();
        let save_success = save_success.clone();
        let show_restart_modal = show_restart_modal.clone();
        let i18n = i18n.clone();
        Callback::from(move |edited: ConfigResponse| {
            let config = config.clone();
            let form_revision = form_revision.clone();
            let saving = saving.clone();
            let save_error = save_error.clone();
            let save_success = save_success.clone();
            let show_restart_modal = show_restart_modal.clone();
            let i18n = i18n.clone();
            spawn_local(async move {
                saving.set(true);
                save_error.set(None);
                save_success.set(false);
                let outcome = match Request::put("/api/bot/config").json(&edited) {
                    Ok(req) => req.send().await.map_err(|_| ()),
                    Err(_) => Err(()),
                };
                match outcome {
                    Ok(resp) if resp.ok() => {
                        if let Some(loaded) = fetch_config().await {
                            config.set(Some(loaded));
                        }
                        form_revision.set(*form_revision + 1);
                        save_success.set(true);
                        show_restart_modal.set(true);
                    }
                    Ok(resp) => {
                        let detail = resp
                            .text()
                            .await
                            .ok()
                            .filter(|text| !text.trim().is_empty())
                            .unwrap_or_else(|| i18n.t("panel.configuration.unknown_error"));
                        save_error.set(Some(
                            format!("{}: {}", i18n.t("panel.configuration.save_error"), detail)
                                .into(),
                        ));
                    }
                    Err(()) => {
                        save_error.set(Some(i18n.t("panel.configuration.network_error").into()))
                    }
                }
                saving.set(false);
            });
        })
    };

    let close_restart_modal = {
        let show_restart_modal = show_restart_modal.clone();
        Callback::from(move |_| show_restart_modal.set(false))
    };

    let restart_now = {
        let show_restart_modal = show_restart_modal.clone();
        let run_action = run_action.clone();
        Callback::from(move |_: MouseEvent| {
            show_restart_modal.set(false);
            run_action.emit(BotAction::Restart);
        })
    };

    let close_stop_modal = {
        let confirm_stop = confirm_stop.clone();
        Callback::from(move |_| confirm_stop.set(false))
    };

    let stop_now = {
        let confirm_stop = confirm_stop.clone();
        let run_action = run_action.clone();
        Callback::from(move |_: MouseEvent| {
            confirm_stop.set(false);
            run_action.emit(BotAction::Stop);
        })
    };

    let running = *bot_status == RUNNING;
    let stopped = *bot_status == STOPPED;

    let settings = if *config_loading {
        html! { <Loading label={i18n.t("panel.configuration.loading")} /> }
    } else if let Some(loaded) = (*config).clone() {
        html! {
            <ConfigForm
                key={*form_revision}
                config={loaded}
                saving={*saving}
                save_error={(*save_error).clone()}
                on_save={save}
            />
        }
    } else {
        html! { <Notice>{i18n.t("panel.configuration.load_error")}</Notice> }
    };

    html! {
        <>
            <PageHeader
                title={i18n.t("panel.configuration.title")}
                description={i18n.t("panel.configuration.description")}
            />
            <div class="flex flex-col gap-6 xl:flex-row xl:items-start">
                <BotCard
                    running={running}
                    stopped={stopped}
                    busy={*busy}
                    presence={(*presence).clone()}
                    on_action={run_action.clone()}
                    on_request_stop={{
                        let confirm_stop = confirm_stop.clone();
                        Callback::from(move |_| confirm_stop.set(true))
                    }}
                    on_presence={change_presence}
                />
                <DiscordProfileCard bot_running={running} />
            </div>
            if *save_success {
                <Notice tone={NoticeTone::Positive}>{i18n.t("panel.configuration.save_success")}</Notice>
            }
            {settings}
            if *show_restart_modal {
                <Modal
                    title={i18n.t("panel.configuration.restart_modal.title")}
                    description={i18n.t("panel.configuration.restart_modal.message")}
                    on_close={close_restart_modal.clone()}
                    footer={html! {
                        <>
                            <Button variant={ButtonVariant::Ghost} onclick={close_restart_modal.reform(|_: MouseEvent| ())}>
                                {i18n.t("panel.configuration.restart_modal.later")}
                            </Button>
                            <Button variant={ButtonVariant::Primary} icon={IconName::RotateCcw} onclick={restart_now}>
                                {i18n.t("panel.configuration.restart_modal.yes")}
                            </Button>
                        </>
                    }}
                />
            }
            if *confirm_stop {
                <Modal
                    title={i18n.t("panel.configuration.stop_modal.title")}
                    description={i18n.t("panel.configuration.stop_modal.message")}
                    on_close={close_stop_modal.clone()}
                    footer={html! {
                        <>
                            <Button variant={ButtonVariant::Ghost} onclick={close_stop_modal.reform(|_: MouseEvent| ())}>
                                {i18n.t("panel.configuration.stop_modal.cancel")}
                            </Button>
                            <Button variant={ButtonVariant::Danger} icon={IconName::Square} onclick={stop_now}>
                                {i18n.t("panel.configuration.stop_modal.confirm")}
                            </Button>
                        </>
                    }}
                />
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct BotCardProps {
    running: bool,
    stopped: bool,
    busy: bool,
    presence: AttrValue,
    on_action: Callback<BotAction>,
    on_request_stop: Callback<()>,
    on_presence: Callback<String>,
}

#[function_component(BotCard)]
fn bot_card(props: &BotCardProps) -> Html {
    let (i18n, _) = use_translation();
    let t = |key: &str| i18n.t(&format!("panel.configuration.{key}"));

    let action = |action: BotAction| {
        let on_action = props.on_action.clone();
        Callback::from(move |_: MouseEvent| on_action.emit(action))
    };
    let request_stop = {
        let on_request_stop = props.on_request_stop.clone();
        Callback::from(move |_: MouseEvent| on_request_stop.emit(()))
    };

    let (dot, tone, status) = if props.running {
        ("bg-positive", "text-positive", t("online"))
    } else {
        ("bg-negative", "text-negative", t("offline"))
    };

    let presence_options: Vec<(AttrValue, AttrValue)> =
        ["online", "idle", "dnd", "invisible", "maintenance"]
            .into_iter()
            .map(|value| {
                (
                    AttrValue::from(value),
                    AttrValue::from(t(&format!("presence.{value}"))),
                )
            })
            .collect();

    html! {
        <section class="card flex shrink-0 flex-col gap-5 p-6 xl:w-[420px]">
            <CardHeader title={t("sections.bot")}>
                <span class={classes!("inline-flex", "items-center", "gap-2", "text-[13px]", tone)}>
                    <StatusDot class={dot} />
                    {status}
                </span>
            </CardHeader>
            <div class="flex flex-wrap items-center gap-2">
                <Button icon={IconName::Play} disabled={props.busy || props.running} onclick={action(BotAction::Start)}>
                    {t("start_bot")}
                </Button>
                <Button variant={ButtonVariant::Danger} icon={IconName::Square} disabled={props.busy || props.stopped} onclick={request_stop}>
                    {t("stop_bot")}
                </Button>
                <Button icon={IconName::RotateCcw} disabled={props.busy} onclick={action(BotAction::Restart)}>
                    {t("restart_bot")}
                </Button>
            </div>
            <Field label={t("presence_status")} help={t("presence_help")}>
                <Select
                    value={props.presence.clone()}
                    options={presence_options}
                    disabled={props.busy || props.stopped}
                    onchange={props.on_presence.clone()}
                />
            </Field>
        </section>
    }
}
