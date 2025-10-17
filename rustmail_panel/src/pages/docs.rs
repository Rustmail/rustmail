use crate::components::language_switcher::LanguageSwitcher;
use i18nrs::yew::use_translation;
use yew::{Html, function_component, html};

#[function_component(Docs)]
pub fn docs() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <>
            <section class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                <img src="logo.png" alt="Rustmail logo" class="w-40 h-40 mb-6" />

                <a href="/"
                    class="absolute top-6 right-6 px-4 py-2 border border-gray-500 rounded-lg hover:bg-gray-800 transition">
                    { i18n.t("docs.back_to_home") }
                </a>

                <h1 class="text-3xl font-bold mb-2">{"Rustmail Panel"}</h1>
                <p class="max-w-xl text-center text-gray-400 mb-8">
                    {"Work in progress..."}
                </p>
            </section>
            <LanguageSwitcher />
        </>
    }
}
