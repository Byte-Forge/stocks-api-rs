use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub regular_market_price: f64,
}