use serenity::all::{Context, Ready};

pub async fn ready(ctx: Context, msg: Ready) {
    println!("{} is connected!", msg.user.name);
}
