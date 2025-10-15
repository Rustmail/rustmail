use yew::{function_component, html, Html};

#[function_component(Panel)]
pub fn panel() -> Html {
    html! {
        <section class="relative flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
            <img src="logo.png" alt="Rustmail logo" class="w-40 h-40 mb-6" />
            <h1 class="text-3xl font-bold mb-2">{"Rustmail Panel"}</h1>
            <p class="max-w-xl text-center text-gray-400 mb-8">
                {"Bienvenue sur le panel de Rustmail"}
            </p>
        </section>
    }
}
