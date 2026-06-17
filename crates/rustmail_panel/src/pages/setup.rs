use yew::prelude::*;

#[function_component(Setup)]
pub fn setup() -> Html {
    html! {
        <div class="min-h-screen bg-gradient-to-b from-slate-900 to-black text-white p-8">
            <h1 class="text-3xl font-bold mb-4">{ "Rustmail Setup Wizard" }</h1>
            <p class="text-gray-400">{ "Welcome to the Rustmail setup wizard. This page is currently under construction." }</p>
        </div>
    }
}
