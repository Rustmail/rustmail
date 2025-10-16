mod components;
mod pages;
mod router;

use crate::components::language_switcher::LanguageSwitcher;
use crate::router::{switch, Route};
use i18nrs::yew::I18nProvider;
use i18nrs::StorageType;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    let translations = HashMap::from([
        ("en", include_str!("i18n/en/en.json")),
        ("fr", include_str!("i18n/fr/fr.json")),
    ]);

    html! {
        <I18nProvider
            translations={translations}
            storage_type={StorageType::LocalStorage}
            storage_name={"i18nrs".to_string()}
            default_language={"en".to_string()}
        >
            <BrowserRouter>
                <Switch<Route> render={switch} />
                <LanguageSwitcher />
            </BrowserRouter>
        </I18nProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
