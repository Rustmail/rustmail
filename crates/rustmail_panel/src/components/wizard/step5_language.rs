use crate::components::ui::{Field, Select, ToggleRow, input_value, textarea_value};
use crate::components::wizard::parts::StepNav;
use crate::components::wizard::types::WizardData;
use crate::i18n::yew::use_translation;
use yew::prelude::*;

pub const LOCALES: [(&str, &str); 2] = [("en", "English (en)"), ("fr", "Français (fr)")];

const TIMEZONES: [&str; 8] = [
    "UTC",
    "Europe/Paris",
    "Europe/London",
    "Europe/Berlin",
    "America/New_York",
    "America/Los_Angeles",
    "Asia/Tokyo",
    "Australia/Sydney",
];

#[derive(Properties, PartialEq)]
pub struct Step5Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
    pub on_prev: Callback<()>,
}

#[function_component(Step5Language)]
pub fn step5_language(props: &Step5Props) -> Html {
    let (i18n, _) = use_translation();
    let locale = use_state(|| props.data.locale.clone());
    let timezone = use_state(|| props.data.timezone.clone());
    let status = use_state(|| props.data.status.clone());
    let direct_message = use_state(|| props.data.direct_message.clone());
    let close_message = use_state(|| props.data.close_message.clone());
    let check_updates = use_state(|| props.data.check_updates);

    let is_valid = !(*status).trim().is_empty()
        && !(*direct_message).trim().is_empty()
        && !(*close_message).trim().is_empty();

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let data = props.data.clone();

        let locale = locale.clone();
        let timezone = timezone.clone();
        let status = status.clone();
        let direct_message = direct_message.clone();
        let close_message = close_message.clone();
        let check_updates = check_updates.clone();

        Callback::from(move |_: MouseEvent| {
            let mut new_data = data.clone();
            new_data.locale = (*locale).clone();
            new_data.timezone = (*timezone).clone();
            new_data.status = (*status).clone();
            new_data.direct_message = (*direct_message).clone();
            new_data.close_message = (*close_message).clone();
            new_data.check_updates = *check_updates;

            props_on_update.emit(new_data);
            props_on_next.emit(());
        })
    };

    let on_prev = {
        let props_on_prev = props.on_prev.clone();
        Callback::from(move |_: MouseEvent| props_on_prev.emit(()))
    };

    let select_setter = |state: &UseStateHandle<String>| {
        let state = state.clone();
        Callback::from(move |value: String| state.set(value))
    };
    let textarea_setter = |state: &UseStateHandle<String>| {
        let state = state.clone();
        Callback::from(move |e: InputEvent| state.set(textarea_value(&e)))
    };
    let on_status_input = {
        let state = status.clone();
        Callback::from(move |e: InputEvent| state.set(input_value(&e)))
    };
    let on_check_updates = {
        let state = check_updates.clone();
        Callback::from(move |checked: bool| state.set(checked))
    };

    let locale_options: Vec<(AttrValue, AttrValue)> = LOCALES
        .iter()
        .map(|(code, name)| (AttrValue::from(*code), AttrValue::from(*name)))
        .collect();
    let timezone_options: Vec<(AttrValue, AttrValue)> = TIMEZONES
        .iter()
        .map(|zone| (AttrValue::from(*zone), AttrValue::from(*zone)))
        .collect();

    html! {
        <>
            <div class="flex flex-col gap-6">
                <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
                    <Field label={i18n.t("wizard.steps.step5.language_label")}>
                        <Select
                            value={AttrValue::from((*locale).clone())}
                            options={locale_options}
                            onchange={select_setter(&locale)}
                        />
                    </Field>
                    <Field
                        label={i18n.t("wizard.steps.step5.timezone_label")}
                        help={i18n.t("wizard.steps.step5.timezone_help")}
                    >
                        <Select
                            value={AttrValue::from((*timezone).clone())}
                            options={timezone_options}
                            onchange={select_setter(&timezone)}
                        />
                    </Field>
                </div>

                <Field
                    label={i18n.t("wizard.steps.step5.bot_status_label")}
                    help={i18n.t("wizard.steps.step5.bot_status_help")}
                >
                    <input
                        type="text"
                        class="input"
                        placeholder={i18n.t("wizard.steps.step5.bot_status_placeholder")}
                        value={(*status).clone()}
                        oninput={on_status_input}
                    />
                </Field>

                <Field
                    label={i18n.t("wizard.steps.step5.welcome_label")}
                    help={i18n.t("wizard.steps.step5.welcome_help")}
                >
                    <textarea
                        rows="3"
                        class="textarea resize-none"
                        placeholder={i18n.t("wizard.steps.step5.welcome_placeholder")}
                        value={(*direct_message).clone()}
                        oninput={textarea_setter(&direct_message)}
                    />
                </Field>

                <Field
                    label={i18n.t("wizard.steps.step5.close_label")}
                    help={i18n.t("wizard.steps.step5.close_help")}
                >
                    <textarea
                        rows="3"
                        class="textarea resize-none"
                        placeholder={i18n.t("wizard.steps.step5.close_placeholder")}
                        value={(*close_message).clone()}
                        oninput={textarea_setter(&close_message)}
                    />
                </Field>

                <ToggleRow
                    label={i18n.t("wizard.steps.step5.check_updates_label")}
                    help={i18n.t("wizard.steps.step5.check_updates_help")}
                    checked={*check_updates}
                    onchange={on_check_updates}
                />
            </div>
            <StepNav
                on_back={on_prev}
                {on_next}
                next_disabled={!is_valid}
                hint={i18n.t("wizard.steps.step5.messages_hint")}
            />
        </>
    }
}
