use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::docs::Docs;
use crate::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/docs")]
    Documentation,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Documentation => html! { <Docs /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}