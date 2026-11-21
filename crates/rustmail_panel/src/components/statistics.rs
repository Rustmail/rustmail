use crate::components::ui::{
    Avatar, Button, ButtonSize, ButtonVariant, CardHeader, EmptyState, Icon, IconName, Loading,
    Notice, PageHeader, Segmented,
};
use crate::i18n::config::I18n;
use crate::i18n::yew::use_translation;
use crate::utils::format::group_digits;
use gloo_net::http::Request;
use rustmail_types::{
    CategoryStats, DailyActivity, StaffMember, Statistics, StatisticsOverview, TopPerformer,
    TopPerformers,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const PLOT_WIDTH: f64 = 1000.0;
const PLOT_HEIGHT: f64 = 240.0;
const COLLAPSED_STAFF_ROWS: usize = 5;
const MONTH_KEYS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

#[derive(Clone, Copy, PartialEq)]
pub enum Period {
    Week,
    Month,
    Quarter,
}

impl Period {
    const ALL: [Period; 3] = [Period::Week, Period::Month, Period::Quarter];

    fn days(&self) -> i64 {
        match self {
            Period::Week => 7,
            Period::Month => 30,
            Period::Quarter => 90,
        }
    }

    fn label(&self, i18n: &I18n) -> String {
        match self {
            Period::Week => i18n.t("panel.statistics.period.week"),
            Period::Month => i18n.t("panel.statistics.period.month"),
            Period::Quarter => i18n.t("panel.statistics.period.quarter"),
        }
    }

    fn from_days(value: &str) -> Option<Period> {
        Period::ALL
            .into_iter()
            .find(|p| p.days().to_string() == value)
    }
}

fn format_duration(seconds: i64, i18n: &I18n) -> String {
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

fn optional_duration(seconds: Option<i64>, i18n: &I18n) -> String {
    seconds
        .map(|s| format_duration(s, i18n))
        .unwrap_or_else(|| "-".to_string())
}

struct CalendarDate {
    year: i32,
    month: usize,
    day: u32,
}

fn parse_date(value: &str) -> Option<CalendarDate> {
    let mut parts = value.split('-');
    let year = parts.next()?.parse().ok()?;
    let month: usize = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;
    (1..=12)
        .contains(&month)
        .then_some(CalendarDate { year, month, day })
}

fn month_name(month: usize, i18n: &I18n) -> String {
    i18n.t(&format!(
        "panel.statistics.months.{}",
        MONTH_KEYS[month - 1]
    ))
}

fn short_date(value: &str, i18n: &I18n) -> String {
    parse_date(value)
        .map(|d| format!("{} {}", d.day, month_name(d.month, i18n)))
        .unwrap_or_else(|| value.to_string())
}

fn date_range(activity: &[DailyActivity], i18n: &I18n) -> Option<String> {
    let first = parse_date(&activity.first()?.date)?;
    let last = parse_date(&activity.last()?.date)?;
    Some(format!(
        "{} {} – {} {} {}",
        first.day,
        month_name(first.month, i18n),
        last.day,
        month_name(last.month, i18n),
        last.year
    ))
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
        Callback::from(move |value: AttrValue| {
            if let Some(next) = Period::from_days(&value) {
                period.set(next);
            }
        })
    };

    let toggle_show_all = {
        let show_all_staff = show_all_staff.clone();
        Callback::from(move |_| show_all_staff.set(!*show_all_staff))
    };

    let period_options: Vec<(AttrValue, AttrValue)> = Period::ALL
        .iter()
        .map(|p| {
            (
                AttrValue::from(p.days().to_string()),
                AttrValue::from(p.label(&i18n)),
            )
        })
        .collect();

    let period_caption = i18n
        .t("panel.statistics.last_days")
        .replace("{days}", &period.days().to_string());
    let subtitle = match (*statistics)
        .as_ref()
        .and_then(|s| date_range(&s.activity, &i18n))
    {
        Some(range) => format!("{} · {}", period_caption, range),
        None => period_caption,
    };

    let body = if *loading {
        html! { <Loading label={i18n.t("panel.statistics.loading")} class="min-h-[50vh]" /> }
    } else if let Some(err) = (*error).clone() {
        html! { <Notice>{err}</Notice> }
    } else if let Some(stats) = (*statistics).clone() {
        html! {
            <>
                <TicketCounts overview={stats.overview.clone()} />
                <div class="flex flex-col gap-6 xl:flex-row xl:items-stretch">
                    <div class="flex min-w-0 flex-1 flex-col">
                        <ActivityChart activity={stats.activity.clone()} />
                    </div>
                    <div class="flex flex-col gap-6 xl:w-[360px] xl:shrink-0">
                        <ResponseTimes overview={stats.overview.clone()} />
                        <CategoryBreakdown categories={stats.categories.clone()} />
                    </div>
                </div>
                <StaffSection
                    performers={stats.top_performers.clone()}
                    staff={stats.staff_leaderboard.clone()}
                    show_all={*show_all_staff}
                    on_toggle={toggle_show_all}
                />
            </>
        }
    } else {
        html! {}
    };

    html! {
        <>
            <PageHeader title={i18n.t("navbar.home")} description={subtitle}>
                <Segmented
                    options={period_options}
                    value={AttrValue::from(period.days().to_string())}
                    onchange={on_period_change}
                />
            </PageHeader>
            {body}
        </>
    }
}

#[derive(Properties, PartialEq)]
struct TicketCountsProps {
    overview: StatisticsOverview,
}

#[function_component(TicketCounts)]
fn ticket_counts(props: &TicketCountsProps) -> Html {
    let (i18n, _) = use_translation();
    let o = &props.overview;
    let month_caption = i18n
        .t("panel.statistics.closed_month_caption")
        .replace("{count}", &group_digits(o.closed_this_month));

    let cells = [
        (
            i18n.t("panel.statistics.open_tickets"),
            o.open_tickets,
            None,
        ),
        (
            i18n.t("panel.statistics.closed_today"),
            o.closed_today,
            None,
        ),
        (
            i18n.t("panel.statistics.closed_week"),
            o.closed_this_week,
            None,
        ),
        (
            i18n.t("panel.statistics.total_closed"),
            o.total_closed,
            Some(month_caption),
        ),
    ];

    html! {
        <section class="card grid grid-cols-2 overflow-hidden lg:grid-cols-4">
            { for cells.into_iter().enumerate().map(|(index, (label, value, caption))| {
                let divider = match index {
                    0 => classes!(),
                    1 => classes!("border-l", "border-line"),
                    2 => classes!("border-t", "border-line", "lg:border-l", "lg:border-t-0"),
                    _ => classes!("border-l", "border-t", "border-line", "lg:border-t-0"),
                };
                html! {
                    <div class={classes!("flex", "flex-col", "gap-1", "p-5", divider)}>
                        <span class="text-[13px] text-fg-secondary">{label}</span>
                        <span class="tnum text-figure font-semibold text-fg">{group_digits(value)}</span>
                        if let Some(caption) = caption {
                            <span class="text-xs text-fg-muted">{caption}</span>
                        }
                    </div>
                }
            }) }
        </section>
    }
}

#[derive(Clone, Copy)]
struct PlotPoint {
    x: f64,
    y: f64,
}

fn nice_axis_max(max_value: i64) -> (i64, i64) {
    let raw_step = (max_value.max(1) as f64 / 4.0).ceil() as i64;
    let mut magnitude = 1;
    while magnitude * 10 <= raw_step {
        magnitude *= 10;
    }
    let step = [1, 2, 5, 10]
        .into_iter()
        .map(|m| m * magnitude)
        .find(|s| *s >= raw_step)
        .unwrap_or(10 * magnitude);
    (step * 4, step)
}

fn plot_points(values: &[i64], axis_max: i64) -> Vec<PlotPoint> {
    let last_index = values.len().saturating_sub(1).max(1) as f64;
    values
        .iter()
        .enumerate()
        .map(|(index, value)| PlotPoint {
            x: index as f64 / last_index * PLOT_WIDTH,
            y: PLOT_HEIGHT - *value as f64 / axis_max as f64 * PLOT_HEIGHT,
        })
        .collect()
}

fn smooth_path(points: &[PlotPoint]) -> String {
    let Some(first) = points.first() else {
        return String::new();
    };
    let mut path = format!("M{:.2},{:.2}", first.x, first.y);
    if points.len() == 1 {
        path.push_str(&format!(" L{:.2},{:.2}", PLOT_WIDTH, first.y));
        return path;
    }
    let clamp_y = |y: f64| y.clamp(0.0, PLOT_HEIGHT);
    for index in 0..points.len() - 1 {
        let previous = points[index.saturating_sub(1)];
        let current = points[index];
        let next = points[index + 1];
        let after = points[(index + 2).min(points.len() - 1)];
        let control_start = PlotPoint {
            x: current.x + (next.x - previous.x) / 6.0,
            y: clamp_y(current.y + (next.y - previous.y) / 6.0),
        };
        let control_end = PlotPoint {
            x: next.x - (after.x - current.x) / 6.0,
            y: clamp_y(next.y - (after.y - current.y) / 6.0),
        };
        path.push_str(&format!(
            " C{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}",
            control_start.x, control_start.y, control_end.x, control_end.y, next.x, next.y
        ));
    }
    path
}

fn axis_label_indices(len: usize) -> Vec<usize> {
    if len <= 1 {
        return vec![0];
    }
    let mut indices: Vec<usize> = (0..5)
        .map(|k| ((k as f64) * (len - 1) as f64 / 4.0).round() as usize)
        .collect();
    indices.dedup();
    indices
}

fn percent(value: f64, total: f64) -> f64 {
    value / total * 100.0
}

#[derive(Properties, PartialEq)]
struct ActivityChartProps {
    activity: Vec<DailyActivity>,
}

#[function_component(ActivityChart)]
fn activity_chart(props: &ActivityChartProps) -> Html {
    let (i18n, _) = use_translation();
    let hovered = use_state(|| None::<usize>);
    let activity = &props.activity;

    let total_created: i64 = activity.iter().map(|a| a.created).sum();
    let total_closed: i64 = activity.iter().map(|a| a.closed).sum();

    let legend = html! {
        <div class="flex items-center gap-4 text-xs text-fg-secondary">
            <span class="flex items-center gap-1.5">
                <span class="h-0.5 w-3 rounded-full bg-accent" />
                {format!("{} · {}", i18n.t("panel.statistics.created"), group_digits(total_created))}
            </span>
            <span class="flex items-center gap-1.5">
                <span class="h-0.5 w-3 rounded-full bg-fg-muted" />
                {format!("{} · {}", i18n.t("panel.statistics.closed"), group_digits(total_closed))}
            </span>
        </div>
    };

    let header = html! {
        <CardHeader
            title={i18n.t("panel.statistics.activity")}
            description={i18n.t("panel.statistics.activity_description")}
        >
            if !activity.is_empty() {
                <div class="pt-1">{legend}</div>
            }
        </CardHeader>
    };

    if activity.is_empty() || (total_created == 0 && total_closed == 0) {
        return html! {
            <section class="card flex h-full flex-col gap-4 p-6">
                {header}
                <div class="flex flex-1 items-center justify-center">
                    <EmptyState icon={IconName::ChartLine} title={i18n.t("panel.statistics.no_data")} />
                </div>
            </section>
        };
    }

    let max_value = activity
        .iter()
        .flat_map(|a| [a.created, a.closed])
        .max()
        .unwrap_or(1);
    let (axis_max, axis_step) = nice_axis_max(max_value);

    let created_points = plot_points(
        &activity.iter().map(|a| a.created).collect::<Vec<_>>(),
        axis_max,
    );
    let closed_points = plot_points(
        &activity.iter().map(|a| a.closed).collect::<Vec<_>>(),
        axis_max,
    );
    let created_path = smooth_path(&created_points);
    let closed_path = smooth_path(&closed_points);

    let grid_values: Vec<i64> = (0..=4).map(|k| k * axis_step).collect();
    let last_index = activity.len().saturating_sub(1).max(1) as f64;
    let column_percent = |index: usize| percent(index as f64, last_index);

    let last_created = created_points.last().copied();
    let label_indices = axis_label_indices(activity.len());
    let last_label = *label_indices.last().unwrap_or(&0);

    let on_leave = {
        let hovered = hovered.clone();
        Callback::from(move |_: MouseEvent| hovered.set(None))
    };

    let tooltip = hovered.and_then(|index| activity.get(index).map(|day| (index, day)));

    html! {
        <section class="card flex h-full flex-col gap-6 p-6">
            {header}
            <div class="flex min-h-[260px] flex-1 gap-3">
                <div class="relative w-6 shrink-0 font-mono text-xs text-fg-muted">
                    { for grid_values.iter().map(|value| html! {
                        <span
                            class="absolute right-0 -translate-y-1/2 tnum"
                            style={format!("top:{:.3}%", 100.0 - percent(*value as f64, axis_max as f64))}
                        >
                            {value}
                        </span>
                    }) }
                </div>
                <div class="flex min-w-0 flex-1 flex-col gap-2">
                    <div class="relative flex-1" onmouseleave={on_leave}>
                        <svg
                            class="absolute inset-0 h-full w-full overflow-visible"
                            viewBox={format!("0 0 {} {}", PLOT_WIDTH, PLOT_HEIGHT)}
                            preserveAspectRatio="none"
                            aria-hidden="true"
                        >
                            { for grid_values.iter().map(|value| {
                                let y = PLOT_HEIGHT - *value as f64 / axis_max as f64 * PLOT_HEIGHT;
                                html! {
                                    <line
                                        x1="0"
                                        x2={PLOT_WIDTH.to_string()}
                                        y1={format!("{:.2}", y)}
                                        y2={format!("{:.2}", y)}
                                        class="stroke-line"
                                        stroke-width="1"
                                        vector-effect="non-scaling-stroke"
                                    />
                                }
                            }) }
                            if let Some((index, _)) = tooltip {
                                <line
                                    x1={format!("{:.2}", column_percent(index) / 100.0 * PLOT_WIDTH)}
                                    x2={format!("{:.2}", column_percent(index) / 100.0 * PLOT_WIDTH)}
                                    y1="0"
                                    y2={PLOT_HEIGHT.to_string()}
                                    class="stroke-line-strong"
                                    stroke-width="1"
                                    vector-effect="non-scaling-stroke"
                                />
                            }
                            <path
                                d={closed_path}
                                fill="none"
                                class="stroke-fg-muted"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                vector-effect="non-scaling-stroke"
                            />
                            <path
                                d={created_path}
                                fill="none"
                                class="stroke-accent"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                vector-effect="non-scaling-stroke"
                            />
                        </svg>
                        if let Some(point) = last_created {
                            <span
                                class="pointer-events-none absolute size-2 -translate-x-1/2 -translate-y-1/2 rounded-full bg-accent"
                                style={format!(
                                    "left:{:.3}%;top:{:.3}%",
                                    percent(point.x, PLOT_WIDTH),
                                    percent(point.y, PLOT_HEIGHT)
                                )}
                            />
                        }
                        <div class="absolute inset-0 flex">
                            { for activity.iter().enumerate().map(|(index, _)| {
                                let on_enter = {
                                    let hovered = hovered.clone();
                                    Callback::from(move |_: MouseEvent| hovered.set(Some(index)))
                                };
                                html! { <div class="h-full flex-1" onmouseenter={on_enter} /> }
                            }) }
                        </div>
                        if let Some((index, day)) = tooltip {
                            <ChartTooltip
                                left_percent={column_percent(index)}
                                date={short_date(&day.date, &i18n)}
                                created={day.created}
                                closed={day.closed}
                            />
                        }
                    </div>
                    <div class="relative h-4 font-mono text-xs text-fg-muted">
                        { for label_indices.iter().map(|index| {
                            let alignment = if *index == 0 {
                                "translate-x-0"
                            } else if *index == last_label {
                                "-translate-x-full"
                            } else {
                                "-translate-x-1/2"
                            };
                            html! {
                                <span
                                    class={classes!("absolute", "top-0", "whitespace-nowrap", alignment)}
                                    style={format!("left:{:.3}%", column_percent(*index))}
                                >
                                    {short_date(&activity[*index].date, &i18n)}
                                </span>
                            }
                        }) }
                    </div>
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct ChartTooltipProps {
    left_percent: f64,
    date: String,
    created: i64,
    closed: i64,
}

#[function_component(ChartTooltip)]
fn chart_tooltip(props: &ChartTooltipProps) -> Html {
    let (i18n, _) = use_translation();
    let alignment = if props.left_percent > 70.0 {
        "-translate-x-[calc(100%+8px)]"
    } else {
        "translate-x-2"
    };
    html! {
        <div
            class={classes!("pointer-events-none", "absolute", "top-2", "z-10", "flex", "min-w-[132px]", "flex-col", "gap-1", "rounded-lg", "border", "border-line-strong", "bg-surface-raised", "px-3", "py-2", "text-xs", alignment)}
            style={format!("left:{:.3}%", props.left_percent)}
        >
            <span class="font-mono text-fg-secondary">{&props.date}</span>
            <span class="flex items-center justify-between gap-4 text-fg-secondary">
                <span class="flex items-center gap-1.5">
                    <span class="h-0.5 w-3 rounded-full bg-accent" />
                    {i18n.t("panel.statistics.created")}
                </span>
                <span class="font-mono tnum text-fg">{props.created}</span>
            </span>
            <span class="flex items-center justify-between gap-4 text-fg-secondary">
                <span class="flex items-center gap-1.5">
                    <span class="h-0.5 w-3 rounded-full bg-fg-muted" />
                    {i18n.t("panel.statistics.closed")}
                </span>
                <span class="font-mono tnum text-fg">{props.closed}</span>
            </span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ResponseTimesProps {
    overview: StatisticsOverview,
}

#[function_component(ResponseTimes)]
fn response_times(props: &ResponseTimesProps) -> Html {
    let (i18n, _) = use_translation();
    let metrics = [
        (
            i18n.t("panel.statistics.avg_response"),
            optional_duration(props.overview.avg_response_time_seconds, &i18n),
        ),
        (
            i18n.t("panel.statistics.avg_resolution"),
            optional_duration(props.overview.avg_resolution_time_seconds, &i18n),
        ),
    ];
    html! {
        <section class="card flex flex-col gap-5 p-6">
            <CardHeader title={i18n.t("panel.statistics.response_times")} />
            <div class="grid grid-cols-2 gap-4">
                { for metrics.into_iter().map(|(label, value)| html! {
                    <div class="flex flex-col gap-1">
                        <span class="text-[13px] text-fg-secondary">{label}</span>
                        <span class="tnum text-section font-semibold text-fg">{value}</span>
                    </div>
                }) }
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct CategoryBreakdownProps {
    categories: Vec<CategoryStats>,
}

#[function_component(CategoryBreakdown)]
fn category_breakdown(props: &CategoryBreakdownProps) -> Html {
    let (i18n, _) = use_translation();
    let mut ranked = props.categories.clone();
    ranked.sort_by_key(|c| std::cmp::Reverse(c.count));

    html! {
        <section class="card flex flex-1 flex-col gap-4 p-6">
            <CardHeader title={i18n.t("panel.statistics.categories")} />
            if ranked.is_empty() {
                <p class="text-[13px] text-fg-muted">{i18n.t("panel.statistics.no_data")}</p>
            } else {
                <div class="flex flex-col gap-4">
                    { for ranked.iter().map(|category| html! {
                        <div class="flex flex-col gap-2">
                            <div class="flex items-baseline gap-3">
                                <span class="min-w-0 flex-1 truncate text-sm text-fg">{&category.name}</span>
                                <span class="font-mono tnum text-xs text-fg-secondary">{group_digits(category.count)}</span>
                                <span class="w-12 text-right font-mono tnum text-xs text-fg-muted">{format!("{:.1}%", category.percentage)}</span>
                            </div>
                            <div class="h-1 overflow-hidden rounded-full bg-surface-raised">
                                <div
                                    class="h-full rounded-full bg-fg-muted"
                                    style={format!("width:{:.2}%", category.percentage.clamp(0.0, 100.0))}
                                />
                            </div>
                        </div>
                    }) }
                </div>
            }
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct StaffSectionProps {
    performers: TopPerformers,
    staff: Vec<StaffMember>,
    show_all: bool,
    on_toggle: Callback<()>,
}

#[function_component(StaffSection)]
fn staff_section(props: &StaffSectionProps) -> Html {
    let (i18n, _) = use_translation();
    let p = &props.performers;

    let leaders: Vec<(String, TopPerformer, String)> = [
        p.fastest_responder.clone().map(|performer| {
            let value = i18n
                .t("panel.statistics.avg_value")
                .replace("{value}", &format_duration(performer.value, &i18n));
            (
                i18n.t("panel.statistics.fastest_responder"),
                performer,
                value,
            )
        }),
        p.most_messages.clone().map(|performer| {
            let value = format!(
                "{} {}",
                group_digits(performer.value),
                i18n.t("panel.statistics.messages")
            );
            (i18n.t("panel.statistics.most_messages"), performer, value)
        }),
        p.most_tickets_closed.clone().map(|performer| {
            let value = format!(
                "{} {}",
                group_digits(performer.value),
                i18n.t("panel.statistics.tickets")
            );
            (
                i18n.t("panel.statistics.most_tickets_closed"),
                performer,
                value,
            )
        }),
    ]
    .into_iter()
    .flatten()
    .collect();

    let visible = if props.show_all {
        props.staff.len()
    } else {
        COLLAPSED_STAFF_ROWS
    };
    let has_more = props.staff.len() > COLLAPSED_STAFF_ROWS;
    let toggle_label = if props.show_all {
        i18n.t("panel.statistics.show_less")
    } else {
        i18n.t("panel.statistics.show_all")
            .replace("{count}", &props.staff.len().to_string())
    };
    let chevron_class = if props.show_all { "rotate-180" } else { "" };

    html! {
        <section class="card overflow-hidden p-0">
            <div class="p-6">
                <CardHeader
                    title={i18n.t("panel.statistics.staff_leaderboard")}
                    description={i18n.t("panel.statistics.staff_description")}
                />
            </div>
            if props.staff.is_empty() && leaders.is_empty() {
                <div class="border-t border-line">
                    <EmptyState icon={IconName::Users} title={i18n.t("panel.statistics.no_data")} />
                </div>
            } else {
                if !leaders.is_empty() {
                    <div class="grid grid-cols-1 border-t border-line md:grid-cols-3">
                        { for leaders.iter().enumerate().map(|(index, (label, performer, value))| {
                            let divider = if index == 0 {
                                classes!()
                            } else {
                                classes!("border-t", "border-line", "md:border-l", "md:border-t-0")
                            };
                            html! {
                                <div key={label.clone()} class={classes!("flex", "items-center", "gap-3", "px-5", "py-4", divider)}>
                                    <Avatar name={performer.username.clone()} size={32} />
                                    <div class="flex min-w-0 flex-col gap-0.5">
                                        <span class="text-xs text-fg-muted">{label}</span>
                                        <span class="flex min-w-0 items-baseline gap-2">
                                            <span class="truncate text-sm font-medium text-fg">{&performer.username}</span>
                                            <span class="shrink-0 font-mono tnum text-xs text-fg-secondary">{value}</span>
                                        </span>
                                    </div>
                                </div>
                            }
                        }) }
                    </div>
                }
                if !props.staff.is_empty() {
                    <div class="overflow-x-auto border-t border-line">
                        <table class="w-full min-w-[560px]">
                            <thead>
                                <tr class="h-9 text-left text-xs text-fg-muted">
                                    <th class="w-12 pl-5 font-normal">{"#"}</th>
                                    <th class="font-normal">{i18n.t("panel.statistics.staff_name")}</th>
                                    <th class="pr-5 text-right font-normal">{i18n.t("panel.statistics.messages_column")}</th>
                                    <th class="pr-5 text-right font-normal">{i18n.t("panel.statistics.tickets_closed")}</th>
                                    <th class="pr-5 text-right font-normal">{i18n.t("panel.statistics.avg_first_reply")}</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for props.staff.iter().take(visible).enumerate().map(|(index, member)| html! {
                                    <tr key={member.user_id.clone()} class="h-[52px] border-t border-line">
                                        <td class="pl-5 font-mono tnum text-[13px] text-fg-muted">{index + 1}</td>
                                        <td>
                                            <span class="flex items-center gap-2.5">
                                                <Avatar name={member.username.clone()} size={24} />
                                                <span class="truncate text-sm text-fg">{&member.username}</span>
                                            </span>
                                        </td>
                                        <td class="pr-5 text-right font-mono tnum text-[13px] text-fg-secondary">{group_digits(member.messages_count)}</td>
                                        <td class="pr-5 text-right font-mono tnum text-[13px] text-fg-secondary">{group_digits(member.tickets_closed)}</td>
                                        <td class="pr-5 text-right font-mono tnum text-[13px] text-fg-secondary">{optional_duration(member.avg_response_time_seconds, &i18n)}</td>
                                    </tr>
                                }) }
                            </tbody>
                        </table>
                    </div>
                }
                if has_more {
                    <div class="border-t border-line px-3 py-2">
                        <Button
                            variant={ButtonVariant::Ghost}
                            size={ButtonSize::Sm}
                            onclick={props.on_toggle.reform(|_: MouseEvent| ())}
                        >
                            <Icon name={IconName::ChevronDown} class={chevron_class} />
                            {toggle_label}
                        </Button>
                    </div>
                }
            }
        </section>
    }
}
