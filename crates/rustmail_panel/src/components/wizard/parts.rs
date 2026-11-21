use crate::components::ui::{Button, ButtonVariant, Icon, IconName};
use crate::i18n::yew::use_translation;
use yew::prelude::*;

pub const STEP_KEYS: [&str; 6] = ["step1", "step2", "step3", "step4", "step5", "step6"];

pub fn step_title_key(step: &str) -> String {
    format!("wizard.steps.{step}.title")
}

#[derive(Properties, PartialEq)]
pub struct StepNavProps {
    #[prop_or_default]
    pub on_back: Option<Callback<MouseEvent>>,
    pub on_next: Callback<MouseEvent>,
    #[prop_or_default]
    pub next_disabled: bool,
    #[prop_or_default]
    pub hint: Option<AttrValue>,
    #[prop_or_default]
    pub finish: bool,
    #[prop_or_default]
    pub busy: bool,
}

#[function_component(StepNav)]
pub fn step_nav(props: &StepNavProps) -> Html {
    let (i18n, _) = use_translation();
    let (label, icon) = if props.finish {
        let key = if props.busy {
            "wizard.steps.step6.saving"
        } else {
            "wizard.steps.step6.save_finish"
        };
        (i18n.t(key), Some(IconName::Check))
    } else {
        (i18n.t("wizard.common.next"), None)
    };

    html! {
        <div class="flex items-center gap-3 border-t border-line pt-5">
            if let Some(on_back) = props.on_back.clone() {
                <Button icon={IconName::ArrowLeft} onclick={on_back} disabled={props.busy}>
                    {i18n.t("wizard.common.back")}
                </Button>
            }
            <span class="flex-1" />
            if props.next_disabled {
                if let Some(hint) = props.hint.clone() {
                    <span class="text-[13px] text-fg-muted">{hint}</span>
                }
            }
            <Button
                variant={ButtonVariant::Primary}
                {icon}
                loading={props.busy}
                disabled={props.next_disabled}
                onclick={props.on_next.clone()}
            >
                {label}
            </Button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct VerifiedLineProps {
    pub text: AttrValue,
    #[prop_or_default]
    pub meta: Option<AttrValue>,
}

#[function_component(VerifiedLine)]
pub fn verified_line(props: &VerifiedLineProps) -> Html {
    html! {
        <div class="flex min-w-0 items-center gap-2 text-[13px] text-positive">
            <Icon name={IconName::CircleCheck} />
            <span class="truncate">{props.text.clone()}</span>
            if let Some(meta) = props.meta.clone() {
                <span class="truncate font-mono text-xs text-fg-muted">{meta}</span>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct VerifyRowProps {
    pub children: Html,
    pub on_verify: Callback<MouseEvent>,
    pub verifying: bool,
    pub disabled: bool,
}

#[function_component(VerifyRow)]
pub fn verify_row(props: &VerifyRowProps) -> Html {
    let (i18n, _) = use_translation();
    html! {
        <div class="flex items-center gap-2">
            <div class="min-w-0 flex-1">{props.children.clone()}</div>
            <Button
                onclick={props.on_verify.clone()}
                loading={props.verifying}
                disabled={props.disabled}
            >
                {i18n.t("wizard.common.verify")}
            </Button>
        </div>
    }
}
