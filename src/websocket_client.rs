use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct SubscribeMessage {
    op: String,
    args: Vec<SubscribeArgs>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SubscribeArgs {
    channel: String,
    instId: String,
}

pub struct OKXWebSocketClient {
    url: String,
}

impl OKXWebSocketClient {
    pub fn new(url: &str) -> Self {
        OKXWebSocketClient {
            url: url.to_string(),
        }
    }

    pub async fn subscribe_to_order_book(&self, symbol: &str, tx: mpsc::Sender<String>) {
        let (ws_stream, _) = connect_async(Url::parse(&self.url).unwrap())
            .await
            .expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let subscribe_message = SubscribeMessage {
            op: "subscribe".to_string(),
            args: vec![SubscribeArgs {
                channel: "books".to_string(),
                instId: symbol.to_string(),
            }],
        };

        let msg = serde_json::to_string(&subscribe_message).unwrap();
        let (mut write, mut read) = ws_stream.split();

        write.send(Message::Text(msg)).await.unwrap();

        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    tx.send(text).await.unwrap();
                }
                Ok(Message::Ping(_)) | Ok(Message::Pong(_)) | Ok(Message::Binary(_)) => (),
                Ok(Message::Close(_)) => {
                    println!("WebSocket connection closed");
                    break;
                }
                Err(e) => {
                    eprintln!("Error receiving message: {:?}", e);
                    break;
                }
            }
        }
    }
}