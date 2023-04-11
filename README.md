# stocks_api

[![CI](https://github.com/Byte-Forge/stocks-api-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Byte-Forge/stocks-api-rs/actions/workflows/ci.yml)

Retrieve finance data using YahooFinanceAPI

## Examples

### Get a single quote
```rust
use stocks_api::YahooFinanceAPI;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let api = YahooFinanceAPI::new();
    let quote = rt.block_on(api.get_quote("AAPL")).unwrap();
    print!("Current AAPL price: {}", quote.regular_market_price)
}
```

### Search for symbols
```rust
use stocks_api::YahooFinanceAPI;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let api = YahooFinanceAPI::new();
    let symbols = rt.block_on(api.search_symbols("Microsoft")).unwrap();
    println!("Search results for: Microsoft");
    println!(
        "{}",
        symbols
            .into_iter()
            .map(|symbol| symbol.symbol)
            .collect::<Vec<String>>()
            .join(",")
    );
}
``` 
