pub use crate::spot::model::response::general::{EmptyResponse, ServerTimeResponse};
pub use crate::spot::model::response::market::{Asks, Bids, KlinesResponse};
pub use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformationResponse {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimitResponse>,
    pub symbols: Vec<SymbolResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitResponse {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u16,
    pub limit: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolResponse {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub order_types: Vec<String>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DepthResponse {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    #[serde(rename = "E")]
    pub e: u64,

    #[serde(rename = "T")]
    pub t: u64,

    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
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
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTradesResponse {
    pub id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quote_qty: f64,

    pub time: u64,
    pub is_buyer_maker: bool,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub mark_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub index_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub estimated_settle_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub last_funding_rate: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub interest_rate: f64,

    pub next_funding_time: u64,
    pub time: u64,
}
