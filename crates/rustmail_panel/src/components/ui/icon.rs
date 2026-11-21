use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IconName {
    Inbox,
    MessageSquare,
    LayoutDashboard,
    ChartLine,
    Folder,
    Ban,
    KeyRound,
    Settings,
    Shield,
    LogOut,
    Search,
    ChevronDown,
    Plus,
    X,
    Check,
    Clock,
    Paperclip,
    Languages,
    User,
    Users,
    ArrowUpRight,
    Copy,
    TrashTwo,
    RefreshCw,
    Power,
    CircleAlert,
    Lock,
    ExternalLink,
    Ticket,
    FileText,
    StickyNote,
    ArrowLeft,
    ArrowRight,
    Play,
    Square,
    RotateCcw,
    CircleCheck,
    Upload,
    Menu,
    TriangleAlert,
    Info,
    LoaderCircle,
}

impl IconName {
    fn paths(self) -> &'static str {
        match self {
            IconName::Inbox => {
                r#"<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/> <path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>"#
            }
            IconName::MessageSquare => {
                r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/>"#
            }
            IconName::LayoutDashboard => {
                r#"<rect width="7" height="9" x="3" y="3" rx="1"/> <rect width="7" height="5" x="14" y="3" rx="1"/> <rect width="7" height="9" x="14" y="12" rx="1"/> <rect width="7" height="5" x="3" y="16" rx="1"/>"#
            }
            IconName::ChartLine => {
                r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/> <path d="m19 9-5 5-4-4-3 3"/>"#
            }
            IconName::Folder => {
                r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#
            }
            IconName::Ban => {
                r#"<circle cx="12" cy="12" r="10"/> <path d="M4.929 4.929 19.07 19.071"/>"#
            }
            IconName::KeyRound => {
                r#"<path d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z"/> <circle cx="16.5" cy="7.5" r=".5" fill="currentColor"/>"#
            }
            IconName::Settings => {
                r#"<path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"/> <circle cx="12" cy="12" r="3"/>"#
            }
            IconName::Shield => {
                r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/>"#
            }
            IconName::LogOut => {
                r#"<path d="m16 17 5-5-5-5"/> <path d="M21 12H9"/> <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>"#
            }
            IconName::Search => r#"<path d="m21 21-4.34-4.34"/> <circle cx="11" cy="11" r="8"/>"#,
            IconName::ChevronDown => r#"<path d="m6 9 6 6 6-6"/>"#,
            IconName::Plus => r#"<path d="M5 12h14"/> <path d="M12 5v14"/>"#,
            IconName::X => r#"<path d="M18 6 6 18"/> <path d="m6 6 12 12"/>"#,
            IconName::Check => r#"<path d="M20 6 9 17l-5-5"/>"#,
            IconName::Clock => r#"<circle cx="12" cy="12" r="10"/> <path d="M12 6v6l4 2"/>"#,
            IconName::Paperclip => {
                r#"<path d="m16 6-8.414 8.586a2 2 0 0 0 2.829 2.829l8.414-8.586a4 4 0 1 0-5.657-5.657l-8.379 8.551a6 6 0 1 0 8.485 8.485l8.379-8.551"/>"#
            }
            IconName::Languages => {
                r#"<path d="m5 8 6 6"/> <path d="m4 14 6-6 2-3"/> <path d="M2 5h12"/> <path d="M7 2h1"/> <path d="m22 22-5-10-5 10"/> <path d="M14 18h6"/>"#
            }
            IconName::User => {
                r#"<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/> <circle cx="12" cy="7" r="4"/>"#
            }
            IconName::Users => {
                r#"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/> <path d="M16 3.128a4 4 0 0 1 0 7.744"/> <path d="M22 21v-2a4 4 0 0 0-3-3.87"/> <circle cx="9" cy="7" r="4"/>"#
            }
            IconName::ArrowUpRight => r#"<path d="M7 7h10v10"/> <path d="M7 17 17 7"/>"#,
            IconName::Copy => {
                r#"<rect width="14" height="14" x="8" y="8" rx="2" ry="2"/> <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#
            }
            IconName::TrashTwo => {
                r#"<path d="M10 11v6"/> <path d="M14 11v6"/> <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/> <path d="M3 6h18"/> <path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>"#
            }
            IconName::RefreshCw => {
                r#"<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/> <path d="M21 3v5h-5"/> <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/> <path d="M8 16H3v5"/>"#
            }
            IconName::Power => r#"<path d="M12 2v10"/> <path d="M18.4 6.6a9 9 0 1 1-12.77.04"/>"#,
            IconName::CircleAlert => {
                r#"<circle cx="12" cy="12" r="10"/> <line x1="12" x2="12" y1="8" y2="12"/> <line x1="12" x2="12.01" y1="16" y2="16"/>"#
            }
            IconName::Lock => {
                r#"<rect width="18" height="11" x="3" y="11" rx="2" ry="2"/> <path d="M7 11V7a5 5 0 0 1 10 0v4"/>"#
            }
            IconName::ExternalLink => {
                r#"<path d="M15 3h6v6"/> <path d="M10 14 21 3"/> <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>"#
            }
            IconName::Ticket => {
                r#"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/> <path d="M13 5v2"/> <path d="M13 17v2"/> <path d="M13 11v2"/>"#
            }
            IconName::FileText => {
                r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/> <path d="M14 2v5a1 1 0 0 0 1 1h5"/> <path d="M10 9H8"/> <path d="M16 13H8"/> <path d="M16 17H8"/>"#
            }
            IconName::StickyNote => {
                r#"<path d="M21 9a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2z"/> <path d="M15 3v5a1 1 0 0 0 1 1h5"/>"#
            }
            IconName::ArrowLeft => r#"<path d="m12 19-7-7 7-7"/> <path d="M19 12H5"/>"#,
            IconName::ArrowRight => r#"<path d="M5 12h14"/> <path d="m12 5 7 7-7 7"/>"#,
            IconName::Play => {
                r#"<path d="M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z"/>"#
            }
            IconName::Square => r#"<rect width="18" height="18" x="3" y="3" rx="2"/>"#,
            IconName::RotateCcw => {
                r#"<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/> <path d="M3 3v5h5"/>"#
            }
            IconName::CircleCheck => {
                r#"<circle cx="12" cy="12" r="10"/> <path d="m16 9-5.5 5.5L8 12"/>"#
            }
            IconName::Upload => {
                r#"<path d="M12 3v12"/> <path d="m17 8-5-5-5 5"/> <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>"#
            }
            IconName::Menu => r#"<path d="M4 5h16"/> <path d="M4 12h16"/> <path d="M4 19h16"/>"#,
            IconName::TriangleAlert => {
                r#"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/> <path d="M12 9v4"/> <path d="M12 17h.01"/>"#
            }
            IconName::Info => {
                r#"<circle cx="12" cy="12" r="10"/> <path d="M12 16v-4"/> <path d="M12 8h.01"/>"#
            }
            IconName::LoaderCircle => r#"<path d="M21 12a9 9 0 1 1-6.219-8.56"/>"#,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub name: IconName,
    #[prop_or(16)]
    pub size: u32,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let class = classes!("shrink-0", props.class.clone()).to_string();
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="{stroke}" stroke-linecap="round" stroke-linejoin="round" class="{class}" aria-hidden="true">{paths}</svg>"#,
        size = props.size,
        stroke = 1.5 * 24.0 / props.size as f32,
        class = class,
        paths = props.name.paths(),
    );
    Html::from_html_unchecked(AttrValue::from(svg))
}
