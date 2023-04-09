// src/lib.rs
mod quote;
mod yahoo_finance_api;

pub use quote::Quote;
pub use yahoo_finance_api::YahooFinanceAPI;