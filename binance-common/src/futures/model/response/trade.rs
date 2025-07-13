pub use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::enums::{AdlLevel, PositionSide};
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResponse {
    pub client_order_id: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: u64,
    pub avg_price: String,
    pub orig_qty: String,
    pub price: String,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    pub stop_price: Option<String>,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    pub r#type: String,
    pub orig_type: String,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
    pub price_match: String,
    pub self_trade_prevention_mode: String,
    pub good_till_date: Option<u64>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestNewOrderResponse {
    pub order_id: u64,
    pub symbol: String,
    pub status: String,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_quote: String,
    pub time_in_force: String,
    pub r#type: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub stop_price: String,
    pub price_protect: bool,
    pub orig_type: String,
    pub update_time: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageResponse {
    pub leverage: u8,

    #[serde_as(as = "DisplayFromStr")]
    pub max_notional_value: u64,

    pub symbol: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    pub code: u32,
    pub msg: String,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskV3Response {
    pub symbol: String,
    pub position_side: PositionSide,

    #[serde_as(as = "DisplayFromStr")]
    pub position_amt: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub entry_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub break_even_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub mark_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub un_realized_profit: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub liquidation_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub isolated_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub notional: f64,

    pub margin_asset: String,

    #[serde_as(as = "DisplayFromStr")]
    pub isolated_wallet: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub maint_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub position_initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub open_order_initial_margin: f64,

    pub adl: AdlLevel,

    #[serde_as(as = "DisplayFromStr")]
    pub bid_notional: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub ask_notional: f64,

    pub update_time: u64,
}
