use crate::components::language_switcher::LanguageSwitcher;
use crate::components::wizard::progress_bar::ProgressBar;
use crate::i18n::yew::use_translation;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub current_step: usize,
    pub total_steps: usize,
    pub step_names: Vec<String>,
    pub title: String,
    pub description: String,
    pub children: Children,
}

#[function_component(WizardLayout)]
pub fn wizard_layout(props: &LayoutProps) -> Html {
    let (i18n, _) = use_translation();
    html! {
        <div class="min-h-screen bg-slate-950 flex flex-col items-center justify-center p-4 sm:p-8 text-white font-sans relative">

            // Language Switcher in top right corner
            <div class="absolute top-4 right-4 sm:top-8 sm:right-8 z-50">
                <LanguageSwitcher placement="nav" />
            </div>

            <div class="w-full max-w-4xl flex flex-col gap-8">

                // Header with logo and title
                <div class="flex flex-col items-center text-center gap-4">
                    <div class="w-16 h-16 bg-indigo-600 rounded-2xl flex items-center justify-center shadow-lg shadow-indigo-500/20">
                        <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                        </svg>
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-gray-400">
                            { i18n.t("wizard.title") }
                        </h1>
                        <p class="text-gray-400 mt-2">{ i18n.t("wizard.progress") }</p>
                    </div>
                </div>

                // Progress Bar
                <ProgressBar
                    current_step={props.current_step}
                    total_steps={props.total_steps}
                    step_names={props.step_names.clone()}
                />

                // Main Content Card
                <div class="bg-slate-900 border border-slate-800 rounded-2xl shadow-xl overflow-hidden flex flex-col">
                    <div class="px-6 py-5 sm:px-8 sm:py-6 border-b border-slate-800 bg-slate-900/50">
                        <h2 class="text-xl font-semibold text-white">{ &props.title }</h2>
                        <p class="text-sm text-gray-400 mt-1">{ &props.description }</p>
                    </div>

                    <div class="p-6 sm:p-8 flex-1">
                        { for props.children.iter() }
                    </div>
                </div>

            </div>
        </div>
    }
}
