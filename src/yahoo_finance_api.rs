use std::{error::Error, vec};

use reqwest::Client;
use serde::Deserialize;

use super::quote::Quote;

#[derive(Debug, Deserialize)]
struct YahooFinanceQuoteResponse  {
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
struct SymbolSearchResponse {
    result: Vec<SymbolSearchResult>,
}

#[derive(Debug, Deserialize)]
struct SymbolSearchResult {
    symbol: String,
    shortname: String,
    longname: String,
    sector: Option<String>,
    industry: Option<String>,
    score: f64,
    exchange: Option<String>,
    #[serde(rename="exchDisp")]
    exch_disp: Option<String>,
}

pub struct YahooFinanceAPI {
    client : Client,
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
        let body = res.text().await?;

        let quote_response = serde_json::from_str::<YahooFinanceQuoteResponse >(&body)?;

        let quote_results = &quote_response.quote_response.result;
        let quotes = quote_results.into_iter().map(|quote_result| Quote {
            symbol: quote_result.symbol.clone(),
            regular_market_price: quote_result.regular_market_price,
        }).collect();

        Ok(quotes)
    }

    pub async fn search_symbols(&self, query: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let url = format!(
            "https://query1.finance.yahoo.com/v1/finance/search?q={}&quotesCount=10&newsCount=0",
            query
        );
        let res = self.client.get(&url).send().await?;
        let body = res.text().await?;

        let search_response = serde_json::from_str::<YahooFinanceSymbolSearchResponse>(&body)?;

        let symbol_results = search_response.quotes;
        let symbols = symbol_results.into_iter().map(|result| result.symbol).collect();

        Ok(symbols)
    }
}
