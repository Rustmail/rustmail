use crate::components::ui::{Field, input_value};
use crate::components::wizard::auth::{authed_post, handle_validation_response};
use crate::components::wizard::parts::{StepNav, VerifiedLine, VerifyRow};
use crate::components::wizard::types::{ValidateTokenRequest, ValidateTokenResponse, WizardData};
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step1Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

#[function_component(Step1Token)]
pub fn step1_token(props: &Step1Props) -> Html {
    let (i18n, _) = use_translation();
    let token = use_state(|| props.data.token.clone());
    let is_validating = use_state(|| false);
    let validation_result = use_state(|| None::<ValidateTokenResponse>);

    let on_token_input = {
        let token = token.clone();
        let validation_result = validation_result.clone();
        Callback::from(move |e: InputEvent| {
            token.set(input_value(&e));
            validation_result.set(None);
        })
    };

    let validate_token = {
        let token = token.clone();
        let is_validating = is_validating.clone();
        let validation_result = validation_result.clone();
        let on_unauthorized = props.on_unauthorized.clone();

        Callback::from(move |_: MouseEvent| {
            let token_val = (*token).clone();
            if token_val.trim().is_empty() {
                return;
            }

            let is_validating = is_validating.clone();
            let validation_result = validation_result.clone();
            let on_unauthorized = on_unauthorized.clone();

            is_validating.set(true);

            spawn_local(async move {
                let req_body = ValidateTokenRequest { token: token_val };

                let res = authed_post("/api/setup/validate-token")
                    .json(&req_body)
                    .unwrap()
                    .send()
                    .await;

                if let Some(data) =
                    handle_validation_response::<ValidateTokenResponse>(res, &on_unauthorized).await
                {
                    validation_result.set(Some(data));
                }

                is_validating.set(false);
            });
        })
    };

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let token = token.clone();
        let current_data = props.data.clone();

        Callback::from(move |_: MouseEvent| {
            let mut new_data = current_data.clone();
            new_data.token = (*token).clone();
            props_on_update.emit(new_data);
            props_on_next.emit(());
        })
    };

    let result = (*validation_result).clone();
    let is_valid = result.as_ref().map(|r| r.valid).unwrap_or(false);
    let error = result
        .as_ref()
        .filter(|r| !r.valid)
        .and_then(|r| r.error.clone())
        .map(AttrValue::from);
    let bot = result.filter(|r| r.valid).and_then(|r| r.bot);

    html! {
        <>
            <div class="flex flex-col gap-3">
                <Field
                    label={i18n.t("wizard.steps.step1.token_label")}
                    help={i18n.t("wizard.steps.step1.token_help")}
                    {error}
                >
                    <VerifyRow
                        on_verify={validate_token}
                        verifying={*is_validating}
                        disabled={token.trim().is_empty()}
                    >
                        <input
                            type="password"
                            class="input input-mono"
                            autocomplete="off"
                            placeholder={i18n.t("wizard.steps.step1.token_placeholder")}
                            value={(*token).clone()}
                            oninput={on_token_input}
                        />
                    </VerifyRow>
                </Field>
                if let Some(bot) = bot {
                    <VerifiedLine
                        text={i18n.t("wizard.steps.step1.logged_in_as").replace("{name}", &bot.username)}
                        meta={bot.id.clone()}
                    />
                }
            </div>
            <StepNav
                {on_next}
                next_disabled={!is_valid}
                hint={i18n.t("wizard.steps.step1.verify_hint")}
            />
        </>
    }
}
