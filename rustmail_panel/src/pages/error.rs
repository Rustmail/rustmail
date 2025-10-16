use gloo_utils::window;
use i18nrs::yew::use_translation;
use web_sys::UrlSearchParams;
use yew::prelude::*;

#[function_component(Error)]
pub fn error() -> Html {
    let (i18n, _set_language) = use_translation();

    let location = window().location();
    let search = location.search().unwrap_or_default();

    let params = UrlSearchParams::new_with_str(&search).unwrap();
    let message = params
        .get("message")
        .unwrap_or_else(|| "Unknown error".into());

    html! {
        <div class="error-page flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
            <a href="/" class="absolute top-6 right-6 px-4 py-2 border border-gray-500 rounded-lg hover:bg-gray-800 transition">
                { i18n.t("back_to_home") }
            </a>
            <h1>{ i18n.t("error.error") }</h1>
            <p>{ message }</p>
        </div>
    }
}
