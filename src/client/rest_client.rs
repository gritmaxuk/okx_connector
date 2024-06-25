use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orderbook {
    pub asks: Vec<(String, String)>,
    pub bids: Vec<(String, String)>,
}

impl Orderbook {
    pub fn parse_floats(&self) -> Result<ParsedOrderbook, OKXClientError> {
        Ok(ParsedOrderbook {
            asks: self.parse_vec(&self.asks)?,
            bids: self.parse_vec(&self.bids)?,
        })
    }

    fn parse_vec(&self, vec: &Vec<(String, String)>) -> Result<Vec<(f64, f64)>, OKXClientError> {
        vec.iter()
            .map(|(price, amount)| {
                Ok((price.parse::<f64>()?, amount.parse::<f64>()?))
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ParsedOrderbook {
    pub asks: Vec<(f64, f64)>,
    pub bids: Vec<(f64, f64)>,
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
        let response: Orderbook = self.client.get(url).send().await?.json().await?;
        response.parse_floats()
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
                    "asks": [["50000", "1"], ["50001", "2"]],
                    "bids": [["49999", "1"], ["49998", "2"]]
                })))
            .mount(&mock_server)
            .await;

        let client = OKXRestClient::new(&mock_server.uri()).unwrap();

        let result = client.get_order_book("BTC-USDT").await;

        match result {
            Ok(orderbook) => {
                println!("Orderbook: {:?}", orderbook);
                assert_eq!(orderbook.asks.len(), 2);
                assert_eq!(orderbook.bids.len(), 2);
                assert_eq!(orderbook.asks[0], (50000.0, 1.0));
                assert_eq!(orderbook.bids[0], (49999.0, 1.0));
            },
            Err(e) => {
                println!("Error: {:?}", e);
                panic!("Test failed: {}", e);
            }
        }
    }
}