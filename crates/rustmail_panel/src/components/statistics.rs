use crate::i18n::yew::use_translation;
use gloo_net::http::Request;
use rustmail_types::{
    CategoryStats, DailyActivity, StaffMember, Statistics, StatisticsOverview, TopPerformers,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Period {
    Week,
    Month,
    Quarter,
}

impl Period {
    fn days(&self) -> i64 {
        match self {
            Period::Week => 7,
            Period::Month => 30,
            Period::Quarter => 90,
        }
    }

    fn label(&self, i18n: &crate::i18n::config::I18n) -> String {
        match self {
            Period::Week => i18n.t("panel.statistics.period.week"),
            Period::Month => i18n.t("panel.statistics.period.month"),
            Period::Quarter => i18n.t("panel.statistics.period.quarter"),
        }
    }
}

fn format_duration(seconds: i64, i18n: &crate::i18n::config::I18n) -> String {
    if seconds < 60 {
        format!("{} {}", seconds, i18n.t("panel.statistics.seconds"))
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        format!("{} {}", minutes, i18n.t("panel.statistics.minutes"))
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        if minutes > 0 {
            format!(
                "{}{} {}{}",
                hours,
                i18n.t("panel.statistics.hours_short"),
                minutes,
                i18n.t("panel.statistics.minutes_short")
            )
        } else {
            format!("{} {}", hours, i18n.t("panel.statistics.hours"))
        }
    } else {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        if hours > 0 {
            format!(
                "{}{} {}{}",
                days,
                i18n.t("panel.statistics.days_short"),
                hours,
                i18n.t("panel.statistics.hours_short")
            )
        } else {
            format!("{} {}", days, i18n.t("panel.statistics.days"))
        }
    }
}

#[function_component(StatisticsDashboard)]
pub fn statistics_dashboard() -> Html {
    let (i18n, _set_language) = use_translation();
    let statistics = use_state(|| None::<Statistics>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let period = use_state(|| Period::Month);
    let show_all_staff = use_state(|| false);

    {
        let statistics = statistics.clone();
        let loading = loading.clone();
        let error = error.clone();
        let period = *period;
        let i18n_clone = i18n.clone();

        use_effect_with(period, move |_| {
            loading.set(true);
            spawn_local(async move {
                let url = format!("/api/bot/statistics?days={}", period.days());
                match Request::get(&url).send().await {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            if let Ok(stats) = resp.json::<Statistics>().await {
                                statistics.set(Some(stats));
                                error.set(None);
                            } else {
                                error.set(Some(i18n_clone.t("panel.statistics.error_parse")));
                            }
                        } else {
                            error.set(Some(format!(
                                "{}: {}",
                                i18n_clone.t("panel.statistics.error_load"),
                                resp.status()
                            )));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!(
                            "{}: {:?}",
                            i18n_clone.t("panel.statistics.error_network"),
                            e
                        )));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_period_change = {
        let period = period.clone();
        Callback::from(move |new_period: Period| {
            period.set(new_period);
        })
    };

    let toggle_show_all = {
        let show_all_staff = show_all_staff.clone();
        Callback::from(move |_| {
            show_all_staff.set(!*show_all_staff);
        })
    };

    html! {
        <div class="space-y-6">
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
                <h2 class="text-2xl font-bold text-white">{i18n.t("panel.statistics.title")}</h2>
                <PeriodSelector
                    current={*period}
                    on_change={on_period_change}
                />
            </div>

            {
                if *loading {
                    html! {
                        <div class="text-center text-gray-400 py-12">
                            <p class="animate-pulse">{i18n.t("panel.statistics.loading")}</p>
                        </div>
                    }
                } else if let Some(err) = (*error).clone() {
                    html! {
                        <div class="bg-red-900/20 border border-red-500 text-red-200 p-4 rounded-md">
                            {err}
                        </div>
                    }
                } else if let Some(stats) = (*statistics).clone() {
                    html! {
                        <>
                            <OverviewCards overview={stats.overview.clone()} />
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 items-stretch">
                                <ActivityChart activity={stats.activity.clone()} />
                                <CategoryBreakdown categories={stats.categories.clone()} />
                            </div>
                            <TopPerformersSection performers={stats.top_performers.clone()} />
                            <StaffLeaderboard
                                staff={stats.staff_leaderboard.clone()}
                                show_all={*show_all_staff}
                                on_toggle={toggle_show_all}
                            />
                        </>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PeriodSelectorProps {
    current: Period,
    on_change: Callback<Period>,
}

#[function_component(PeriodSelector)]
fn period_selector(props: &PeriodSelectorProps) -> Html {
    let (i18n, _) = use_translation();
    let periods = [Period::Week, Period::Month, Period::Quarter];

    html! {
        <div class="flex bg-slate-800 rounded-lg p-1">
            {
                periods.iter().map(|p| {
                    let is_active = *p == props.current;
                    let on_change = props.on_change.clone();
                    let period = *p;
                    html! {
                        <button
                            onclick={Callback::from(move |_| on_change.emit(period))}
                            class={classes!(
                                "px-4",
                                "py-2",
                                "text-sm",
                                "rounded-md",
                                "transition",
                                if is_active {
                                    "bg-blue-600 text-white"
                                } else {
                                    "text-gray-400 hover:text-white"
                                }
                            )}
                        >
                            {p.label(&i18n)}
                        </button>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct OverviewCardsProps {
    overview: StatisticsOverview,
}

#[function_component(OverviewCards)]
fn overview_cards(props: &OverviewCardsProps) -> Html {
    let (i18n, _) = use_translation();
    let o = &props.overview;

    html! {
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
            <StatCard
                label={i18n.t("panel.statistics.open_tickets")}
                value={o.open_tickets.to_string()}
                color="blue"
            />
            <StatCard
                label={i18n.t("panel.statistics.closed_today")}
                value={o.closed_today.to_string()}
                color="green"
            />
            <StatCard
                label={i18n.t("panel.statistics.closed_week")}
                value={o.closed_this_week.to_string()}
                color="purple"
            />
            <StatCard
                label={i18n.t("panel.statistics.total_closed")}
                value={o.total_closed.to_string()}
                color="gray"
            />
            <StatCard
                label={i18n.t("panel.statistics.avg_response")}
                value={o.avg_response_time_seconds.map(|s| format_duration(s, &i18n)).unwrap_or_else(|| "-".to_string())}
                color="yellow"
            />
            <StatCard
                label={i18n.t("panel.statistics.avg_resolution")}
                value={o.avg_resolution_time_seconds.map(|s| format_duration(s, &i18n)).unwrap_or_else(|| "-".to_string())}
                color="orange"
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct StatCardProps {
    label: String,
    value: String,
    color: &'static str,
}

#[function_component(StatCard)]
fn stat_card(props: &StatCardProps) -> Html {
    let border_color = match props.color {
        "blue" => "border-blue-500",
        "green" => "border-green-500",
        "purple" => "border-purple-500",
        "yellow" => "border-yellow-500",
        "orange" => "border-orange-500",
        _ => "border-slate-600",
    };

    html! {
        <div class={classes!("bg-slate-800", "rounded-lg", "p-4", "border-l-4", border_color)}>
            <p class="text-gray-400 text-sm mb-1">{&props.label}</p>
            <p class="text-2xl font-bold text-white">{&props.value}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ActivityChartProps {
    activity: Vec<DailyActivity>,
}

#[function_component(ActivityChart)]
fn activity_chart(props: &ActivityChartProps) -> Html {
    let (i18n, _) = use_translation();

    if props.activity.is_empty() {
        return html! {
            <div class="bg-slate-800 rounded-lg p-6 h-full">
                <h3 class="text-lg font-semibold text-white mb-4">{i18n.t("panel.statistics.activity")}</h3>
                <p class="text-gray-400 text-center py-8">{i18n.t("panel.statistics.no_data")}</p>
            </div>
        };
    }

    let max_value = props
        .activity
        .iter()
        .flat_map(|a| [a.created, a.closed])
        .max()
        .unwrap_or(1)
        .max(1);

    let chart_height = 200.0;

    let num_days = props.activity.len();
    let bar_width = if num_days > 60 {
        4
    } else if num_days > 30 {
        6
    } else {
        10
    };
    let chart_min_width = num_days * bar_width * 2 + num_days * 2;

    html! {
        <div class="bg-slate-800 rounded-lg p-6 h-full flex flex-col">
            <h3 class="text-lg font-semibold text-white mb-4">{i18n.t("panel.statistics.activity")}</h3>
            <div class="flex-1 flex flex-col justify-end overflow-x-auto">
                <div
                    class="flex items-end gap-[2px] border-b border-slate-600"
                    style={format!("height: 200px; min-width: {}px;", chart_min_width)}
                >
                    {
                        props.activity.iter().map(|day| {
                            let created_height = (day.created as f64 / max_value as f64 * chart_height) as i32;
                            let closed_height = (day.closed as f64 / max_value as f64 * chart_height) as i32;
                            html! {
                                <div
                                    class="flex-1 flex gap-[1px] items-end cursor-pointer hover:opacity-80 transition-opacity"
                                    style={format!("min-width: {}px;", bar_width * 2)}
                                    title={format!("{}\n{}: {}\n{}: {}", day.date, i18n.t("panel.statistics.created"), day.created, i18n.t("panel.statistics.closed"), day.closed)}
                                >
                                    <div
                                        class="flex-1 bg-blue-500 rounded-t"
                                        style={format!("height: {}px; min-width: {}px;", created_height.max(3), bar_width)}
                                    />
                                    <div
                                        class="flex-1 bg-green-500 rounded-t"
                                        style={format!("height: {}px; min-width: {}px;", closed_height.max(3), bar_width)}
                                    />
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
            <div class="flex justify-center gap-6 mt-4 text-sm">
                <div class="flex items-center gap-2">
                    <div class="w-3 h-3 bg-blue-500 rounded" />
                    <span class="text-gray-400">{i18n.t("panel.statistics.created")}</span>
                </div>
                <div class="flex items-center gap-2">
                    <div class="w-3 h-3 bg-green-500 rounded" />
                    <span class="text-gray-400">{i18n.t("panel.statistics.closed")}</span>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CategoryBreakdownProps {
    categories: Vec<CategoryStats>,
}

#[function_component(CategoryBreakdown)]
fn category_breakdown(props: &CategoryBreakdownProps) -> Html {
    let (i18n, _) = use_translation();

    if props.categories.is_empty() {
        return html! {
            <div class="bg-slate-800 rounded-lg p-6 h-full">
                <h3 class="text-lg font-semibold text-white mb-4">{i18n.t("panel.statistics.categories")}</h3>
                <p class="text-gray-400 text-center py-4">{i18n.t("panel.statistics.no_data")}</p>
            </div>
        };
    }

    let colors = [
        "bg-blue-500",
        "bg-green-500",
        "bg-purple-500",
        "bg-yellow-500",
        "bg-orange-500",
        "bg-pink-500",
        "bg-cyan-500",
        "bg-red-500",
        "bg-indigo-500",
        "bg-teal-500",
    ];

    html! {
        <div class="bg-slate-800 rounded-lg p-6 h-full">
            <h3 class="text-lg font-semibold text-white mb-4">{i18n.t("panel.statistics.categories")}</h3>
            <div class="space-y-3">
                {
                    props.categories.iter().enumerate().map(|(i, cat)| {
                        let color = colors[i % colors.len()];
                        html! {
                            <div class="space-y-1">
                                <div class="flex justify-between text-sm">
                                    <span class="text-gray-300">{&cat.name}</span>
                                    <span class="text-gray-400">{format!("{} ({:.1}%)", cat.count, cat.percentage)}</span>
                                </div>
                                <div class="h-2 bg-slate-700 rounded-full overflow-hidden">
                                    <div
                                        class={classes!("h-full", "rounded-full", color)}
                                        style={format!("width: {}%", cat.percentage)}
                                    />
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TopPerformersSectionProps {
    performers: TopPerformers,
}

#[function_component(TopPerformersSection)]
fn top_performers_section(props: &TopPerformersSectionProps) -> Html {
    let (i18n, _) = use_translation();
    let p = &props.performers;

    html! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            {
                if let Some(ref performer) = p.fastest_responder {
                    html! {
                        <div class="bg-slate-800 rounded-lg p-4 border border-yellow-500/30">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 bg-yellow-500/20 rounded-full flex items-center justify-center text-yellow-500">
                                    {"1"}
                                </div>
                                <div>
                                    <p class="text-xs text-yellow-500 uppercase">{i18n.t("panel.statistics.fastest_responder")}</p>
                                    <p class="text-white font-semibold">{&performer.username}</p>
                                    <p class="text-gray-400 text-sm">{format_duration(performer.value, &i18n)}</p>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(ref performer) = p.most_messages {
                    html! {
                        <div class="bg-slate-800 rounded-lg p-4 border border-blue-500/30">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 bg-blue-500/20 rounded-full flex items-center justify-center text-blue-500">
                                    {"1"}
                                </div>
                                <div>
                                    <p class="text-xs text-blue-500 uppercase">{i18n.t("panel.statistics.most_messages")}</p>
                                    <p class="text-white font-semibold">{&performer.username}</p>
                                    <p class="text-gray-400 text-sm">{format!("{} {}", performer.value, i18n.t("panel.statistics.messages"))}</p>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(ref performer) = p.most_tickets_closed {
                    html! {
                        <div class="bg-slate-800 rounded-lg p-4 border border-green-500/30">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 bg-green-500/20 rounded-full flex items-center justify-center text-green-500">
                                    {"1"}
                                </div>
                                <div>
                                    <p class="text-xs text-green-500 uppercase">{i18n.t("panel.statistics.most_tickets_closed")}</p>
                                    <p class="text-white font-semibold">{&performer.username}</p>
                                    <p class="text-gray-400 text-sm">{format!("{} {}", performer.value, i18n.t("panel.statistics.tickets"))}</p>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct StaffLeaderboardProps {
    staff: Vec<StaffMember>,
    show_all: bool,
    on_toggle: Callback<()>,
}

#[function_component(StaffLeaderboard)]
fn staff_leaderboard(props: &StaffLeaderboardProps) -> Html {
    let (i18n, _) = use_translation();

    if props.staff.is_empty() {
        return html! {
            <div class="bg-slate-800 rounded-lg p-6">
                <h3 class="text-lg font-semibold text-white mb-4">{i18n.t("panel.statistics.staff_leaderboard")}</h3>
                <p class="text-gray-400 text-center py-4">{i18n.t("panel.statistics.no_data")}</p>
            </div>
        };
    }

    let display_count = if props.show_all { props.staff.len() } else { 5 };
    let has_more = props.staff.len() > 5;

    html! {
        <div class="bg-slate-800 rounded-lg p-6">
            <h3 class="text-lg font-semibold text-white mb-4">{i18n.t("panel.statistics.staff_leaderboard")}</h3>
            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr class="text-left text-gray-400 text-sm border-b border-slate-700">
                            <th class="pb-3 pr-4">{"#"}</th>
                            <th class="pb-3 pr-4">{i18n.t("panel.statistics.staff_name")}</th>
                            <th class="pb-3 pr-4 text-right">{i18n.t("panel.statistics.messages")}</th>
                            <th class="pb-3 text-right">{i18n.t("panel.statistics.tickets_closed")}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            props.staff.iter().take(display_count).enumerate().map(|(i, member)| {
                                html! {
                                    <tr class="border-b border-slate-700/50 hover:bg-slate-700/30">
                                        <td class="py-3 pr-4 text-gray-500">{i + 1}</td>
                                        <td class="py-3 pr-4 text-white">{&member.username}</td>
                                        <td class="py-3 pr-4 text-right text-gray-300">{member.messages_count}</td>
                                        <td class="py-3 text-right text-gray-300">{member.tickets_closed}</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
            </div>
            {
                if has_more {
                    html! {
                        <button
                            onclick={props.on_toggle.reform(|_| ())}
                            class="mt-4 w-full py-2 text-sm text-blue-400 hover:text-blue-300 transition"
                        >
                            {if props.show_all { i18n.t("panel.statistics.show_less") } else { i18n.t("panel.statistics.show_all") }}
                        </button>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
