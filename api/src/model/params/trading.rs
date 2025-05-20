use super::binance::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderParams<'a> {
    pub symbol: &'a str,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<&'a str>,
    pub strategy_id: Option<u64>,
    pub strategy_type: Option<u64>,
    pub stop_price: Option<f64>,
    pub trailing_delta: Option<u64>,
    pub icerberg_qty: Option<f64>,
    pub new_order_resp_type: Option<OrderResponseType>,
    pub self_trade_prevention_mode: Option<StpModes>,
    pub recv_window: Option<u16>,
}
