use crate::components::wizard::types::{ValidateOAuth2Request, ValidateOAuth2Response, WizardData};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step4Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
    pub on_prev: Callback<()>,
}

#[function_component(Step4Panel)]
pub fn step4_panel(props: &Step4Props) -> Html {
    let panel_url = use_state(|| props.data.panel_url.clone());
    let api_port = use_state(|| props.data.api_port.to_string());
    let client_id = use_state(|| props.data.client_id.clone());
    let client_secret = use_state(|| props.data.client_secret.clone());
    let redirect_url = use_state(|| props.data.redirect_url.clone());

    let is_validating = use_state(|| false);
    let validation_result = use_state(|| None::<ValidateOAuth2Response>);

    // Auto-update redirect_url when panel_url changes
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

    // Auto-fill panel_url with window.location.origin on mount if empty
    {
        let panel_url = panel_url.clone();
        use_effect_with((), move |_| {
            if (*panel_url).is_empty() {
                if let Some(window) = web_sys::window() {
                    if let Ok(origin) = window.location().origin() {
                        panel_url.set(origin);
                    }
                }
            }
            || ()
        });
    }

    let validate_oauth = {
        let client_id = client_id.clone();
        let client_secret = client_secret.clone();
        let is_validating = is_validating.clone();
        let validation_result = validation_result.clone();

        Callback::from(move |_| {
            let cid = (*client_id).clone();
            let csec = (*client_secret).clone();
            if cid.trim().is_empty() || csec.trim().is_empty() {
                return;
            }

            let is_validating = is_validating.clone();
            let validation_result = validation_result.clone();

            is_validating.set(true);

            spawn_local(async move {
                let req_body = ValidateOAuth2Request {
                    client_id: cid,
                    client_secret: csec,
                };

                let res = Request::post("/api/setup/validate-oauth2")
                    .json(&req_body)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) => {
                        if let Ok(data) = resp.json::<ValidateOAuth2Response>().await {
                            validation_result.set(Some(data));
                        } else {
                            validation_result.set(Some(ValidateOAuth2Response {
                                valid: false,
                                error: Some("Invalid response from server".to_string()),
                            }));
                        }
                    }
                    Err(_) => {
                        validation_result.set(Some(ValidateOAuth2Response {
                            valid: false,
                            error: Some("Network error".to_string()),
                        }));
                    }
                }

                is_validating.set(false);
            });
        })
    };

    let is_valid = !(*panel_url).trim().is_empty()
        && !(*api_port).trim().is_empty()
        && !(*redirect_url).trim().is_empty()
        && (*api_port).parse::<u16>().is_ok()
        && validation_result.as_ref().map(|r| r.valid).unwrap_or(false);

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let data = props.data.clone();

        let panel_url = panel_url.clone();
        let api_port = api_port.clone();
        let client_id = client_id.clone();
        let client_secret = client_secret.clone();
        let redirect_url = redirect_url.clone();

        Callback::from(move |_| {
            let mut new_data = data.clone();
            new_data.panel_url = (*panel_url).clone();
            new_data.api_port = (*api_port).parse().unwrap_or(8080);
            new_data.client_id = (*client_id).clone();
            new_data.client_secret = (*client_secret).clone();
            new_data.redirect_url = (*redirect_url).clone();

            props_on_update.emit(new_data);
            props_on_next.emit(());
        })
    };

    let on_prev = {
        let props_on_prev = props.on_prev.clone();
        Callback::from(move |_| {
            props_on_prev.emit(());
        })
    };

    html! {
        <div class="flex flex-col gap-6 animate-fade-in">
            <div class="bg-indigo-500/10 border border-indigo-500/20 rounded-xl p-4 flex gap-3 text-sm text-indigo-300">
                <svg class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                <p>
                    { "The web panel requires Discord OAuth2 to authenticate staff members. Make sure to add the " }
                    <strong class="text-indigo-200">{ "Redirect URL" }</strong>
                    { " below to your OAuth2 Redirect URIs in the Discord Developer Portal." }
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-300">{ "External Panel URL" }</label>
                    <input
                        type="url"
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="e.g. http://192.168.1.10:8080 or https://panel.domain.com"
                        value={(*panel_url).clone()}
                        onchange={
                            let state = panel_url.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let mut val = input.value();
                                // Remove trailing slash if present
                                if val.ends_with('/') {
                                    val.pop();
                                }
                                state.set(val);
                            })
                        }
                    />
                </div>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-300">{ "Internal API Port (Docker Bind)" }</label>
                    <input
                        type="number"
                        min="1" max="65535"
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="e.g. 3002 (Default)"
                        value={(*api_port).clone()}
                        onchange={
                            let state = api_port.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                            })
                        }
                    />
                </div>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-300">{ "Discord OAuth2 Client ID" }</label>
                    <input
                        type="text"
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="Found in Developer Portal"
                        value={(*client_id).clone()}
                        onchange={
                            let state = client_id.clone();
                            let validation_result = validation_result.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                                validation_result.set(None);
                            })
                        }
                    />
                </div>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-300">{ "Discord OAuth2 Client Secret" }</label>
                    <div class="flex gap-3">
                        <input
                            type="password"
                            class="flex-1 bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                            placeholder="Found in Developer Portal"
                            value={(*client_secret).clone()}
                            onchange={
                                let state = client_secret.clone();
                                let validation_result = validation_result.clone();
                                Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    state.set(input.value());
                                    validation_result.set(None);
                                })
                            }
                        />
                        <button
                            class="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-white font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center min-w-[100px]"
                            onclick={validate_oauth}
                            disabled={*is_validating || client_id.trim().is_empty() || client_secret.trim().is_empty()}
                        >
                            if *is_validating {
                                <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                            } else {
                                { "Verify" }
                            }
                        </button>
                    </div>
                </div>

                if let Some(result) = (*validation_result).as_ref() {
                    <div class="md:col-span-2 mt-1">
                        if result.valid {
                            <div class="bg-green-500/10 border border-green-500/20 rounded-xl p-3 flex items-center gap-3">
                                <svg class="w-5 h-5 text-green-400 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                <p class="text-green-400 text-sm font-medium">{ "OAuth2 credentials verified successfully!" }</p>
                            </div>
                        } else if let Some(error) = &result.error {
                            <div class="bg-red-500/10 border border-red-500/20 rounded-xl p-3 flex items-start gap-3">
                                <svg class="w-5 h-5 text-red-400 mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                <div>
                                    <p class="text-red-400 text-sm font-medium">{ "Verification Failed" }</p>
                                    <p class="text-xs text-red-300/80 mt-0.5">{ error }</p>
                                </div>
                            </div>
                        }
                    </div>
                }

                <div class="flex flex-col gap-2 md:col-span-2">
                    <label class="text-sm font-medium text-gray-300">{ "OAuth2 Redirect URL" }</label>
                    <input
                        type="url"
                        class="bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="e.g. https://panel.rustmail.rs/api/auth/callback"
                        value={(*redirect_url).clone()}
                        onchange={
                            let state = redirect_url.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                            })
                        }
                    />
                    <p class="text-xs text-gray-500">{ "This is automatically calculated based on your panel URL and port, but you can override it if you use a reverse proxy." }</p>
                </div>
            </div>

            <div class="flex justify-between pt-4 mt-2 border-t border-slate-800/50">
                <button
                    class="px-6 py-2.5 bg-slate-800 hover:bg-slate-700 text-white font-medium rounded-lg transition-colors"
                    onclick={on_prev}
                >
                    { "Back" }
                </button>
                <button
                    class="px-6 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-white font-medium rounded-lg transition-colors shadow-lg shadow-indigo-600/20 disabled:opacity-50 disabled:cursor-not-allowed"
                    onclick={on_next}
                    disabled={!is_valid}
                >
                    { "Next Step" }
                </button>
            </div>
        </div>
    }
}
