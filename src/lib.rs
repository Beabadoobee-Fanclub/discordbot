use std::{collections::HashMap, hash::Hash, sync::Arc};

use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;

pub mod client;
pub mod commands;
pub mod handler;
pub mod websocket;

// pub enum

pub type InteractionCommandResult =
    Result<serenity::builder::CreateInteractionResponse, Box<dyn std::error::Error + Send + Sync>>;

struct Command {
    name: String,
    description: String,
    usage: String,
}

trait CommandHandler {
    fn handle(&self);
}
