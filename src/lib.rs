use std::{collections::HashMap, hash::Hash};

use serenity::prelude::TypeMapKey;

pub mod commands;
pub mod handler;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PrefixType {
    Guild(String),
    Default,
}
pub struct PrefixCache;

impl TypeMapKey for PrefixCache {
    type Value = HashMap<PrefixType, String>;
}

pub struct Command {
    name: String,
    description: String,
    usage: String,
    category: String,
}

pub trait CommandHandler {
    fn execute(&self);
    fn register() -> Command;
}
