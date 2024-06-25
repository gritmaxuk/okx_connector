use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
struct SubscribeMessage {
    op: String,
    args: Vec<SubscribeArgs>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SubscribeArgs {
    channel: String,
    #[serde(rename = "instId")]
    inst_id: String,
}

#[derive(Error, Debug)]
pub enum OKXWebSocketError {
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
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

    pub async fn subscribe_to_order_book(
        &self,
        symbol: &str,
        tx: mpsc::Sender<String>,
    ) -> Result<(), OKXWebSocketError> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        println!("WebSocket handshake has been successfully completed");

        let (mut write, mut read) = ws_stream.split();

        self.send_subscribe_message(&mut write, symbol).await?;

        self.handle_messages(read, write, tx).await
    }

    async fn send_subscribe_message(
        &self,
        write: &mut futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>
            >,
            Message,
        >,
        symbol: &str,
    ) -> Result<(), OKXWebSocketError> {
        let subscribe_message = SubscribeMessage {
            op: "subscribe".to_string(),
            args: vec![SubscribeArgs {
                channel: "books".to_string(),
                inst_id: symbol.to_string(),
            }],
        };
        let msg = serde_json::to_string(&subscribe_message)?;
        write.send(Message::Text(msg)).await?;
        Ok(())
    }

    async fn handle_messages(
        &self,
        mut read: futures_util::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>
            >,
        >,
        mut write: futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>
            >,
            Message,
        >,
        tx: mpsc::Sender<String>,
    ) -> Result<(), OKXWebSocketError> {
        while let Some(message) = read.next().await {
            match message? {
                Message::Text(text) => {
                    if let Err(_) = tx.send(text).await {
                        println!("Failed to send message through channel");
                        break;
                    }
                }
                Message::Binary(data) => {
                    println!("Received binary data: {} bytes", data.len());
                }
                Message::Ping(data) => {
                    println!("Received ping");
                    write.send(Message::Pong(data)).await?;
                }
                Message::Pong(_) => {
                    println!("Received pong");
                }
                Message::Close(frame) => {
                    println!("WebSocket connection closed: {:?}", frame);
                    break;
                }
                Message::Frame(frame) => {
                    println!("Received raw frame: {:?}", frame);
                }
            }
        }
        Ok(())
    }
}