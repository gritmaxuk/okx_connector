use serde_json::Value;
use std::error::Error;
use crate::models::Orderbook;

pub fn parse_order_book(data: &str) -> Result<Orderbook, Box<dyn Error>> {
    let v: Value = serde_json::from_str(data)?;
    
    // Check if the data field is present and is an array
    let data_array = v["data"].as_array().ok_or("Missing or invalid 'data' field")?;
    
    // Get the first element of the data array
    let first_data = data_array.get(0).ok_or("Empty 'data' array")?;
    
    // Extract asks and bids
    let asks = first_data["asks"].as_array().ok_or("Missing or invalid 'asks' field")?;
    let bids = first_data["bids"].as_array().ok_or("Missing or invalid 'bids' field")?;
    
    // Construct the orderbook
    let orderbook = Orderbook {
        asks: parse_orders(asks)?,
        bids: parse_orders(bids)?,
    };
    
    Ok(orderbook)
}

fn parse_orders(orders: &[Value]) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    orders
        .iter()
        .map(|order| {
            let price = order[0].as_str().ok_or("Invalid price")?.parse::<f64>()?;
            let amount = order[1].as_str().ok_or("Invalid amount")?.parse::<f64>()?;
            Ok((price, amount))
        })
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_order_book() {
        let data = r#"{"code":"0","msg":"","data":[{"asks":[["41006.8","0.60030921"]],"bids":[["41006.3","0.30178210"]],"ts":"1621447077008"}]}"#;
        let orderbook = parse_order_book(data);
        assert!(orderbook.is_ok(), "Failed to parse orderbook: {:?}", orderbook.err());
        let orderbook = orderbook.unwrap();
        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 1);
        assert_eq!(orderbook.asks[0], (41006.8, 0.60030921));
        assert_eq!(orderbook.bids[0], (41006.3, 0.30178210));
    }

    #[test]
    fn test_validate_order_book_data() {
        let valid_data = r#"{"asks":[["41006.8","0.60030921"]],"bids":[["41006.3","0.30178210"]],"ts":"1621447077008"}"#;
        let invalid_data = r#"{"ask":[["41006.8","0.60030921"]],"bids":[["41006.3","0.30178210"]],"ts":"1621447077008"}"#;

        assert!(validate_order_book_data(valid_data).is_ok(), "Valid data failed validation");
        assert!(validate_order_book_data(invalid_data).is_err(), "Invalid data passed validation");
    }
}