#[derive(Clone, Debug)]
pub struct QuoteIndicator {
    pub volume: Vec<Option<u64>>,
    pub low: Vec<Option<f64>>,
    pub high: Vec<Option<f64>>,
    pub open: Vec<Option<f64>>,
    pub close: Vec<Option<f64>>,
}

#[derive(Clone, Debug)]
pub struct Chart {
    pub timestamps: Vec<u64>,
    pub indicators: Vec<QuoteIndicator>,
}