mod components;
mod pages;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{switch, Route};

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
