use std::{collections::HashMap, sync::Arc};

use serenity::{
    all::{ClientBuilder, GatewayIntents, ShardManager},
    prelude::TypeMapKey,
    Client,
};
use tokio::sync::Mutex;

use crate::{handler::Handler, websocket};

pub type CachedPrefixes = Arc<Mutex<HashMap<PrefixType, String>>>;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PrefixType {
    Guild(String),
    Default,
}
pub struct RequestClient;
pub struct PrefixCache;
pub struct DevGuild;
pub struct BackendSocket;
pub struct ShardManagerContainer;

impl TypeMapKey for PrefixCache {
    type Value = CachedPrefixes;
}
impl TypeMapKey for DevGuild {
    type Value = u64;
}
impl TypeMapKey for BackendSocket {
    type Value = websocket::WebsocketConnection;
}
impl TypeMapKey for RequestClient {
    type Value = reqwest::Client;
}
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

pub async fn setup_client(token: String, intents: GatewayIntents) -> Client {
    ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error Starting Client")
}
