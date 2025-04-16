use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BinanceError{
    pub code: i16,
    pub msg: String,
}