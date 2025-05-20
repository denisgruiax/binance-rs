use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AckResponse {
    symbol: String,
    order_id: u64,
    order_list_id: i64,
    client_order_id: String,
    transact_time: u64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultResponse {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub orig_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub executed_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub orig_quote_order_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub cummulative_quote_qty: f64,

    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,
    pub working_time: u64,
    pub self_trade_prevention_mode: String,
}