use crate::components::wizard::types::WizardData;
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

    let is_valid = !(*panel_url).trim().is_empty()
        && !(*api_port).trim().is_empty()
        && !(*client_id).trim().is_empty()
        && !(*client_secret).trim().is_empty()
        && (*api_port).parse::<u16>().is_ok();

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let data = props.data.clone();

        let panel_url = panel_url.clone();
        let api_port = api_port.clone();
        let client_id = client_id.clone();
        let client_secret = client_secret.clone();

        Callback::from(move |_| {
            let mut new_data = data.clone();
            new_data.panel_url = (*panel_url).clone();
            new_data.api_port = (*api_port).parse().unwrap_or(8080);
            new_data.client_id = (*client_id).clone();
            new_data.client_secret = (*client_secret).clone();

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
                    { "The web panel requires Discord OAuth2 to authenticate staff members. Make sure to add " }
                    <strong class="text-indigo-200">{ format!("{}/api/auth/callback", if (*panel_url).is_empty() { "YOUR_URL" } else { &*panel_url }) }</strong>
                    { " to your OAuth2 Redirect URIs in the Discord Developer Portal." }
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-300">{ "Panel Base URL" }</label>
                    <input
                        type="url"
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="e.g. https://panel.rustmail.rs"
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
                    <label class="text-sm font-medium text-gray-300">{ "API Server Port" }</label>
                    <input
                        type="number"
                        min="1" max="65535"
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="e.g. 8080"
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
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                            })
                        }
                    />
                </div>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-300">{ "Discord OAuth2 Client Secret" }</label>
                    <input
                        type="password"
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="Found in Developer Portal"
                        value={(*client_secret).clone()}
                        onchange={
                            let state = client_secret.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                            })
                        }
                    />
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
