use serenity::all::{Command, Context, CreateCommand, GuildId, Ready};

use crate::{client::DevGuild, commands::ping};

pub async fn ready(ctx: Context, msg: Ready) {
    let data_store = ctx.data.read().await;
    let dev_guild = data_store
        .get::<DevGuild>()
        .expect("Expected DevGuild in TypeMap");

    let guild = GuildId::new(*dev_guild);
    let commands_vec: Vec<CreateCommand> = vec![ping::register()];

    let commands = guild.set_commands(&ctx.http, commands_vec).await;

    if let Err(why) = commands {
        println!("Error setting commands: {:?}", why);
    }

    // let guild_command = Command::create_global_command(&ctx.http, ping::register()).await; // This is for global commands

    println!("{} is connected!", msg.user.name);
}
