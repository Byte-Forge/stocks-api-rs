// src/lib.rs
mod quote;
mod yahoo_finance_api;

pub use quote::Quote;
pub use yahoo_finance_api::YahooFinanceAPI;


#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use super::*;

    #[test]
    fn test_get_quote() {
        let rt = Runtime::new().unwrap();
        let api = YahooFinanceAPI::new();
        let quote = rt.block_on(api.get_quote("AAPL")).unwrap();
        assert_eq!(quote.symbol, "AAPL");
    }

    #[test]
    fn test_get_quotes() {
        let rt = Runtime::new().unwrap();
        let api = YahooFinanceAPI::new();
        let quotes = rt.block_on(api.get_quotes(vec!["AAPL","MSFT"])).unwrap();
        assert_eq!(quotes.len(), 2);
    }

    #[test]
    fn test_search_symbols() {
        let rt = Runtime::new().unwrap();
        let api = YahooFinanceAPI::new();
        let symbols = rt.block_on(api.search_symbols("Microsoft")).unwrap();
        assert!(symbols.len() > 0);
    }
}
