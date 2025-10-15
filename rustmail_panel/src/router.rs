use crate::pages::docs::Docs;
use crate::pages::home::Home;
use crate::pages::panel::Panel;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/docs")]
    Documentation,
    #[at("/panel")]
    Panel,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Documentation => html! { <Docs /> },
        Route::Panel => html! { <Panel /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
