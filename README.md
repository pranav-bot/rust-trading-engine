# OrderBook System

## Overview

This project is an implementation of a basic order book for a trading system, written in Rust. An order book is a collection of buy and sell orders organized by price levels. This system allows the addition of buy (bid) orders and sell (ask) orders to the order book and organizes them by price. The order book is capable of handling orders, grouping them into price levels, and maintaining the order at each level.

## Components

### 1. Enums

#### `BidOrAsk`

This enum represents whether an order is a bid (buy) or an ask (sell).

```rust
#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask
}
```

### 2. Structures

#### `OrderBook`

The `OrderBook` struct contains two hash maps: one for asks (sell orders) and one for bids (buy orders). Each hash map uses `Price` as the key and `Limit` as the value.

```rust
#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}
```

##### Methods

- `new() -> OrderBook`: Initializes a new `OrderBook` with empty hash maps for bids and asks.
- `add_order(&mut self, price: f64, order: Order)`: Adds an order to the order book at the specified price. The order is added to the bids if it is a bid, otherwise to the asks.

#### `Price`

The `Price` struct represents a price level in the order book. It separates the integral and fractional parts of the price for precision.

```rust
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}
```

##### Methods

- `new(price: f64) -> Price`: Creates a new `Price` from a floating-point number by separating the integral and fractional parts and using a fixed scalar for precision.

#### `Limit`

The `Limit` struct represents a price level that contains multiple orders.

```rust
#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}
```

##### Methods

- `new(price: Price) -> Limit`: Initializes a new `Limit` at the specified price with an empty vector of orders.
- `add_order(&mut self, order: Order)`: Adds an order to the limit.

#### `Order`

The `Order` struct represents an individual order with a size and whether it is a bid or ask.

```rust
#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}
```

##### Methods

- `new(bid_or_ask: BidOrAsk, size: f64) -> Order`: Creates a new order with the specified type (bid or ask) and size.

## Usage

Here's an example of how to use the order book system:

```rust
fn main() {
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order);
    println!("{:?}", orderbook);
}
```

This code creates a new order book, adds a buy order for 5.5 units at the price of 4.4, and prints the state of the order book.

## Building and Running

To build and run the project, you need to have Rust installed on your machine. Follow these steps:

1. Clone the repository:
   ```sh
   git clone <repository-url>
   cd <repository-directory>
   ```

2. Build the project:
   ```sh
   cargo build
   ```

3. Run the project:
   ```sh
   cargo run
   ```

## Future Enhancements

- **Ask Orders**: Implement the logic to handle ask (sell) orders in the `add_order` method.
- **Order Matching**: Add functionality to match orders and execute trades.
- **Order Removal**: Add methods to remove or update orders.
- **Persistence**: Implement persistent storage for the order book to retain state across sessions.
- **Improved Precision**: Enhance the `Price` struct to handle more complex decimal manipulations.

This README provides an overview and usage guide for the order book system.