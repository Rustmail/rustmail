use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub current_step: usize,
    pub total_steps: usize,
    pub step_names: Vec<String>,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let progress_percent = if props.total_steps > 1 {
        (props.current_step as f64 / (props.total_steps - 1) as f64) * 100.0
    } else {
        100.0
    };

    html! {
        <div class="w-full mb-8">
            <div class="flex justify-between items-center mb-2 px-1">
                {
                    for props.step_names.iter().enumerate().map(|(idx, name)| {
                        let is_active = idx == props.current_step;
                        let is_past = idx < props.current_step;

                        let text_color = if is_active {
                            "text-indigo-400 font-semibold"
                        } else if is_past {
                            "text-gray-300"
                        } else {
                            "text-gray-600"
                        };

                        html! {
                            <div class={classes!("text-xs", "sm:text-sm", "transition-colors", "duration-300", text_color)}>
                                { name }
                            </div>
                        }
                    })
                }
            </div>

            <div class="h-2 w-full bg-slate-800 rounded-full overflow-hidden">
                <div
                    class="h-full bg-indigo-500 transition-all duration-500 ease-out"
                    style={format!("width: {}%", progress_percent)}
                ></div>
            </div>
        </div>
    }
}
