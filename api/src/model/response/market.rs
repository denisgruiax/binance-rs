use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: u32,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bids{
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asks{
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,
}