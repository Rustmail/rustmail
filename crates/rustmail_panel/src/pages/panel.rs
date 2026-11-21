use crate::components::api_keys::ApiKeysPage;
use crate::components::bans::BansPage;
use crate::components::categories::CategoriesPage;
use crate::components::configuration::ConfigurationPage;
use crate::components::home::Home;
use crate::components::sidebar::AppShell;
use crate::components::ticket::{TicketDetails, TicketsList};
use crate::components::ui::Loading;
use crate::components::update_banner::UpdateBanner;
use crate::i18n::yew::use_translation;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use gloo_utils::window;
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
    #[at("/panel/apikeys")]
    ApiKeys,
    #[at("/panel/categories")]
    Categories,
    #[at("/panel/bans")]
    Bans,
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

    let authorized = authorized.clone();
    use_effect_with(*authorized, move |auth| {
        if matches!(auth, Some(false)) {
            let win = window();
            let _ = win.location().set_href("/error?message=Access+Denied");
        }
        || ()
    });

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

    let permissions = use_state(Vec::<PanelPermission>::new);
    {
        let permissions = permissions.clone();
        use_effect_with((), move |()| {
            let permissions = permissions.clone();

            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/user/permissions").send().await
                    && let Ok(perms) = resp.json::<Vec<PanelPermission>>().await
                {
                    permissions.set(perms);
                }
            });
            || ()
        });
    }

    let avatar_url = (*avatar).clone().map(AttrValue::from);
    let user_permissions = (*permissions).clone();

    match *authorized {
        None => html! {
            <div class="flex min-h-screen items-center justify-center bg-surface-page">
                <Loading label={i18n.t("panel.check_access")} />
            </div>
        },
        Some(true) => html! {
            <BrowserRouter>
                <AppShell permissions={user_permissions} avatar_url={avatar_url}>
                    <UpdateBanner />
                    <Switch<PanelRoute> render={move |route| switch(route, navigator.clone())} />
                </AppShell>
            </BrowserRouter>
        },
        Some(false) => html! {},
    }
}

fn switch(route: PanelRoute, navigator: Option<Navigator>) -> Html {
    match route {
        PanelRoute::Home => html! { <Home /> },
        PanelRoute::Configuration => html! { <ConfigurationPage /> },
        PanelRoute::ApiKeys => html! { <ApiKeysPage /> },
        PanelRoute::Categories => html! { <CategoriesPage /> },
        PanelRoute::Bans => html! { <BansPage /> },
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
