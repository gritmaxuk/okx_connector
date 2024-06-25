use okx_connector::client::{OKXRestClient, OKXWebSocketClient};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize REST client
    let rest_client = OKXRestClient::new("https://www.okx.com")?;
    
    // Fetch order book snapshot
    match rest_client.get_order_book("BTC-USDT").await {
        Ok(snapshot) => {
            println!("Initial Snapshot:");
            println!("Timestamp: {}", snapshot.ts);
            println!("Top 5 asks:");
            for (i, (price, amount)) in snapshot.asks.iter().take(5).enumerate() {
                println!("  {}. Price: {}, Amount: {}", i+1, price, amount);
            }
            println!("Top 5 bids:");
            for (i, (price, amount)) in snapshot.bids.iter().take(5).enumerate() {
                println!("  {}. Price: {}, Amount: {}", i+1, price, amount);
            }
        },
        Err(e) => {
            eprintln!("Error fetching order book: {:?}", e);
            return Err(e.into());
        }
    }

    // Initialize WebSocket client
    let ws_client = OKXWebSocketClient::new("wss://ws.okx.com:8443/ws/v5/public");

    // Create a channel for receiving WebSocket messages
    let (tx, mut rx) = mpsc::channel(100);

    // Spawn a task to handle WebSocket connection and messages
    let ws_handle = tokio::spawn(async move {
        if let Err(e) = ws_client.subscribe_to_order_book("BTC-USDT", tx).await {
            eprintln!("WebSocket error: {:?}", e);
        }
    });

    // Main loop to process incoming messages
    for _ in 0..10 {  // Process 10 updates as an example
        match rx.recv().await {
            Some(message) => {
                println!("Received update: {}", message);
            },
            None => {
                println!("WebSocket channel closed");
                break;
            }
        }
    }

    // Ensure the WebSocket task is properly closed
    ws_handle.abort();

    Ok(())
}