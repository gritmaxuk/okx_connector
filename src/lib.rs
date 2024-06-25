pub mod rest_client;
pub mod websocket_client;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::rest_client::OKXRestClient;
    use super::websocket_client::OKXWebSocketClient;
    use super::utils::{parse_order_book, validate_order_book_data, Orderbook};
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

    #[test]
    fn test_parse_order_book() {
        let data = r#"{"asks":[[41006.8,0.60030921]],"bids":[[41006.3,0.30178210]],"ts":"1621447077008"}"#;
        let orderbook = parse_order_book(data);
        assert!(orderbook.is_ok());
    }

    #[test]
    fn test_validate_order_book_data() {
        let valid_data = r#"{"asks":[[41006.8,0.60030921]],"bids":[[41006.3,0.30178210]],"ts":"1621447077008"}"#;
        let invalid_data = r#"{"ask":[[41006.8,0.60030921]],"bids":[[41006.3,0.30178210]],"ts":"1621447077008"}"#;

        assert!(validate_order_book_data(valid_data).is_ok());
        assert!(validate_order_book_data(invalid_data).is_err());
    }
}