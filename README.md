# Order Book

This project implements an order book and a matching engine for managing and matching buy (bid) and sell (ask) orders in a financial market.

## Project Structure

```
order_book/
├── src/
│   ├── main.rs
│   ├── matching_engine/
│   │   ├── mod.rs
│   │   ├── orderbook.rs
│   │   └── engine.rs
├── Cargo.toml
└── README.md
```

### Main Components

- **main.rs**: The entry point of the application.
- **matching_engine/mod.rs**: The module declaration for the matching engine.
- **matching_engine/orderbook.rs**: Contains the `OrderBook`, `Order`, `Limit`, `Price`, and `BidOrAsk` enums.
- **matching_engine/engine.rs**: Contains the `MatchingEngine` and `TradingPair` structs.

## Getting Started

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

### Running the Application

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/order_book.git
    cd order_book
    ```

2. Build and run the application:
    ```bash
    cargo run
    ```

## Code Overview

### main.rs

```rust
mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{Order, OrderBook, BidOrAsk};

fn main() {
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let mut orderbook = OrderBook::new();  
    orderbook.add_order(4.4, buy_order);
    //println!("{:?}", orderbook);
    
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());
    
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    engine.place_limit_order(pair, 10.000, buy_order).unwrap();
}
```

This is the entry point of the application where:
- An `OrderBook` is created, and a buy order is added.
- A `MatchingEngine` is initialized, and a new market (trading pair) is added.
- A buy order is placed in the matching engine's order book for the specified trading pair.

### matching_engine/orderbook.rs

Defines the `OrderBook` and related structures:

- **BidOrAsk**: Enum to differentiate between bid (buy) and ask (sell) orders.
- **OrderBook**: Struct managing `bids` and `asks` at various price levels.
- **Order**: Struct representing an order with size and type (bid or ask).
- **Limit**: Struct grouping orders at a specific price.
- **Price**: Struct for precise price representation, split into integral and fractional parts.

### matching_engine/engine.rs

Defines the `MatchingEngine` and `TradingPair`:

- **TradingPair**: Struct representing a trading pair (e.g., BTC/USD).
- **MatchingEngine**: Struct managing multiple order books for different trading pairs.
    - `add_new_market`: Adds a new trading pair to the matching engine.
    - `place_limit_order`: Places a limit order in the order book for the specified trading pair.

### matching_engine/mod.rs

Module declaration to include `orderbook` and `engine` modules.

```rust
pub mod orderbook;
pub mod engine;
```

## Example Usage

```rust
fn main() {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    engine.place_limit_order(pair, 10.000, buy_order).unwrap();
}
```

This example initializes the matching engine, adds a new market (BTC/USD), and places a buy order at a specific price.

## Contributing

Feel free to submit pull requests or open issues to improve the project.

---

This README provides an overview of the project, how to get started, and a brief explanation of the main components and their functionalities.
