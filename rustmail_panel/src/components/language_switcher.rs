use i18nrs::yew::use_translation;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(LanguageSwitcher)]
pub fn language_switcher() -> Html {
    let (i18n, set_language) = use_translation();

    let language_ref = use_node_ref();
    let language_state = use_state(|| "".to_string());

    {
        let language_state = language_state.clone();
        let i18n = i18n.clone();
        use_effect(move || {
            let current = i18n.get_current_language().to_string();
            if *language_state != current {
                language_state.set(current);
            }
            || ()
        });
    }

    let onchange = {
        let language_ref = language_ref.clone();
        let language_state = language_state.clone();
        let set_language = set_language.clone();

        Callback::from(move |_| {
            if let Some(input) = language_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                language_state.set(value.clone());
                set_language.emit(input.value());
            }
        })
    };

    html! {
        <div class="absolute top-6 left-6">
            <div class="relative inline-block">
                <select
                    class="
                        px-4 py-2
                        border border-gray-500
                        rounded-lg
                        bg-gray-900 text-white
                        hover:bg-gray-800
                        transition
                        appearance-none
                        focus:outline-none
                        cursor-pointer
                        pr-8
                    "
                    ref={language_ref}
                    value={(*language_state).clone()}
                    onchange={onchange}
                >
                    <option value="en">{ "🇺🇸 English" }</option>
                    <option value="fr">{ "🇫🇷 French" }</option>
                </select>
                <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-white">
                    { "▼" }
                </div>
            </div>
        </div>
    }
}
