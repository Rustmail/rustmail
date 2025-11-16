mod components;
mod pages;
mod router;
mod utils;

use crate::router::{Route, switch};
use i18nrs::StorageType;
use i18nrs::yew::I18nProvider;
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
            </BrowserRouter>
        </I18nProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
