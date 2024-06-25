## How to run 
```bash
cargo run --bin okx_demo
```

## 1. Requirements Analysis

### Key Requirements:
- **Realtime Market Data:** Connect to OKX WebSocket API to receive order book snapshots and updates.
- **REST API Request:** Implement a REST API client to make at least one request to get order book data.
- **Focus on Types and Design:** Emphasize on creating clear and extendable types, particularly for the `Orderbook`.
- **Usability and Safety:** Ensure the code is user-friendly, safe from incorrect usage, and easy to extend.

### Constraints:
- **No Private API:** Only public API needs to be handled.
- **Orderbook Type:** Should handle snapshots and incremental updates, sorting asks in ascending and bids in descending order.

## 2. Design Plan

### High-Level Architecture:
- **WebSocket Client:** Handles the connection to the WebSocket API, subscribes to the order book channel, and processes incoming messages.
- **REST Client:** A simple wrapper around the REST API to fetch order book data.
- **Orderbook Type:** A class that can apply updates to snapshots, maintaining the correct order of asks and bids.
- **Utilities and Helpers:** Various utilities to aid in parsing, validating, and managing data.

### Detailed Components:

#### 2.1 WebSocket Client
- **Class:** `OKXWebSocketClient`
  - **Methods:**
    - `connect()`: Establish a WebSocket connection.
    - `subscribe_to_order_book(symbol: str)`: Subscribe to the order book channel for a specific symbol.
    - `on_message()`: Handle incoming messages, updating the `Orderbook` as needed.
    - `disconnect()`: Close the WebSocket connection.
  - **Attributes:**
    - `url`: WebSocket endpoint.
    - `subscriptions`: Active subscriptions for symbols.
    - `orderbook`: An instance of `Orderbook` to apply updates.

- **WebSocket Request Example:**
  ```json
  {
    "op": "subscribe",
    "args": [
      {
        "channel": "books",
        "instId": "BTC-USDT"
      }
    ]
  }
  ```

- **WebSocket Response Example:**
  ```json
  {
    "event": "subscribe",
    "arg": {
      "channel": "books",
      "instId": "BTC-USDT"
    },
    "connId": "a4d3ae55"
  }
  ```

#### 2.2 REST Client
- **Class:** `OKXRestClient`
  - **Methods:**
    - `get_order_book(symbol: str) -> Orderbook`: Fetch the order book data for a specific symbol.
  - **Attributes:**
    - `base_url`: Base URL for the REST API.
    - `session`: HTTP session to manage requests.

- **REST API Request Example:**
  ```http
  GET /api/v5/market/books?instId=BTC-USDT
  ```

- **REST API Response Example:**
  ```json
  {
    "code": "0",
    "msg": "",
    "data": [
      {
        "asks": [
          ["41006.8", "0.60030921", "0", "1"]
        ],
        "bids": [
          ["41006.3", "0.30178210", "0", "2"]
        ],
        "ts": "1621447077008"
      }
    ]
  }
  ```

#### 2.3 Orderbook
- **Class:** `Orderbook`
  - **Methods:**
    - `from_snapshot(data: dict) -> Orderbook`: Create an `Orderbook` instance from a snapshot.
    - `apply_update(update: dict)`: Apply an incremental update to the order book.
    - `sort_order_book()`: Ensure asks and bids are sorted correctly.
  - **Attributes:**
    - `asks`: List of asks, each with price and quantity.
    - `bids`: List of bids, each with price and quantity.

#### 2.4 Utilities and Helpers
- **Functions:**
  - `parse_order_book(data: dict) -> Orderbook`: Parse raw order book data into an `Orderbook` instance.
  - `validate_order_book_data(data: dict)`: Ensure the integrity and correctness of order book data.

### Type Definitions:
- **Orderbook Type:**
  - `asks: List[Tuple[float, float]]`
  - `bids: List[Tuple[float, float]]`

### Example Usage:
```python
# Initialize the REST client and fetch order book snapshot
rest_client = OKXRestClient(base_url="https://www.okx.com")
snapshot = rest_client.get_order_book("BTC-USDT")

# Initialize the WebSocket client and connect
ws_client = OKXWebSocketClient(url="wss://ws.okx.com:8443")
ws_client.connect()
ws_client.subscribe_to_order_book("BTC-USDT")

# Handle incoming messages to update the order book
def handle_message(message):
    update = parse_order_book(message)
    snapshot.apply_update(update)
    snapshot.sort_order_book()

ws_client.on_message = handle_message

# At this point, `snapshot` will be continuously updated with real-time data
```

## 3. Implementation Plan

1. **Set Up Project Structure:**
   - Create directories and files for `websocket_client.py`, `rest_client.py`, `orderbook.py`, `utils.py`.
   - Set up a virtual environment and install necessary dependencies (`websockets`, `requests`, etc.).

2. **Implement the Orderbook Class:**
   - Define the class with methods to handle snapshots and updates.
   - Ensure the sorting logic for asks and bids is correct.

3. **Implement the REST Client:**
   - Create methods to perform the HTTP request and parse the response into an `Orderbook`.

4. **Implement the WebSocket Client:**
   - Establish connection, manage subscriptions, and handle incoming messages.
   - Integrate the order book updates with the `Orderbook` class.

5. **Write Utilities and Helpers:**
   - Implement parsing and validation functions.

6. **Testing:**
   - Write unit tests for each component.
   - Test the end-to-end flow from fetching the snapshot to updating with real-time data.

7. **Documentation:**
   - Provide clear documentation and examples for each component and its usage.
