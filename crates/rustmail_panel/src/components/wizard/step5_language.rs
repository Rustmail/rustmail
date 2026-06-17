use crate::components::wizard::types::WizardData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step5Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
    pub on_prev: Callback<()>,
}

#[function_component(Step5Language)]
pub fn step5_language(props: &Step5Props) -> Html {
    let locale = use_state(|| props.data.locale.clone());
    let timezone = use_state(|| props.data.timezone.clone());
    let status = use_state(|| props.data.status.clone());
    let direct_message = use_state(|| props.data.direct_message.clone());

    let is_valid = !(*status).trim().is_empty() && !(*direct_message).trim().is_empty();

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let data = props.data.clone();

        let locale = locale.clone();
        let timezone = timezone.clone();
        let status = status.clone();
        let direct_message = direct_message.clone();

        Callback::from(move |_| {
            let mut new_data = data.clone();
            new_data.locale = (*locale).clone();
            new_data.timezone = (*timezone).clone();
            new_data.status = (*status).clone();
            new_data.direct_message = (*direct_message).clone();

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
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-300">{ "Language" }</label>
                    <select
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors cursor-pointer appearance-none"
                        value={(*locale).clone()}
                        onchange={
                            let state = locale.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                state.set(input.value());
                            })
                        }
                    >
                        <option value="en">{ "English (en)" }</option>
                        <option value="fr">{ "Français (fr)" }</option>
                    </select>
                </div>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-300">{ "Timezone" }</label>
                    <select
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors cursor-pointer appearance-none"
                        value={(*timezone).clone()}
                        onchange={
                            let state = timezone.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                state.set(input.value());
                            })
                        }
                    >
                        <option value="UTC">{ "UTC" }</option>
                        <option value="Europe/Paris">{ "Europe/Paris" }</option>
                        <option value="Europe/London">{ "Europe/London" }</option>
                        <option value="Europe/Berlin">{ "Europe/Berlin" }</option>
                        <option value="America/New_York">{ "America/New_York" }</option>
                        <option value="America/Los_Angeles">{ "America/Los_Angeles" }</option>
                        <option value="Asia/Tokyo">{ "Asia/Tokyo" }</option>
                        <option value="Australia/Sydney">{ "Australia/Sydney" }</option>
                    </select>
                </div>
            </div>

            <div class="flex flex-col gap-4 bg-slate-800/30 p-5 rounded-xl border border-slate-700/50 mt-2">
                <h3 class="text-white font-medium mb-1">{ "Bot Messages" }</h3>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-gray-400">{ "Bot Status / Activity" }</label>
                    <input
                        type="text"
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="e.g. Need help? DM me!"
                        value={(*status).clone()}
                        onchange={
                            let state = status.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                            })
                        }
                    />
                    <p class="text-xs text-gray-500">{ "Displayed under the bot's name in the member list." }</p>
                </div>

                <div class="flex flex-col gap-2 mt-2">
                    <label class="text-sm font-medium text-gray-400">{ "Welcome Message (DM)" }</label>
                    <textarea
                        class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors resize-none h-24"
                        placeholder="e.g. Thank you for contacting support! A staff member will be with you shortly."
                        value={(*direct_message).clone()}
                        onchange={
                            let state = direct_message.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                state.set(input.value());
                            })
                        }
                    />
                    <p class="text-xs text-gray-500">{ "This message is sent to the user when they open a new ticket." }</p>
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
