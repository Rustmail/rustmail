use crate::components::language_switcher::LanguageSwitcher;
use crate::components::ui::{Button, ButtonVariant, IconName};
use crate::components::wizard::auth::authed_get;
use crate::components::wizard::parts::{STEP_KEYS, step_title_key};
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WelcomeProps {
    pub on_start: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

#[function_component(Welcome)]
pub fn welcome(props: &WelcomeProps) -> Html {
    let (i18n, _) = use_translation();
    let is_checking = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_click = {
        let on_start = props.on_start.clone();
        let on_unauthorized = props.on_unauthorized.clone();
        let is_checking = is_checking.clone();
        let error = error.clone();
        let unexpected_message = i18n.t("wizard.welcome_unexpected_response");
        let network_message = i18n.t("wizard.network_error");

        Callback::from(move |_: MouseEvent| {
            let on_start = on_start.clone();
            let on_unauthorized = on_unauthorized.clone();
            let is_checking = is_checking.clone();
            let error = error.clone();
            let unexpected_message = unexpected_message.clone();
            let network_message = network_message.clone();

            is_checking.set(true);
            error.set(None);

            spawn_local(async move {
                let res = authed_get("/api/setup/verify").send().await;

                match res {
                    Ok(resp) if resp.status() == 401 => on_unauthorized.emit(()),
                    Ok(resp) if resp.ok() => on_start.emit(()),
                    Ok(_) => error.set(Some(unexpected_message)),
                    Err(_) => error.set(Some(network_message)),
                }

                is_checking.set(false);
            });
        })
    };

    html! {
        <main class="flex min-h-screen items-center justify-center bg-surface-page px-6 py-16">
            <div class="flex w-full max-w-[480px] flex-col gap-6">
                <img src="/logo.png" alt="" class="size-11" />
                <div class="flex flex-col gap-3">
                    <h1 class="text-title font-semibold text-fg">{i18n.t("wizard.welcome_title")}</h1>
                    <p class="text-sm text-fg-secondary">{i18n.t("wizard.welcome_description")}</p>
                </div>
                <ol class="flex flex-col gap-2">
                    { for STEP_KEYS.iter().enumerate().map(|(index, key)| html! {
                        <li class="flex items-baseline gap-3 text-sm text-fg">
                            <span class="w-3 shrink-0 font-mono text-xs text-fg-muted">{index + 1}</span>
                            {i18n.t(&step_title_key(key))}
                        </li>
                    }) }
                </ol>
                <div class="flex items-center gap-2">
                    <Button
                        variant={ButtonVariant::Primary}
                        icon={IconName::ArrowRight}
                        loading={*is_checking}
                        onclick={on_click}
                    >
                        {i18n.t("wizard.welcome_start")}
                    </Button>
                    <LanguageSwitcher />
                </div>
                if let Some(err) = (*error).as_ref() {
                    <p class="text-[13px] text-negative">{err}</p>
                }
            </div>
        </main>
    }
}
