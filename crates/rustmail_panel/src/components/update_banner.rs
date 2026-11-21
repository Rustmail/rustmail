use crate::components::ui::{ButtonSize, ButtonVariant, Icon, IconButton, IconName};
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

    if !info.check_enabled || !info.update_available {
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
        <div class="flex h-11 items-center gap-3 rounded-xl border border-line bg-surface pl-3.5 pr-2">
            <Icon name={IconName::ArrowUpRight} class="text-accent-fg" />
            <p class="min-w-0 flex-1 truncate text-sm text-fg">
                { i18n.t("panel.update.available") }
                <span class="ml-3 font-mono text-xs text-fg-muted">
                    { format!("v{} → {}", info.current, latest) }
                </span>
            </p>
            <a href={release_url} target="_blank" rel="noopener noreferrer" class={crate::components::ui::button_classes(ButtonVariant::Ghost, ButtonSize::Sm)}>
                <Icon name={IconName::ExternalLink} />
                { i18n.t("panel.update.view_release") }
            </a>
            <IconButton icon={IconName::X} label={i18n.t("panel.update.dismiss")} onclick={on_dismiss} />
        </div>
    }
}
