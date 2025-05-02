use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

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
pub struct Bids {
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asks {
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradeResponse {
    pub id: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_qty: f64,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OldTradeLookupResponse {
    pub id: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_qty: f64,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct Kline {
    pub open_time: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub open: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub high: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub low: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub close: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,
    pub close_time: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub quote_asset_volume: f64,

    pub number_of_trades: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub taker_buy_base_asset_volume: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub taker_buy_quote_asset_volume: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub ignore_field: u8
}
