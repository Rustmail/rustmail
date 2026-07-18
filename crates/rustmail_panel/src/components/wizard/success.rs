use crate::components::wizard::auth::authed_post;
use crate::components::wizard::persistence::clear_progress;
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SuccessScreenProps {
    pub panel_url: String,
    pub api_port: u16,
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

    html! {
        <div class="min-h-screen bg-slate-950 flex flex-col items-center justify-center p-4 sm:p-8 text-white font-sans">
            <div class="flex flex-col items-center justify-center py-12 animate-fade-in text-center gap-4 max-w-lg">
                <div class="w-20 h-20 bg-green-500/20 rounded-full flex items-center justify-center text-green-400 mb-2 ring-4 ring-green-500/10">
                    <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                </div>
                <h2 class="text-2xl font-bold text-white">{ i18n.t("wizard.steps.step6.save_success") }</h2>
                <p class="text-gray-400 max-w-md">
                    { i18n.t("wizard.steps.step6.success_desc") }
                </p>
                <div class="mt-6 p-4 bg-slate-900 border border-slate-700 rounded-lg max-w-md w-full">
                    <p class="text-sm text-indigo-300 font-medium mb-2">{ i18n.t("wizard.steps.step6.apply_changes") }</p>
                    <ol class="text-sm text-gray-400 text-left list-decimal pl-5 space-y-2">
                        <li>{ i18n.t("wizard.steps.step6.instruction_1") }</li>
                        <li>{ i18n.t("wizard.steps.step6.instruction_2") }</li>
                        <li>{ i18n.t("wizard.steps.step6.instruction_3").replace("{url}", &target_url) }</li>
                    </ol>
                </div>
                <button
                    class="mt-6 px-6 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-white font-medium rounded-lg transition-colors shadow-lg shadow-indigo-600/20"
                    onclick={Callback::from(move |_| {
                        let target = target_url.clone();
                        spawn_local(async move {
                            let _ = authed_post("/api/setup/restart").send().await;

                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            clear_progress();
                            let _ = web_sys::window().unwrap().location().set_href(&target);
                        });
                    })}
                >

                    { i18n.t("wizard.steps.step6.restart_bot") }
                </button>
            </div>
        </div>
    }
}
