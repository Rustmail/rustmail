use crate::components::wizard::layout::WizardLayout;
use yew::prelude::*;

#[function_component(Setup)]
pub fn setup() -> Html {
    let step_names = vec![
        "Bot Token",
        "Guilds",
        "Thread Config",
        "Web Panel",
        "Language",
        "Review",
    ];

    html! {
        <WizardLayout
            current_step={0}
            total_steps={step_names.len()}
            step_names={step_names}
            title={"Step 1: Discord Bot Token"}
            description={"Let's start by linking your Discord bot. You can find your token in the Discord Developer Portal."}
        >
            <div class="flex flex-col gap-4">
                <div class="animate-pulse flex space-x-4">
                    <div class="flex-1 space-y-6 py-1">
                        <div class="h-2 bg-slate-700 rounded"></div>
                        <div class="space-y-3">
                            <div class="grid grid-cols-3 gap-4">
                                <div class="h-2 bg-slate-700 rounded col-span-2"></div>
                                <div class="h-2 bg-slate-700 rounded col-span-1"></div>
                            </div>
                            <div class="h-2 bg-slate-700 rounded"></div>
                        </div>
                    </div>
                </div>
                <p class="text-sm text-gray-500 text-center mt-8">
                    { "The first step will be implemented in the next issue." }
                </p>
            </div>
        </WizardLayout>
    }
}
