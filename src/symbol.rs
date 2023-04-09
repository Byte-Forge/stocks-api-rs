#[derive(Clone, Debug)]
pub struct Symbol
{
    pub symbol : String,
    pub score: f64,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub exchange: Option<String>,
}