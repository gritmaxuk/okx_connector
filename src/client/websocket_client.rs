use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Channel send error: {0}")]
    ChannelSendError(#[from] tokio::sync::mpsc::error::SendError<String>),
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

    pub async fn subscribe_to_order_book(&self, symbol: &str, tx: mpsc::Sender<String>) -> Result<(), WebSocketError> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        println!("WebSocket handshake has been successfully completed");

        let (mut write, mut read) = ws_stream.split();

        let subscribe_message = serde_json::json!({
            "op": "subscribe",
            "args": [{
                "channel": "books",
                "instId": symbol
            }]
        });

        write.send(Message::Text(subscribe_message.to_string())).await?;

        while let Some(message) = read.next().await {
            match message? {
                Message::Text(text) => {
                    tx.send(text).await?;
                }
                Message::Close(frame) => {
                    println!("WebSocket connection closed: {:?}", frame);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}