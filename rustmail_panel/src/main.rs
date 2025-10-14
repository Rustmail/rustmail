mod components;
mod pages;
mod router;

use crate::components::home::Home;
use yew::prelude::*;

#[component]
fn App() -> Html {
    html! {
        <Home />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
