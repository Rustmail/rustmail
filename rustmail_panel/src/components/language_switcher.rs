use i18nrs::yew::use_translation;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LanguageSwitcherProps {
    #[prop_or("top-left".to_string())]
    pub placement: String,
}

#[function_component(LanguageSwitcher)]
pub fn language_switcher(props: &LanguageSwitcherProps) -> Html {
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
            if let Some(select) = language_ref.cast::<web_sys::HtmlSelectElement>() {
                let value = select.value();
                language_state.set(value.clone());
                set_language.emit(value);
            }
        })
    };

    let container_classes = if props.placement == "nav" {
        "relative inline-flex items-center ml-4"
    } else {
        "absolute top-6 left-6"
    };

    html! {
        <div class={container_classes}>
            <div class="relative inline-flex items-center">
                <select
                    ref={language_ref}
                    value={(*language_state).clone()}
                    onchange={onchange}
                    class="
                        appearance-none
                        bg-gray-900/90 text-gray-200
                        border border-white/10
                        rounded-xl
                        px-3 py-2 pr-8
                        text-sm font-medium
                        hover:bg-gray-800/80
                        focus:outline-none focus:ring-2 focus:ring-blue-500
                        transition
                        cursor-pointer
                        backdrop-blur-md
                    "
                >
                    <option value="en">{ "🇺🇸 English" }</option>
                    <option value="fr">{ "🇫🇷 Français" }</option>
                </select>

                <div class="pointer-events-none absolute inset-y-0 right-2 flex items-center text-gray-400">
                    <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 10.94l3.71-3.71a.75.75 0 011.08 1.04l-4.25 4.25a.75.75 0 01-1.08 0L5.21 8.27a.75.75 0 01.02-1.06z" clip-rule="evenodd"/>
                    </svg>
                </div>
            </div>
        </div>
    }
}
