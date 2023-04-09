use stocks_api::YahooFinanceAPI;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let api = YahooFinanceAPI::new();
    let quote = rt.block_on(api.get_quote("AAPL")).unwrap();
    print!("Current AAPL price: {}", quote.regular_market_price)
}