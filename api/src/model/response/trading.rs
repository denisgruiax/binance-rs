use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AckResponse {
    symbol: String,
    order_id: u64,
    order_list_id: i64,
    client_order_id: String,
    transact_time: u64,
}

#[derive(Clone, Debug)]
pub enum OrderResponse {
    Ack(AckResponse),
    Result(ResultResponse),
    Full(FullResponse),
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

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullResponse {
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
    pub fills: Vec<Fill>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub commission: f64,

    pub commission_asset: String,
    pub trade_id: u64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderIdResponse {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,

    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub orig_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub executed_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub cummulative_quote_qty: f64,

    pub status: String,
    pub time_in_force: String,

    #[serde(rename = "type")]
    pub order_type: String,

    pub side: String,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub working_time: u64,
    // #[serde_as(as = "DisplayFromStr")]
    // #[serde(default)]
    // pub stop_price: Option<f64>,

    // #[serde_as(as = "DisplayFromStr")]
    // #[serde(default)]
    // pub iceberg_qty: Option<f64>,

    // #[serde_as(as = "DisplayFromStr")]
    // #[serde(default)]
    // pub orig_quote_order_qty: Option<f64>,

    // #[serde(default)]
    // pub prevented_match_id: Option<u64>,

    // #[serde_as(as = "DisplayFromStr")]
    // #[serde(default)]
    // pub prevented_quantity: Option<f64>,

    // #[serde(default)]
    // pub strategy_id: Option<u64>,

    // #[serde(default)]
    // pub strategy_type: Option<u64>,

    // #[serde(default)]
    // pub trailing_delta: Option<u64>,

    // #[serde(default)]
    // pub trailing_time: Option<i64>,

    // #[serde(default)]
    // pub used_sor: Option<bool>,

    // #[serde(default)]
    // pub working_floor: Option<String>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub symbol: String,
    pub orig_client_order_id: String,
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
    pub cummulative_quote_qty: f64,

    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,
    pub self_trade_prevention_mode: String,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrderResponse {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,

    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub orig_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub executed_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub cummulative_quote_qty: f64,

    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,

    #[serde_as(as = "DisplayFromStr")]
    pub stop_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub iceberg_qty: f64,

    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,

    #[serde_as(as = "DisplayFromStr")]
    pub orig_quote_order_qty: f64,

    pub working_time: u64,
    pub self_trade_prevention_mode: String,
}