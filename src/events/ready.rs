use serenity::all::{Context, Ready};

pub async fn handle(_: &Context, ready: Ready) {
    println!("{} is online !", ready.user.name);
}
