use crate::components::statistics::StatisticsDashboard;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! { <StatisticsDashboard /> }
}
