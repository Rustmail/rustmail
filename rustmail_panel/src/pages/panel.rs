use crate::components::navbar::RustmailNavbar;
use gloo_net::http::Request;
use gloo_utils::window;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::configuration::ConfigurationPage;
use crate::components::ticket::TicketsPage;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct UserAvatar {
    pub avatar_url: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum Page {
    Home,
    Configuration,
    Tickets,
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
        let authorized = authorized.clone();
        use_effect_with((*authorized).clone(), move |auth| {
            if matches!(auth, Some(false)) {
                let _ = window().location().set_href("/");
            }
            || ()
        });
    }

    let avatar = use_state(|| None::<String>);
    {
        let avatar = avatar.clone();
        use_effect_with((), move |()| {
            let avatar = avatar.clone();

            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/user/avatar").send().await {
                    if let Ok(user_avatar) = resp.json::<UserAvatar>().await {
                        avatar.set(user_avatar.avatar_url);
                    } else {
                        avatar.set(None);
                    }
                } else {
                    avatar.set(None);
                }
            });
            || ()
        });
    }

    let avatar_url = (*avatar).clone().unwrap_or_default();
    let current_page = use_state(|| Page::Home);

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
                        <>
                            <RustmailNavbar
                                avatar_url={avatar_url.clone()}
                                current_page={(*current_page).clone()}
                                on_page_change={Callback::from({
                                    let current_page = current_page.clone();
                                    move |page: Page| current_page.set(page)
                                })}
                            />

                            <section class="pt-24 min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                                <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                                    {
                                        match &*current_page {
                                            Page::Home => html! {
                                                <div class="flex flex-col items-center justify-center text-center">
                                                    <img src="logo.png" alt="Rustmail logo" class="w-40 h-40 mb-6" />
                                                    <h1 class="text-3xl font-bold mb-2">{"Rustmail Panel"}</h1>
                                                    <p class="max-w-xl text-gray-400 mb-8">
                                                        { i18n.t("panel.welcome") }
                                                    </p>
                                                </div>
                                            },
                                            Page::Configuration => html! {
                                                <ConfigurationPage />
                                            },
                                            Page::Tickets => html! {
                                                <TicketsPage />
                                            },
                                        }
                                    }
                                </main>
                            </section>
                        </>
                    },
                    Some(false) => html! {},
                }
            }
        </>
    }
}
