use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: u32,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bids{
    pub price: f64,
    pub quantity: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asks{
    pub price: f64,
    pub quantity: f64,
}