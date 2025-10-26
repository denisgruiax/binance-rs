pub use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::enums::futures::{
    AdlLevel, OrderSide, OrderStatus, OrderType, PositionSide, StpModes, TimeInForce, WorkingType,
};

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub client_order_id: String,

    #[serde_as(as = "DisplayFromStr")]
    pub cum_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub cum_quote: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub executed_qty: f64,

    pub order_id: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub avg_price: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub orig_qty: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    pub reduce_only: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    pub stop_price: Option<String>,

    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub orig_type: String,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: u64,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub price_match: String,
    pub self_trade_prevention_mode: StpModes,
    pub good_till_date: Option<u64>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestOrderResponse {
    pub order_id: u64,
    pub symbol: String,
    pub status: OrderStatus,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_quote: String,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: OrderSide,
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

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub avg_price: f64,

    pub client_order_id: String,

    #[serde_as(as = "DisplayFromStr")]
    pub cum_quote: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub executed_qty: f64,

    pub order_id: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub orig_qty: f64,

    pub orig_type: String,

    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    pub reduce_only: bool,
    pub side: OrderSide,
    pub position_side: String,
    pub status: OrderStatus,

    #[serde_as(as = "DisplayFromStr")]
    pub stop_price: f64,

    pub close_position: bool,
    pub symbol: String,
    pub time: u64,
    pub time_in_force: TimeInForce,

    #[serde(rename = "type")]
    pub order_type: String,

    pub activate_price: Option<String>,
    pub price_rate: Option<f64>,

    pub update_time: u64,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub price_match: String,
    pub self_trade_prevention_mode: StpModes,
    pub good_till_date: u64,
}
