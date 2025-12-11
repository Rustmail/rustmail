use strum::EnumString;

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum BotStatus {
    Online,
    Idle,
    Dnd,
    Invisible,
    Maintenance,
}
