#[derive(Clone, Debug)]
pub struct Quote {
    pub symbol: String,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub regular_market_price: f64,
    pub regular_market_change: f64,
    pub currency : Option<String>,
    pub market_state : Option<String>,
}