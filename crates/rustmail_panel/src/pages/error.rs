use crate::components::ui::{Button, ButtonVariant, IconName};
use crate::i18n::yew::use_translation;
use gloo_utils::window;
use web_sys::UrlSearchParams;
use yew::prelude::*;

pub fn brand() -> Html {
    html! {
        <a href="/" class="flex items-center gap-2.5">
            <img src="/logo.png" alt="" class="size-7" />
            <span class="text-base font-medium text-fg">{"Rustmail"}</span>
        </a>
    }
}

#[derive(Properties, PartialEq)]
struct CenteredColumnProps {
    children: Html,
}

#[function_component(CenteredColumn)]
fn centered_column(props: &CenteredColumnProps) -> Html {
    html! {
        <main class="flex min-h-screen items-center justify-center bg-surface-page px-6 py-16">
            <div class="flex w-full max-w-[440px] flex-col gap-6">
                {brand()}
                {props.children.clone()}
            </div>
        </main>
    }
}

fn server_message(fallback: String) -> String {
    let search = window().location().search().unwrap_or_default();
    UrlSearchParams::new_with_str(&search)
        .ok()
        .and_then(|params| params.get("message"))
        .filter(|message| !message.trim().is_empty())
        .unwrap_or(fallback)
}

#[function_component(Error)]
pub fn error() -> Html {
    let (i18n, _) = use_translation();
    let message = server_message(i18n.t("error.unknown"));

    html! {
        <CenteredColumn>
            <h1 class="text-section font-semibold text-fg">{i18n.t("error.title")}</h1>
            <div class="flex flex-col gap-1 rounded-lg bg-negative-subtle px-3 py-2.5">
                <span class="text-xs text-fg-secondary">{i18n.t("error.server_message")}</span>
                <span class="break-words font-mono text-[13px] text-negative">{message}</span>
            </div>
            <p class="text-sm text-fg-secondary">{i18n.t("error.help")}</p>
            <div>
                <Button variant={ButtonVariant::Primary} icon={IconName::ArrowLeft} href="/">
                    {i18n.t("error.back")}
                </Button>
            </div>
        </CenteredColumn>
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let (i18n, _) = use_translation();

    html! {
        <CenteredColumn>
            <div class="flex flex-col gap-2">
                <span class="font-mono text-xs text-fg-muted">{"404"}</span>
                <h1 class="text-section font-semibold text-fg">{i18n.t("not_found.title")}</h1>
                <p class="text-sm text-fg-secondary">{i18n.t("not_found.description")}</p>
            </div>
            <div>
                <Button icon={IconName::ArrowLeft} href="/panel">
                    {i18n.t("not_found.back")}
                </Button>
            </div>
        </CenteredColumn>
    }
}
