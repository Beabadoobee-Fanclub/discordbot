use std::sync::Mutex;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serenity::{
    all::token,
    futures::{
        stream::{SplitSink, SplitStream},
        StreamExt,
    },
    Client,
};

use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        http::{self, Uri},
        ClientRequestBuilder, Message,
    },
    Connector, MaybeTlsStream, WebSocketStream,
};

use crate::client::{BackendSocket, CachedPrefixes, PrefixCache};

const WEBSOCKET_URL: &str = "ws://127.0.0.1:8787/ws";

pub type SocketReader<S> = SplitStream<WebSocketStream<S>>;
pub type SocketWriter<S> = SplitSink<WebSocketStream<S>, Message>;

#[derive(Default)]
pub struct TlsConnector(pub Mutex<Option<Connector>>);

#[derive(Default)]
pub struct WebsocketConnection(pub Mutex<Option<SocketWriter<MaybeTlsStream<TcpStream>>>>);

pub async fn initialize_websocket(client: &mut Client, token: String) {
    let mut data = client.data.write().await;

    let req = ClientRequestBuilder::new(Uri::from_static(WEBSOCKET_URL))
        .with_header(AUTHORIZATION.to_string(), format!("Bearer {}", token))
        .with_header(USER_AGENT.to_string(), "DiscordBot".to_string());

    let (ws_stream, _) = connect_async(req)
        .await
        .expect("Failed to connect to websocket");
    println!("Connected to websocket");

    let (write, mut read) = ws_stream.split();

    let (prefix_data) = { (data.get::<PrefixCache>().unwrap().clone(),) };

    tokio::spawn(async move {
        while let Some(Ok(message)) = read.next().await {
            match message {
                Message::Text(text) => {
                    println!("Received message: {}", text);
                    // let prefix_data = prefix_data.lock().await;
                }
                _ => break,
            }
        }
    });

    data.insert::<BackendSocket>(WebsocketConnection(Mutex::new(Some(write))));
}
