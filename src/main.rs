use bot::{
    client::{DevGuild, PrefixCache, PrefixType, RequestClient, ShardManagerContainer},
    handler::Handler,
    websocket,
};
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, USER_AGENT},
    RequestBuilder,
};
use serenity::{all::GatewayIntents, Client};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;

const INTENTS: GatewayIntents = GatewayIntents::all();

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected Discord Token in env");
    let access_token = env::var("API_ACCESS_TOKEN").expect("Can't access API without token");

    let mut client = bot::client::setup_client(token, INTENTS).await;
    request_access(&mut client, access_token.clone()).await;
    initialize_dev_guild(&mut client).await;
    initialize_prefixes(&mut client).await;
    clone_shard_manager(&mut client).await;

    // websocket::initialize_websocket(&mut client, access_token.clone()).await;
    if let Err(why) = client.start_shards(1).await {
        println!("Client error: {:?}", why);
    }
}

async fn request_access(client: &mut Client, access_code: String) {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", access_code).parse().unwrap(),
    );

    let client_builder = reqwest::Client::builder()
        .user_agent("DiscordBot")
        .default_headers(headers);

    if let Ok(request_client) = client_builder.build() {
        let mut data = client.data.write().await;
        data.insert::<RequestClient>(request_client);
    }
}

async fn initialize_prefixes(client: &mut Client) {
    let mut data = client.data.write().await;
    let mut prefix_data: HashMap<PrefixType, String> = HashMap::new();
    prefix_data.insert(PrefixType::Default, "!".to_string());
    data.insert::<PrefixCache>(Arc::new(Mutex::new(prefix_data)));
}

async fn initialize_dev_guild(client: &mut Client) {
    let mut data = client.data.write().await;
    let dev_server_id = env::var("GUILD_ID").expect("Expected Guild ID in env");
    let dev_server_id = dev_server_id
        .parse::<u64>()
        .expect("Expected Guild ID to be a number");
    data.insert::<DevGuild>(dev_server_id);
}

async fn clone_shard_manager(client: &mut Client) {
    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
}
