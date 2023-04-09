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
