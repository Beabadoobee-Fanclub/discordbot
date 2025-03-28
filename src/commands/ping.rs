use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::{client::ShardManagerContainer, InteractionCommandResult};

pub async fn run(ctx: &Context, _interaction: &CommandInteraction) -> InteractionCommandResult {
    let data = ctx.data.read().await;
    let shard_manager = data
        .get::<ShardManagerContainer>()
        .expect("Expected ShardManager in TypeMap");
    // let client_latency = ctx.h

    let runners = shard_manager.runners.lock().await;
    println!("Runners: {:?}", runners);
    let runner = runners
        .get(&ctx.shard_id)
        .unwrap_or_else(|| panic!("Expected Shard Runner for Shard ID: {}", ctx.shard_id));

    let content = match runner.latency {
        Some(latency) => format!("Pong! Latency: {}ms", latency.as_millis()),
        None => "Pong!".to_string(),
    };

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content(content),
    ))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Replies with Pong!")
}
