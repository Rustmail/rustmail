use crate::components::ui::{Button, Icon, IconName, Loading};
use crate::i18n::yew::use_translation;
use crate::pages::panel::PanelRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Forbidden403Props {
    pub required_permission: String,
}

#[function_component(Forbidden403)]
pub fn forbidden_403(props: &Forbidden403Props) -> Html {
    let (i18n, _set_language) = use_translation();
    let navigator = use_navigator();

    let go_home = Callback::from(move |_| {
        if let Some(nav) = &navigator {
            nav.push(&PanelRoute::Home);
        }
    });

    html! {
        <div class="flex min-h-[70vh] items-center justify-center">
            <div class="flex w-full max-w-[440px] flex-col gap-4">
                <span class="inline-flex size-10 items-center justify-center rounded-xl border border-line bg-surface-raised text-fg-secondary">
                    <Icon name={IconName::Lock} />
                </span>
                <h1 class="text-section font-semibold text-fg">
                    {i18n.t("panel.forbidden.title").replace("{page}", &props.required_permission)}
                </h1>
                <p class="text-sm text-fg-secondary">{i18n.t("panel.forbidden.message")}</p>
                <div class="flex items-center gap-3 rounded-lg border border-line bg-surface px-3 py-2.5">
                    <span class="flex-1 text-[13px] text-fg-secondary">{i18n.t("panel.forbidden.required_permission")}</span>
                    <span class="rounded border border-line-strong bg-surface-raised px-1.5 py-0.5 font-mono text-xs text-fg">
                        {props.required_permission.clone()}
                    </span>
                </div>
                <div class="pt-2">
                    <Button icon={IconName::ArrowLeft} onclick={go_home}>{i18n.t("panel.forbidden.back_home")}</Button>
                </div>
            </div>
        </div>
    }
}

#[function_component(CheckingPermissions)]
pub fn checking_permissions() -> Html {
    let (i18n, _set_language) = use_translation();
    html! { <Loading label={i18n.t("panel.forbidden.checking_permissions")} class="min-h-[60vh]" /> }
}
