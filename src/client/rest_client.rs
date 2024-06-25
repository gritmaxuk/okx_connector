use reqwest::Client;
use std::error::Error;
use crate::models::Orderbook;

pub struct OKXRestClient {
    base_url: String,
    client: Client,
}

impl OKXRestClient {
    pub fn new(base_url: &str) -> Self {
        OKXRestClient {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_order_book(&self, symbol: &str) -> Result<Orderbook, Box<dyn Error>> {
        let url = format!("{}/api/v5/market/books?instId={}", self.base_url, symbol);
        let response = self.client.get(&url).send().await?.text().await?;
        Orderbook::from_snapshot(&response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_order_book() {
        let client = OKXRestClient::new("https://www.okx.com");
        match client.get_order_book("BTC-USDT").await {
            Ok(orderbook) => {
                assert!(!orderbook.asks.is_empty());
                assert!(!orderbook.bids.is_empty());
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}