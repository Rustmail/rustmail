use crate::components::ui::{Field, Notice, NoticeTone, input_value};
use crate::components::wizard::auth::{authed_post, handle_validation_response};
use crate::components::wizard::parts::{StepNav, VerifiedLine, VerifyRow};
use crate::components::wizard::types::{ValidateOAuth2Request, ValidateOAuth2Response, WizardData};
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step4Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
    pub on_prev: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

fn without_trailing_slash(mut url: String) -> String {
    if url.ends_with('/') {
        url.pop();
    }
    url
}

#[function_component(Step4Panel)]
pub fn step4_panel(props: &Step4Props) -> Html {
    let (i18n, _) = use_translation();
    let panel_url = use_state(|| props.data.panel_url.clone());
    let api_port = use_state(|| props.data.api_port.to_string());
    let client_id = use_state(|| props.data.client_id.clone());
    let client_secret = use_state(|| props.data.client_secret.clone());
    let redirect_url = use_state(|| props.data.redirect_url.clone());

    let is_validating = use_state(|| false);
    let validation_result = use_state(|| None::<ValidateOAuth2Response>);

    {
        let panel_url = panel_url.clone();
        let redirect_url = redirect_url.clone();

        use_effect_with((*panel_url).clone(), move |url| {
            if !url.is_empty() {
                redirect_url.set(format!("{}/api/auth/callback", url));
            }
            || ()
        });
    }

    {
        let panel_url = panel_url.clone();
        use_effect_with((), move |_| {
            if (*panel_url).is_empty()
                && let Some(window) = web_sys::window()
                && let Ok(origin) = window.location().origin()
            {
                panel_url.set(origin);
            }
            || ()
        });
    }

    let validate_oauth = {
        let client_id = client_id.clone();
        let client_secret = client_secret.clone();
        let is_validating = is_validating.clone();
        let validation_result = validation_result.clone();
        let on_unauthorized = props.on_unauthorized.clone();

        Callback::from(move |_: MouseEvent| {
            let cid = (*client_id).clone();
            let csec = (*client_secret).clone();
            if cid.trim().is_empty() || csec.trim().is_empty() {
                return;
            }

            let is_validating = is_validating.clone();
            let validation_result = validation_result.clone();
            let on_unauthorized = on_unauthorized.clone();

            is_validating.set(true);

            spawn_local(async move {
                let req_body = ValidateOAuth2Request {
                    client_id: cid,
                    client_secret: csec,
                };

                let res = authed_post("/api/setup/validate-oauth2")
                    .json(&req_body)
                    .unwrap()
                    .send()
                    .await;

                if let Some(data) =
                    handle_validation_response::<ValidateOAuth2Response>(res, &on_unauthorized)
                        .await
                {
                    validation_result.set(Some(data));
                }

                is_validating.set(false);
            });
        })
    };

    let oauth_verified = validation_result.as_ref().map(|r| r.valid).unwrap_or(false);
    let oauth_error = validation_result
        .as_ref()
        .filter(|r| !r.valid)
        .and_then(|r| r.error.clone())
        .map(AttrValue::from);
    let port_valid = (*api_port).parse::<u16>().is_ok();
    let urls_filled = !(*panel_url).trim().is_empty() && !(*redirect_url).trim().is_empty();

    let is_valid = urls_filled && !(*api_port).trim().is_empty() && port_valid && oauth_verified;

    let hint = if !oauth_verified {
        i18n.t("wizard.steps.step4.verify_hint")
    } else if !port_valid {
        i18n.t("wizard.steps.step4.port_hint")
    } else {
        i18n.t("wizard.steps.step4.urls_hint")
    };

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let data = props.data.clone();

        let panel_url = panel_url.clone();
        let api_port = api_port.clone();
        let client_id = client_id.clone();
        let client_secret = client_secret.clone();
        let redirect_url = redirect_url.clone();

        Callback::from(move |_: MouseEvent| {
            let mut new_data = data.clone();
            new_data.panel_url = (*panel_url).clone();
            new_data.api_port = (*api_port).parse().unwrap_or(3002);
            new_data.client_id = (*client_id).clone();
            new_data.client_secret = (*client_secret).clone();
            new_data.redirect_url = (*redirect_url).clone();

            props_on_update.emit(new_data);
            props_on_next.emit(());
        })
    };

    let on_prev = {
        let props_on_prev = props.on_prev.clone();
        Callback::from(move |_: MouseEvent| props_on_prev.emit(()))
    };

    let on_panel_url_change = {
        let state = panel_url.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(without_trailing_slash(input.value()));
        })
    };

    let on_port_input = {
        let state = api_port.clone();
        Callback::from(move |e: InputEvent| state.set(input_value(&e)))
    };

    let credential_input = |state: &UseStateHandle<String>| {
        let state = state.clone();
        let validation_result = validation_result.clone();
        Callback::from(move |e: InputEvent| {
            state.set(input_value(&e));
            validation_result.set(None);
        })
    };

    let on_redirect_input = {
        let state = redirect_url.clone();
        Callback::from(move |e: InputEvent| state.set(input_value(&e)))
    };

    html! {
        <>
            <div class="flex flex-col gap-6">
                <Notice tone={NoticeTone::Neutral}>
                    <span class="text-fg-secondary">{i18n.t("wizard.steps.step4.oauth_help")}</span>
                </Notice>

                <div class="grid grid-cols-1 gap-x-3 gap-y-6 sm:grid-cols-2">
                    <Field
                        label={i18n.t("wizard.steps.step4.panel_url_label")}
                        help={i18n.t("wizard.steps.step4.panel_url_help")}
                    >
                        <input
                            type="url"
                            class="input input-mono"
                            placeholder={i18n.t("wizard.steps.step4.panel_url_placeholder")}
                            value={(*panel_url).clone()}
                            onchange={on_panel_url_change}
                        />
                    </Field>
                    <Field
                        label={i18n.t("wizard.steps.step4.api_port_label")}
                        help={i18n.t("wizard.steps.step4.api_port_help")}
                        error={(!port_valid).then(|| AttrValue::from(i18n.t("wizard.steps.step4.port_hint")))}
                    >
                        <input
                            type="number"
                            min="1"
                            max="65535"
                            class="input input-mono"
                            placeholder={i18n.t("wizard.steps.step4.api_port_placeholder")}
                            value={(*api_port).clone()}
                            oninput={on_port_input}
                        />
                    </Field>
                    <Field label={i18n.t("wizard.steps.step4.client_id_label")}>
                        <input
                            type="text"
                            inputmode="numeric"
                            class="input input-mono"
                            placeholder={i18n.t("wizard.steps.step4.client_id_placeholder")}
                            value={(*client_id).clone()}
                            oninput={credential_input(&client_id)}
                        />
                    </Field>
                    <Field label={i18n.t("wizard.steps.step4.client_secret_label")}>
                        <VerifyRow
                            on_verify={validate_oauth}
                            verifying={*is_validating}
                            disabled={client_id.trim().is_empty() || client_secret.trim().is_empty()}
                        >
                            <input
                                type="password"
                                autocomplete="off"
                                class="input input-mono"
                                placeholder={i18n.t("wizard.steps.step4.client_secret_placeholder")}
                                value={(*client_secret).clone()}
                                oninput={credential_input(&client_secret)}
                            />
                        </VerifyRow>
                    </Field>
                </div>

                if oauth_verified {
                    <VerifiedLine text={i18n.t("wizard.steps.step4.oauth_valid")} />
                } else if let Some(error) = oauth_error {
                    <span class="text-xs text-negative">{error}</span>
                }

                <Field
                    label={i18n.t("wizard.steps.step4.redirect_url_label")}
                    help={i18n.t("wizard.steps.step4.redirect_url_help")}
                >
                    <input
                        type="url"
                        class="input input-mono"
                        placeholder={i18n.t("wizard.steps.step4.redirect_url_placeholder")}
                        value={(*redirect_url).clone()}
                        oninput={on_redirect_input}
                    />
                </Field>
            </div>
            <StepNav
                on_back={on_prev}
                {on_next}
                next_disabled={!is_valid}
                {hint}
            />
        </>
    }
}
