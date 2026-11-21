use crate::components::ui::{Button, ButtonVariant, Icon, IconName};
use crate::components::wizard::auth::authed_post;
use crate::components::wizard::persistence::clear_progress;
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SuccessScreenProps {
    pub panel_url: String,
    pub api_port: u16,
    pub on_unauthorized: Callback<()>,
}

fn build_target_url(panel_url: &str, api_port: u16) -> String {
    if panel_url.is_empty() {
        return format!("http://localhost:{}", api_port);
    }

    if let Some((scheme, rest)) = panel_url.split_once("://") {
        let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
        let authority = &rest[..authority_end];
        let suffix = &rest[authority_end..];
        let host_port = authority.rsplit('@').next().unwrap_or(authority);

        let (host, has_port) = if let Some((host, port)) = host_port.rsplit_once(':') {
            if port.parse::<u16>().is_ok() {
                (host, true)
            } else {
                (host_port, false)
            }
        } else {
            (host_port, false)
        };

        let is_local_host = host == "localhost" || host.parse::<std::net::Ipv4Addr>().is_ok();

        if is_local_host && !has_port {
            format!("{}://{}:{}{}", scheme, authority, api_port, suffix)
        } else {
            panel_url.to_string()
        }
    } else {
        panel_url.to_string()
    }
}

#[function_component(SuccessScreen)]
pub fn success_screen(props: &SuccessScreenProps) -> Html {
    let (i18n, _) = use_translation();
    let target_url = build_target_url(&props.panel_url, props.api_port);
    let is_restarting = use_state(|| false);
    let restart_error = use_state(|| None::<String>);

    let on_restart = {
        let on_unauthorized = props.on_unauthorized.clone();
        let is_restarting = is_restarting.clone();
        let restart_error = restart_error.clone();
        let target_url = target_url.clone();
        let failure_message = i18n.t("wizard.steps.step6.restart_failed");
        Callback::from(move |_: MouseEvent| {
            let target = target_url.clone();
            let on_unauthorized = on_unauthorized.clone();
            let is_restarting = is_restarting.clone();
            let restart_error = restart_error.clone();
            let failure_message = failure_message.clone();

            is_restarting.set(true);
            restart_error.set(None);

            spawn_local(async move {
                match authed_post("/api/setup/restart").send().await {
                    Ok(resp) if resp.status() == 401 => {
                        on_unauthorized.emit(());
                    }
                    Ok(resp) if resp.ok() => {
                        gloo_timers::future::TimeoutFuture::new(2000).await;
                        clear_progress();
                        let _ = web_sys::window().unwrap().location().set_href(&target);
                        return;
                    }
                    _ => {
                        restart_error.set(Some(failure_message));
                    }
                }

                is_restarting.set(false);
            });
        })
    };

    let instructions = [
        i18n.t("wizard.steps.step6.instruction_1"),
        i18n.t("wizard.steps.step6.instruction_2")
            .replace("{url}", &target_url),
        i18n.t("wizard.steps.step6.instruction_3"),
    ];

    html! {
        <main class="flex min-h-screen items-center justify-center bg-surface-page px-6 py-16">
            <div class="card flex w-full max-w-[520px] flex-col gap-5 p-6">
                <span class="inline-flex size-9 items-center justify-center rounded-xl bg-positive-subtle text-positive">
                    <Icon name={IconName::CircleCheck} />
                </span>
                <div class="flex flex-col gap-2">
                    <h1 class="text-section font-semibold text-fg">{i18n.t("wizard.steps.step6.save_success")}</h1>
                    <p class="text-sm text-fg-secondary">{i18n.t("wizard.steps.step6.success_desc")}</p>
                </div>
                <ol class="flex flex-col gap-2">
                    { for instructions.iter().enumerate().map(|(index, text)| html! {
                        <li class="flex gap-3 text-sm text-fg">
                            <span class="w-3 shrink-0 font-mono text-xs leading-5 text-fg-muted">{index + 1}</span>
                            <span class="min-w-0 break-words">{text.clone()}</span>
                        </li>
                    }) }
                </ol>
                if let Some(error) = (*restart_error).as_ref() {
                    <p class="text-[13px] text-negative">{error}</p>
                }
                <div class="flex justify-end">
                    <Button
                        variant={ButtonVariant::Primary}
                        icon={IconName::RotateCcw}
                        loading={*is_restarting}
                        onclick={on_restart}
                    >
                        {i18n.t("wizard.steps.step6.restart_bot")}
                    </Button>
                </div>
            </div>
        </main>
    }
}
