use crate::components::statistics::StatisticsDashboard;
use crate::i18n::yew::use_translation;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <div class="space-y-8">
            <div class="flex flex-col items-center justify-center text-center">
                <img src="logo.png" alt="Rustmail logo" class="w-24 h-24 mb-4" />
                <h1 class="text-3xl font-bold mb-2">{i18n.t("panel.title")}</h1>
                <p class="max-w-xl text-gray-400">
                    {i18n.t("panel.welcome")}
                </p>
            </div>
            <StatisticsDashboard />
        </div>
    }
}
