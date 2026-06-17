use crate::components::wizard::types::{ValidateTokenRequest, ValidateTokenResponse, WizardData};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step1Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
}

#[function_component(Step1Token)]
pub fn step1_token(props: &Step1Props) -> Html {
    let token = use_state(|| props.data.token.clone());
    let is_validating = use_state(|| false);
    let validation_result = use_state(|| None::<ValidateTokenResponse>);

    let on_token_change = {
        let token = token.clone();
        let validation_result = validation_result.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            token.set(input.value());
            validation_result.set(None); // Reset validation on typing
        })
    };

    let validate_token = {
        let token = token.clone();
        let is_validating = is_validating.clone();
        let validation_result = validation_result.clone();

        Callback::from(move |_| {
            let token_val = (*token).clone();
            if token_val.trim().is_empty() {
                return;
            }

            let is_validating = is_validating.clone();
            let validation_result = validation_result.clone();

            is_validating.set(true);

            spawn_local(async move {
                let req_body = ValidateTokenRequest { token: token_val };

                let res = Request::post("/api/setup/validate-token")
                    .json(&req_body)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) => {
                        if let Ok(data) = resp.json::<ValidateTokenResponse>().await {
                            validation_result.set(Some(data));
                        } else {
                            validation_result.set(Some(ValidateTokenResponse {
                                valid: false,
                                bot: None,
                                error: Some("Invalid response from server".to_string()),
                            }));
                        }
                    }
                    Err(_) => {
                        validation_result.set(Some(ValidateTokenResponse {
                            valid: false,
                            bot: None,
                            error: Some("Network error".to_string()),
                        }));
                    }
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

        Callback::from(move |_| {
            let mut new_data = current_data.clone();
            new_data.token = (*token).clone();
            props_on_update.emit(new_data);
            props_on_next.emit(());
        })
    };

    let is_valid = validation_result.as_ref().map(|r| r.valid).unwrap_or(false);

    html! {
        <div class="flex flex-col gap-6 animate-fade-in">
            <div class="flex flex-col gap-2">
                <label class="text-sm font-medium text-gray-300">{ "Discord Bot Token" }</label>
                <div class="flex gap-3">
                    <input
                        type="password"
                        class="flex-1 bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="Paste your bot token here..."
                        value={(*token).clone()}
                        onchange={on_token_change}
                    />
                    <button
                        class="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-white font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center min-w-[100px]"
                        onclick={validate_token}
                        disabled={*is_validating || token.trim().is_empty()}
                    >
                        if *is_validating {
                            <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                        } else {
                            { "Validate" }
                        }
                    </button>
                </div>
                <p class="text-xs text-gray-500 mt-1">
                    { "You can get your token from the " }
                    <a href="https://discord.com/developers/applications" target="_blank" class="text-indigo-400 hover:text-indigo-300 underline decoration-indigo-400/30 underline-offset-2">
                        { "Discord Developer Portal" }
                    </a>
                    { "." }
                </p>
            </div>

            // Validation Result Area
            if let Some(result) = (*validation_result).as_ref() {
                if result.valid {
                    if let Some(bot) = &result.bot {
                        <div class="bg-green-500/10 border border-green-500/20 rounded-xl p-4 flex items-center gap-4">
                            if let Some(avatar) = &bot.avatar {
                                <img src={format!("https://cdn.discordapp.com/avatars/{}/{}.png", bot.id, avatar)} class="w-12 h-12 rounded-full ring-2 ring-green-500/30" />
                            } else {
                                <div class="w-12 h-12 rounded-full bg-slate-800 flex items-center justify-center ring-2 ring-green-500/30">
                                    <span class="text-gray-400 text-lg font-bold">{ bot.username.chars().next().unwrap_or('?') }</span>
                                </div>
                            }
                            <div>
                                <h3 class="text-green-400 font-medium">{ "Connected Successfully!" }</h3>
                                <p class="text-sm text-gray-300">{ format!("Logged in as ") }<span class="font-semibold text-white">{ &bot.username }</span></p>
                            </div>
                        </div>
                    }
                } else if let Some(error) = &result.error {
                    <div class="bg-red-500/10 border border-red-500/20 rounded-xl p-4 flex items-start gap-3">
                        <svg class="w-5 h-5 text-red-400 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                        <div>
                            <h3 class="text-red-400 font-medium">{ "Validation Failed" }</h3>
                            <p class="text-sm text-red-300/80 mt-0.5">{ error }</p>
                        </div>
                    </div>
                }
            }

            <div class="flex justify-end pt-4 mt-2 border-t border-slate-800/50">
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
