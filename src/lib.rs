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

struct Command {
    name: String,
    description: String,
    usage: String,
}

trait CommandHandler {
    fn handle(&self);
}
