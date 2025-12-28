use crate::i18n::yew::use_translation;
use yew::{Html, function_component, html};

#[function_component(LogoutButton)]
pub fn logout_button() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <a href="/api/auth/logout"
            class="absolute top-6 right-6 px-4 py-2 border border-gray-500 rounded-lg hover:bg-gray-800 transition">
            { i18n.t("panel.logout") }
        </a>
    }
}
