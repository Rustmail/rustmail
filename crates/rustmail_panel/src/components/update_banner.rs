use crate::i18n::yew::use_translation;
use gloo_net::http::Request;
use rustmail_types::VersionInfo;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const DISMISSED_KEY: &str = "rustmail_dismissed_update";

fn dismissed_version() -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()
        .flatten()?
        .get_item(DISMISSED_KEY)
        .ok()
        .flatten()
}

fn dismiss_version(version: &str) {
    if let Some(Ok(Some(storage))) = web_sys::window().map(|w| w.local_storage()) {
        let _ = storage.set_item(DISMISSED_KEY, version);
    }
}

#[function_component(UpdateBanner)]
pub fn update_banner() -> Html {
    let (i18n, _set_language) = use_translation();

    let version_info = use_state(|| None::<VersionInfo>);
    let dismissed = use_state(dismissed_version);

    {
        let version_info = version_info.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/bot/version").send().await
                    && resp.ok()
                    && let Ok(info) = resp.json::<VersionInfo>().await
                {
                    version_info.set(Some(info));
                }
            });
            || ()
        });
    }

    let Some(info) = (*version_info).clone() else {
        return html! {};
    };

    if !info.update_available {
        return html! {};
    }

    let Some(latest) = info.latest.clone() else {
        return html! {};
    };

    if (*dismissed).as_deref() == Some(latest.as_str()) {
        return html! {};
    }

    let on_dismiss = {
        let dismissed = dismissed.clone();
        let latest = latest.clone();
        Callback::from(move |_| {
            dismiss_version(&latest);
            dismissed.set(Some(latest.clone()));
        })
    };

    let release_url = info
        .release_url
        .clone()
        .unwrap_or_else(|| "https://github.com/Rustmail/rustmail/releases".to_string());

    html! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-4">
            <div class="flex items-center justify-between gap-4 px-4 py-2 rounded-lg bg-indigo-500/10 border border-indigo-500/30">
                <p class="text-sm text-indigo-200">
                    { i18n.t("panel.update.available") }
                    <span class="text-xs text-gray-500 ml-2">
                        { format!("v{} → {}", info.current, latest) }
                    </span>
                </p>

                <div class="flex items-center gap-3 shrink-0">
                    <a
                        href={release_url}
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-sm font-medium text-indigo-300 hover:text-indigo-200 underline underline-offset-2"
                    >
                        { i18n.t("panel.update.view_release") }
                    </a>
                    <button
                        onclick={on_dismiss}
                        aria-label={i18n.t("panel.update.dismiss")}
                        title={i18n.t("panel.update.dismiss")}
                        class="text-gray-500 hover:text-gray-300 transition-colors px-1"
                    >
                        { "✕" }
                    </button>
                </div>
            </div>
        </div>
    }
}
