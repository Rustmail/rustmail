use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section class="relative flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
            <a href="/docs"
                class="absolute top-6 right-6 px-4 py-2 border border-gray-500 rounded-lg hover:bg-gray-800 transition">
                {"Documentation"}
            </a>

            <img src="logo.png" alt="Rustmail logo" class="w-40 h-40 mb-6" />
            <h1 class="text-3xl font-bold mb-2">{"Rustmail Panel"}</h1>
            <p class="max-w-xl text-center text-gray-400 mb-8">
                {"Bienvenue. Pour continuer, veuillez vous connecter avec votre compte Discord."}
            </p>
            <div class="flex gap-4">
                <a
                    href="/api/auth/login"
                    class="px-4 py-2 rounded-lg transition"
                    style="background-color: #4f6fd7;"
                >
                    {"Se connecter"}
                </a>
            </div>
        </section>
    }
}
