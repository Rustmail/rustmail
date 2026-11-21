use crate::components::ui::{Icon, IconName};
use crate::i18n::yew::use_translation;
use yew::prelude::*;

fn example_setup_url() -> String {
    let origin = web_sys::window()
        .and_then(|window| window.location().origin().ok())
        .unwrap_or_else(|| "http://your-host:3002".to_string());
    format!("{origin}/setup?token=")
}

#[function_component(Unauthorized)]
pub fn unauthorized() -> Html {
    let (i18n, _) = use_translation();

    html! {
        <main class="flex min-h-screen items-center justify-center bg-surface-page px-6 py-16">
            <div class="flex w-full max-w-[480px] flex-col gap-5">
                <span class="inline-flex size-9 items-center justify-center rounded-xl border border-line bg-surface-raised text-fg-secondary">
                    <Icon name={IconName::Lock} />
                </span>
                <div class="flex flex-col gap-2">
                    <h1 class="text-section font-semibold text-fg">{i18n.t("wizard.unauthorized_title")}</h1>
                    <p class="text-sm text-fg-secondary">{i18n.t("wizard.unauthorized_description")}</p>
                </div>
                <div class="overflow-x-auto rounded-lg border border-line bg-surface px-3 py-2.5 font-mono text-[13px] text-fg">
                    {example_setup_url()}
                    <span class="text-fg-muted">{i18n.t("wizard.unauthorized_token_placeholder")}</span>
                </div>
            </div>
        </main>
    }
}
