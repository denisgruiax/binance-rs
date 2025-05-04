use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthResponse {
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
pub struct TradesResponse {
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
pub struct HistoricalTradesResponse {
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
pub struct KlinesResponse {
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
    pub ignore_field: u8,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvgPriceResponse {
    pub mins: u8,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    pub close_time: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hFullResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub price_change: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub price_change_percent: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub weighted_avg_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub prev_close_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub last_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub last_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub bid_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub bid_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub ask_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub ask_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub open_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub high_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub low_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quote_volume: f64,

    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hMiniResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub open_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub high_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub low_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub last_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quote_volume: f64,

    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerDayFullResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub price_change: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub price_change_percent: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub weighted_avg_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub open_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub high_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub low_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub last_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quote_volume: f64,

    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerDayMiniResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub open_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub high_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub low_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub last_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quote_volume: f64,

    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTickerResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
}
