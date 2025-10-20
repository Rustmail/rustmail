use crate::components::configuration::ConfigurationPage;
use crate::components::home::Home;
use crate::components::navbar::RustmailNavbar;
use crate::components::ticket::{TicketDetails, TicketsList};
use gloo_net::http::Request;
use gloo_utils::window;
use i18nrs::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::navigator::Navigator;
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct UserAvatar {
    pub avatar_url: Option<String>,
}

#[derive(Clone, Routable, PartialEq)]
pub enum PanelRoute {
    #[at("/panel")]
    Home,
    #[at("/panel/configuration")]
    Configuration,
    #[at("/panel/tickets")]
    TicketsList,
    #[at("/panel/tickets/:id")]
    TicketDetails { id: String },
}

#[function_component(Panel)]
pub fn panel() -> Html {
    let (i18n, _set_language) = use_translation();

    let authorized = use_state(|| None::<bool>);
    let navigator = use_navigator();
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

    html! {
        <>
            {
                match *authorized {
                    None => html! {
                        <section class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                            <p class="text-gray-400 animate-pulse">{i18n.t("panel.check_access")}</p>
                        </section>
                    },
                    Some(true) => html! {
                        <BrowserRouter>
                            <RustmailNavbar avatar_url={avatar_url.clone()} />
                            <section class="pt-24 min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                                <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                                    <Switch<PanelRoute> render={move |route| switch(route, navigator.clone())} />
                                </main>
                            </section>
                        </BrowserRouter>
                    },
                    Some(false) => html! {},
                }
            }
        </>
    }
}

fn switch(route: PanelRoute, navigator: Option<Navigator>) -> Html {
    match route {
        PanelRoute::Home => html! { <Home /> },
        PanelRoute::Configuration => html! { <ConfigurationPage /> },
        PanelRoute::TicketsList => html! { <TicketsList /> },
        PanelRoute::TicketDetails { id } => {
            let nav = navigator.clone();
            html! {
                <TicketDetails
                    id={id}
                    on_back={Callback::from(move |_| {
                        if let Some(ref nav) = nav {
                            nav.push(&PanelRoute::TicketsList);
                        }
                    })}
                />
            }
        }
    }
}
