use crate::components::language_switcher::LanguageSwitcher;
use crate::components::logout_button::LogoutButton;
use gloo_net::http::Request;
use gloo_utils::window;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum PanelTab {
    Config,
    Logs,
}

#[function_component(Panel)]
pub fn panel() -> Html {
    let (i18n, _set_language) = i18nrs::yew::use_translation();

    let authorized = use_state(|| None::<bool>);

    {
        let authorized = authorized.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let ok = Request::get("/api/panel/check")
                    .send()
                    .await
                    .map(|r| r.status() == 200)
                    .unwrap_or(false);
                authorized.set(Some(ok));
            });
            || ()
        });
    }

    {
        let dep = (*authorized).clone();
        use_effect_with(dep, move |auth: &Option<bool>| {
            if matches!(auth, Some(false)) {
                let _ = window().location().set_href("/");
            }
            || ()
        });
    }

    html! {
        <>
            {
                match *authorized {
                    None => html! {
                        <section class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                            <p class="text-gray-400 animate-pulse">{"Vérification de l'accès..."}</p>
                        </section>
                    },
                    Some(true) => html! {
                        <section class="relative flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                            <img src="logo.png" alt="Rustmail logo" class="w-40 h-40 mb-6" />

                            <LogoutButton />

                            <h1 class="text-3xl font-bold mb-2">{"Rustmail Panel"}</h1>
                            <p class="max-w-xl text-center text-gray-400 mb-8">
                                { i18n.t("panel.welcome") }
                            </p>
                        </section>
                    },
                    Some(false) => html! {},
                }
            }
            <LanguageSwitcher />
        </>
    }
}
