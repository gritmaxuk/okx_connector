use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use url::Url;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum OKXClientError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Failed to parse float: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Failed to parse integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Unexpected response structure: {0}")]
    UnexpectedResponseStructure(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orderbook {
    pub asks: Vec<(String, String, String, String)>,
    pub bids: Vec<(String, String, String, String)>,
    pub ts: String,
}

impl Orderbook {
    pub fn parse_floats(&self) -> Result<ParsedOrderbook, OKXClientError> {
        Ok(ParsedOrderbook {
            asks: self.parse_vec(&self.asks)?,
            bids: self.parse_vec(&self.bids)?,
            ts: self.ts.parse::<u64>()?,
        })
    }

    fn parse_vec(&self, vec: &Vec<(String, String, String, String)>) -> Result<Vec<(f64, f64)>, OKXClientError> {
        vec.iter()
            .map(|(price, amount, _, _)| {
                Ok((price.parse::<f64>()?, amount.parse::<f64>()?))
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ParsedOrderbook {
    pub asks: Vec<(f64, f64)>,
    pub bids: Vec<(f64, f64)>,
    pub ts: u64,
}

pub struct OKXRestClient {
    base_url: Url,
    client: Client,
}

impl OKXRestClient {
    pub fn new(base_url: &str) -> Result<Self, OKXClientError> {
        Ok(OKXRestClient {
            base_url: Url::parse(base_url)?,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("OKX-Rust-Client/1.0")
                .build()?,
        })
    }

    pub async fn get_order_book(&self, symbol: &str) -> Result<ParsedOrderbook, OKXClientError> {
        let url = self.base_url.join(&format!("api/v5/market/books?instId={}", symbol))?;
        let response_text = self.client.get(url).send().await?.text().await?;
        
        println!("Raw API response: {}", response_text);

        let response_value: Value = serde_json::from_str(&response_text)?;

        // Check if the response has the expected structure
        let orderbook_data = response_value["data"]
            .as_array()
            .and_then(|arr| arr.get(0))
            .ok_or_else(|| OKXClientError::UnexpectedResponseStructure("Missing 'data' array or empty".into()))?;

        let orderbook: Orderbook = serde_json::from_value(orderbook_data.clone())?;
        orderbook.parse_floats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_get_order_book() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v5/market/books"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "code": "0",
                    "msg": "",
                    "data": [{
                        "asks": [["50000", "1", "0", "7"]],
                        "bids": [["49999", "1", "0", "6"]],
                        "ts": "1719335318504"
                    }]
                })))
            .mount(&mock_server)
            .await;

        let client = OKXRestClient::new(&mock_server.uri()).unwrap();

        let result = client.get_order_book("BTC-USDT").await;

        match result {
            Ok(orderbook) => {
                println!("Orderbook: {:?}", orderbook);
                assert_eq!(orderbook.asks.len(), 1);
                assert_eq!(orderbook.bids.len(), 1);
                assert_eq!(orderbook.asks[0], (50000.0, 1.0));
                assert_eq!(orderbook.bids[0], (49999.0, 1.0));
                assert_eq!(orderbook.ts, 1719335318504);
            },
            Err(e) => {
                println!("Error: {:?}", e);
                panic!("Test failed: {}", e);
            }
        }
    }
}