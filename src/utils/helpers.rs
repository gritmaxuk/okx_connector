use serde_json::Value;
use std::error::Error;
use crate::models::Orderbook;

pub fn parse_order_book(data: &str) -> Result<Orderbook, Box<dyn Error>> {
    serde_json::from_str(data).map_err(|e| e.into())
}

pub fn validate_order_book_data(data: &str) -> Result<(), Box<dyn Error>> {
    let v: Value = serde_json::from_str(data)?;
    
    if !v.is_object() {
        return Err("Data is not a valid JSON object".into());
    }
    
    let obj = v.as_object().unwrap();
    if !obj.contains_key("asks") || !obj.contains_key("bids") {
        return Err("Missing required fields: 'asks' or 'bids'".into());
    }
    
    Ok(())
}