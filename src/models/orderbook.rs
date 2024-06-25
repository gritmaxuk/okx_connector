use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Orderbook {
    pub asks: Vec<(f64, f64)>,
    pub bids: Vec<(f64, f64)>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderbookSnapshotResponse {
    code: String,
    msg: String,
    data: Vec<Orderbook>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderbookUpdate {
    asks: Vec<(f64, f64)>,
    bids: Vec<(f64, f64)>,
}

impl Orderbook {
    pub fn from_snapshot(data: &str) -> Result<Self, Box<dyn Error>> {
        let response: OrderbookSnapshotResponse = serde_json::from_str(data)?;
        let orderbook = response.data.into_iter().next().ok_or("Empty response data")?;
        Ok(orderbook)
    }

    pub fn apply_update(&mut self, update: &str) {
        let update: OrderbookUpdate = serde_json::from_str(update).unwrap();
        self.asks.extend(update.asks);
        self.bids.extend(update.bids);
        self.sort_order_book();
    }

    fn sort_order_book(&mut self) {
        self.asks.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.bids.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderbookResponse {
    code: String,
    msg: String,
    data: Vec<Orderbook>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_from_snapshot() {
        let data = r#"{"code":"0","msg":"","data":[{"asks":[[41006.8,0.60030921]],"bids":[[41006.3,0.30178210]],"ts":"1621447077008"}]}"#;
        let orderbook = Orderbook::from_snapshot(data).unwrap();
        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 1);
        assert_eq!(orderbook.asks[0], (41006.8, 0.60030921));
        assert_eq!(orderbook.bids[0], (41006.3, 0.30178210));
    }

    #[test]
    fn test_orderbook_apply_update() {
        let data = r#"{"code":"0","msg":"","data":[{"asks":[[41006.8,0.60030921]],"bids":[[41006.3,0.30178210]],"ts":"1621447077008"}]}"#;
        let mut orderbook = Orderbook::from_snapshot(data).unwrap();
        let update = r#"{"asks":[[41007.0,0.20000000]],"bids":[[41005.0,0.10000000]]}"#;
        orderbook.apply_update(update);
        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert_eq!(orderbook.asks[1], (41007.0, 0.20000000));
        assert_eq!(orderbook.bids[1], (41005.0, 0.10000000));
    }

    #[test]
    fn test_orderbook_sort_order_book() {
        let mut orderbook = Orderbook {
            asks: vec![(41007.0, 0.20000000), (41006.8, 0.60030921)],
            bids: vec![(41005.0, 0.10000000), (41006.3, 0.30178210)],
        };
        orderbook.sort_order_book();
        assert_eq!(orderbook.asks, vec![(41006.8, 0.60030921), (41007.0, 0.20000000)]);
        assert_eq!(orderbook.bids, vec![(41006.3, 0.30178210), (41005.0, 0.10000000)]);
    }
}