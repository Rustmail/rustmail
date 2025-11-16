use crate::components::language_switcher::LanguageSwitcher;
use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {

        <>
            <section class="relative flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                <img src="logo.png" alt="Rustmail logo" class="w-40 h-40 mb-6" />
                <h1 class="text-3xl font-bold mb-2">{"Rustmail Panel"}</h1>
                <p class="max-w-xl text-center text-gray-400 mb-8">
                    { i18n.t("home.welcome") }
                </p>
                <div class="flex gap-4">
                    <a
                        href="/api/auth/login"
                        class="px-4 py-2 rounded-lg transition"
                        style="background-color: #4f6fd7;"
                    >
                        { i18n.t("home.connect_button") }
                    </a>
                </div>
            </section>
            <LanguageSwitcher placement="top-left" />
        </>
    }
}
