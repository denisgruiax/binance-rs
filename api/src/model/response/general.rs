use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct Empty {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    pub server_time: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformationResponse {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimitResponse>,
    pub symbols: Vec<SymbolResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitResponse {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u16,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SymbolResponse {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    //pub filters: Vec<Filters>,
}
