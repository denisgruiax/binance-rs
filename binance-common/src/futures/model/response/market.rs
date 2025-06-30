pub use crate::spot::model::response::general::{EmptyResponse, ServerTimeResponse};
pub use crate::spot::model::response::market::{Asks, Bids};
pub use serde::{Deserialize, Serialize};
use serde_with::serde_as;
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
#[serde(rename_all = "camelCase")]
pub struct DepthResponse {
    pub last_update_id: u64,

    #[serde(rename = "E")]
    pub e: u64,

    #[serde(rename = "T")]
    pub t: u64,

    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}
