use std::{error::Error, vec};

use reqwest::Client;
use serde::Deserialize;

use super::quote::Quote;
use super::symbol::Symbol;

#[derive(Debug, Deserialize)]
struct YahooFinanceQuoteResponse {
    #[serde(rename = "quoteResponse")]
    quote_response: QuoteResponse,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    result: Vec<QuoteResult>,
}

#[derive(Debug, Deserialize)]
struct QuoteResult {
    symbol: String,
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: f64,
}

#[derive(Debug, Deserialize)]
struct YahooFinanceSymbolSearchResponse {
    #[serde(rename = "quotes")]
    quotes: Vec<SymbolSearchResult>,
}

#[derive(Debug, Deserialize)]
struct SymbolSearchResult {
    symbol: String,
    shortname: Option<String>,
    longname: Option<String>,
    sector: Option<String>,
    industry: Option<String>,
    score: f64,
    exchange: Option<String>,
    #[serde(rename = "exchDisp")]
    exch_disp: Option<String>,
}

pub struct YahooFinanceAPI {
    client: Client,
}

impl YahooFinanceAPI {
    pub fn new() -> YahooFinanceAPI {
        YahooFinanceAPI {
            client: Client::new(),
        }
    }

    pub async fn get_quote(&self, symbol: &str) -> Result<Quote, Box<dyn Error>> {
        let quotes = self.get_quotes(vec![symbol]).await?;
        assert_eq!(quotes.len(), 1);
        Ok(quotes[0].clone())
    }

    pub async fn get_quotes(&self, symbols: Vec<&str>) -> Result<Vec<Quote>, Box<dyn Error>> {
        let url = format!(
            "https://query1.finance.yahoo.com/v7/finance/quote?symbols={}",
            symbols.join(",")
        );
        let res = self.client.get(&url).send().await?;
        let quote_response = res.json::<YahooFinanceQuoteResponse>().await?;

        let quote_results = &quote_response.quote_response.result;
        let quotes = quote_results
            .into_iter()
            .map(|quote_result| Quote {
                symbol: quote_result.symbol.clone(),
                regular_market_price: quote_result.regular_market_price,
            })
            .collect();

        Ok(quotes)
    }

    pub async fn search_symbols(&self, query: &str) -> Result<Vec<Symbol>, Box<dyn Error>> {
        let url = format!(
            "https://query1.finance.yahoo.com/v1/finance/search?q={}&quotesCount=10&newsCount=0",
            query
        );
        let res = self.client.get(&url).send().await?;
        let search_response = res.json::<YahooFinanceSymbolSearchResponse>().await?;

        let symbol_results = search_response.quotes;
        let symbols = symbol_results
            .into_iter()
            .map(|symbol_result| Symbol {
                symbol: symbol_result.symbol.clone(),
                shortname: symbol_result.shortname,
                longname: symbol_result.longname,
                exchange: symbol_result.exchange,
                sector: symbol_result.sector,
                industry: symbol_result.industry,
                score: symbol_result.score,
            })
            .collect();

        Ok(symbols)
    }
}

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
        let quotes = rt.block_on(api.get_quotes(vec!["AAPL", "MSFT"])).unwrap();
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
