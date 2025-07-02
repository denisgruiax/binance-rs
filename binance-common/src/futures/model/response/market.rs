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

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistoryResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,

    pub funding_time: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub mark_price: f64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub price_change: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub price_change_percent: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub weighted_avg_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub last_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub last_qty: f64,

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
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerPriceResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    pub time: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub bid_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub bid_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub ask_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub ask_qty: f64,

    pub time: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceResponse {
    pub delivery_time: u64,
    pub delivery_price: f64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub open_interest: f64,

    pub symbol: String,
    pub time: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHistoryResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub sum_open_interest: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub sum_open_interest_value: f64,

    pub timestamp: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortPositionRatioResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub long_short_ratio: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub long_account: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub short_account: f64,

    pub timestamp: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortAccountRatioResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub long_short_ratio: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub long_account: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub short_account: f64,

    pub timestamp: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLongShortAccountRatioResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub long_short_ratio: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub long_account: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub short_account: f64,

    pub timestamp: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakerBuySellVolumeResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub buy_sell_ratio: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub buy_vol: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub sell_vol: f64,

    pub timestamp: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasisResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub index_price: f64,

    pub contract_type: String,

    #[serde_as(as = "DisplayFromStr")]
    pub basis_rate: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub futures_price: f64,

    pub annualized_basis_rate: Option<String>,

    #[serde_as(as = "DisplayFromStr")]
    pub basis: f64,

    pub pair: String,

    pub timestamp: u64,
}
