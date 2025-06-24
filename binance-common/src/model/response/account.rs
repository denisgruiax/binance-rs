use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub maker_commission: u32,
    pub taker_commission: u32,
    pub buyer_commission: u32,
    pub seller_commission: u32,
    pub commission_rates: ComissionRatesResponse,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub prevent_sor: bool,
    pub update_time: u64,
    pub account_type: String,
    pub balances: Vec<AssetResponse>,
    pub permissions: Vec<String>,
    pub uid: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComissionRatesResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub maker: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub taker: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub buyer: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub seller: f64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
    pub asset: String,

    #[serde_as(as = "DisplayFromStr")]
    pub free: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub locked: f64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTradesResponse {
    pub symbol: String,
    pub id: u64,
    pub order_id: u64,
    pub order_list_id: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub quote_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub commission: f64,

    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnfilledOrderCountResponse {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u64,
    pub limit: u64,
    pub count: u64,
}
