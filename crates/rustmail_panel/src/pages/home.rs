use crate::components::language_switcher::LanguageSwitcher;
use crate::components::ui::{Avatar, Badge, Icon, IconName, TicketStub, Tone};
use crate::i18n::yew::use_translation;
use crate::pages::error::brand;
use yew::prelude::*;

const PREVIEW_MEMBER: &str = "driftwood";
const PREVIEW_STAFF: &str = "Lou";

struct PreviewMessage {
    author: &'static str,
    staff: bool,
    time: &'static str,
    text_key: &'static str,
}

const PREVIEW_MESSAGES: [PreviewMessage; 3] = [
    PreviewMessage {
        author: PREVIEW_MEMBER,
        staff: false,
        time: "11:47",
        text_key: "home.preview.message_1",
    },
    PreviewMessage {
        author: PREVIEW_STAFF,
        staff: true,
        time: "11:52",
        text_key: "home.preview.message_2",
    },
    PreviewMessage {
        author: PREVIEW_MEMBER,
        staff: false,
        time: "11:53",
        text_key: "home.preview.message_3",
    },
];

#[function_component(TranscriptPreview)]
fn transcript_preview() -> Html {
    let (i18n, _) = use_translation();

    html! {
        <div
            aria-hidden="true"
            class="absolute left-24 top-1/2 w-[760px] -translate-y-1/2 overflow-hidden rounded-2xl border border-line-strong bg-surface shadow-[0_24px_80px_rgba(0,0,0,0.55)]"
        >
            <div class="flex h-12 items-center gap-3 border-b border-line px-5">
                <TicketStub number="1290" />
                <span class="text-sm font-medium text-fg">{PREVIEW_MEMBER}</span>
                <span class="text-[13px] text-fg-muted">{i18n.t("home.preview.category")}</span>
                <span class="flex-1" />
                <Badge tone={Tone::Accent}>{i18n.t("home.preview.open")}</Badge>
            </div>
            <div class="flex flex-col gap-5 px-5 py-5">
                { for PREVIEW_MESSAGES.iter().map(|message| html! {
                    <div class="flex gap-3">
                        <Avatar name={message.author} size={32} member={!message.staff} />
                        <div class="flex min-w-0 max-w-[520px] flex-col gap-1">
                            <div class="flex items-center gap-2">
                                <span class="text-sm font-medium text-fg">{message.author}</span>
                                if message.staff {
                                    <Badge tone={Tone::Accent}>{i18n.t("home.preview.staff")}</Badge>
                                    <span class="font-mono text-xs text-fg-muted">{"#1"}</span>
                                }
                                <span class="font-mono text-xs text-fg-muted">{message.time}</span>
                            </div>
                            <p class="text-sm text-fg">{i18n.t(message.text_key)}</p>
                        </div>
                    </div>
                }) }
                <span class="text-xs text-fg-muted">
                    {i18n.t("home.preview.typing").replace("{name}", PREVIEW_STAFF)}
                </span>
            </div>
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let (i18n, _) = use_translation();

    html! {
        <main class="flex min-h-screen bg-surface-page">
            <section class="flex min-h-screen w-full flex-col px-10 py-24 lg:w-[640px] lg:shrink-0">
                {brand()}
                <div class="flex flex-1 flex-col justify-center gap-6 py-16">
                    <div class="flex flex-col gap-4">
                        <h1 class="max-w-[460px] text-display font-semibold text-fg">
                            {i18n.t("home.headline")}
                        </h1>
                        <p class="max-w-[460px] text-base text-fg-secondary">{i18n.t("home.subhead")}</p>
                    </div>
                    <div>
                        <a
                            href="/api/auth/login"
                            class="inline-flex h-11 items-center gap-2 rounded-xl bg-blurple px-5 text-sm font-medium text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.16)] transition-colors duration-150 hover:bg-blurple/90"
                        >
                            <Icon name={IconName::MessageSquare} />
                            {i18n.t("home.connect_button")}
                        </a>
                    </div>
                </div>
                <div class="flex items-center gap-3">
                    <LanguageSwitcher />
                </div>
            </section>
            <section class="relative hidden flex-1 overflow-hidden border-l border-line bg-surface-sunken lg:block">
                <TranscriptPreview />
            </section>
        </main>
    }
}
