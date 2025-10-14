use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
            <img src="logo.png" alt="Rustmail logo" class="w-40 h-40 mb-6" />
            <h1 class="text-3xl font-bold mb-2">{"Rustmail Panel"}</h1>
            <p class="max-w-xl text-center text-gray-400 mb-8">
                {"Bienvenue sur le pannel de Rustmail"}
            </p>
            <div class="flex gap-4">
                <a href="https://github.com" class="px-4 py-2 bg-indigo-600 rounded-lg hover:bg-indigo-700 transition">{"GitHub"}</a>
                <a href="/docs" class="px-4 py-2 border border-gray-500 rounded-lg hover:bg-gray-800 transition">{"Documentation"}</a>
            </div>
        </section>
    }
}
