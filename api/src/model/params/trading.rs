use super::binance::*;
use serde::Serialize;

#[derive(Default, Serialize)]
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

impl<'a> NewOrderParams<'a> {
    fn limit(
        symbol: &'a str,
        side: OrderSide,
        r#type: OrderType,
        price: f64,
        quantity: f64,
        time_in_force: TimeInForce,
    ) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type,
            price: Some(price),
            quantity: Some(quantity),
            time_in_force: Some(time_in_force),
            ..Default::default()
        }
    }

    fn market(
        symbol: &'a str,
        side: OrderSide,
        r#type: OrderType,
        quote_order_qty: f64,
    ) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type,
            quote_order_qty: Some(quote_order_qty),
            ..Default::default()
        }
    }
}
