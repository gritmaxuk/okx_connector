pub mod rest_client;
pub mod websocket_client;

#[cfg(test)]
mod tests {
    use super::rest_client::OKXRestClient;
    use super::websocket_client::OKXWebSocketClient;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_rest_client() {
        let client = OKXRestClient::new("https://www.okx.com");
        match client.get_order_book("BTC-USDT").await {
            Ok(orderbook) => println!("Orderbook: {:?}", orderbook),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_websocket_client() {
        let (tx, mut rx) = mpsc::channel(100);
        let client = OKXWebSocketClient::new("wss://ws.okx.com:8443");
        tokio::spawn(async move {
            client.subscribe_to_order_book("BTC-USDT", tx).await;
        });

        while let Some(update) = rx.recv().await {
            println!("Received update: {}", update);
        }
    }
}