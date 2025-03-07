use bot::{handler::Handler, PrefixCache, PrefixType};
use dotenv::dotenv;
use serenity::{
    all::{ClientBuilder, GatewayIntents},
    Client,
};
use std::{collections::HashMap, env};

const INTENTS: GatewayIntents = GatewayIntents::all();

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected Discord Token in env");

    let mut client = setup_client(token).await;

    initialize_prefixes(&mut client).await;

    if let Err(why) = client.start_shards(1).await {
        println!("Client error: {:?}", why);
    }
}

async fn setup_client(token: String) -> Client {
    ClientBuilder::new(token, INTENTS)
        .event_handler(Handler)
        .await
        .expect("Error Starting Client")
}

async fn initialize_prefixes(client: &mut Client) {
    let mut data = client.data.write().await;
    let mut prefix_data: HashMap<PrefixType, String> = HashMap::new();
    prefix_data.insert(PrefixType::Default, "!".to_string());
    data.insert::<PrefixCache>(prefix_data);
}
