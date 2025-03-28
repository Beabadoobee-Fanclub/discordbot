use std::error;

use serenity::all::{
    CommandInteraction, ComponentInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateModal, Interaction, Message, ModalInteraction,
    PingInteraction,
};

use crate::{
    client::{PrefixCache, PrefixType},
    commands::ping,
    InteractionCommandResult,
};

pub async fn handle_message_command(ctx: &Context, msg: &Message) -> bool {
    let data = ctx.data.read().await;
    let prefixes = data
        .get::<PrefixCache>()
        .expect("Expected PrefixCache in TypeMap")
        .lock()
        .await;
    let prefix = match prefixes.get(&PrefixType::Guild(msg.guild_id.unwrap().to_string())) {
        Some(p) => p,
        None => prefixes
            .get(&PrefixType::Default)
            .expect("Expected default prefix to be set"),
    };

    if !msg.content.starts_with(prefix) {
        return false;
    }

    let content = msg.content.trim_start_matches(prefix);
    println!("Command: {}", content);
    true
}

pub async fn handle_interaction(ctx: &Context, interaction: &Interaction) {
    match interaction {
        Interaction::Command(command) => handle_command(ctx, command).await,
        Interaction::Autocomplete(autocomplete) => handle_autocomplete(ctx, autocomplete).await,
        Interaction::Modal(modal) => handle_modal(ctx, modal).await,
        Interaction::Component(component) => handle_component(ctx, component).await,
        Interaction::Ping(ping) => handle_ping(ctx, ping).await,
        _ => {}
    };
}

pub async fn handle_command(ctx: &Context, command: &CommandInteraction) {
    println!("Command: {}", command.data.name);
    let response: InteractionCommandResult = match command.data.name.as_str() {
        "ping" => ping::run(&ctx, &command).await,
        _ => panic!("Command not found"),
    };

    match response {
        Ok(response) => command
            .create_response(&ctx.http, response)
            .await
            .expect("Error sending response"),
        Err(why) => println!("Error running command: {:?}", why.to_string()),
    }
}

pub async fn handle_autocomplete(ctx: &Context, autocomplete: &CommandInteraction) {}

pub async fn handle_modal(ctx: &Context, modal: &ModalInteraction) {}

pub async fn handle_component(ctx: &Context, component: &ComponentInteraction) {}

pub async fn handle_ping(ctx: &Context, ping: &PingInteraction) {}
