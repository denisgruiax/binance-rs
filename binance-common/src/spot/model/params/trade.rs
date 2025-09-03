use crate::enums::{CancelRestrictions, spot::*};
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
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
    pub fn new(symbol: &'a str, side: OrderSide, r#type: OrderType) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = side;
        self
    }

    pub fn r#type(mut self, r#type: OrderType) -> Self {
        self.r#type = r#type;
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn quote_order_qty(mut self, quote_order_qty: f64) -> Self {
        self.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &'a str) -> Self {
        self.new_client_order_id = Some(new_client_order_id);
        self
    }

    pub fn strategy_id(mut self, strategy_id: u64) -> Self {
        self.strategy_id = Some(strategy_id);
        self
    }

    pub fn strategy_type(mut self, strategy_type: u64) -> Self {
        self.strategy_type = Some(strategy_type);
        self
    }

    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn trailing_delta(mut self, trailing_delta: u64) -> Self {
        self.trailing_delta = Some(trailing_delta);
        self
    }

    pub fn icerberg_qty(mut self, icerberg_qty: f64) -> Self {
        self.icerberg_qty = Some(icerberg_qty);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn self_trade_prevention_mode(mut self, self_trade_prevention_mode: StpModes) -> Self {
        self.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn limit(symbol: &'a str, side: OrderSide, price: f64, quantity: f64) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type: OrderType::Limit,
            price: Some(price),
            quantity: Some(quantity),
            time_in_force: Some(TimeInForce::Gtc),
            ..Default::default()
        }
    }

    pub fn market(symbol: &'a str, side: OrderSide, quote_order_qty: f64) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type: OrderType::Market,
            quote_order_qty: Some(quote_order_qty),
            ..Default::default()
        }
    }

    pub fn stop_loss(symbol: &'a str, side: OrderSide, quantity: f64, stop_price: f64) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type: OrderType::StopLoss,
            quantity: Some(quantity),
            stop_price: Some(stop_price),
            ..Default::default()
        }
    }

    pub fn stop_loss_limit(
        symbol: &'a str,
        side: OrderSide,
        price: f64,
        quantity: f64,
        stop_price: f64,
    ) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type: OrderType::StopLossLimit,
            price: Some(price),
            quantity: Some(quantity),
            stop_price: Some(stop_price),
            time_in_force: Some(TimeInForce::Gtc),
            ..Default::default()
        }
    }

    pub fn take_profit(symbol: &'a str, side: OrderSide, quantity: f64, stop_price: f64) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type: OrderType::TakeProfit,
            quantity: Some(quantity),
            stop_price: Some(stop_price),
            ..Default::default()
        }
    }

    pub fn take_profit_limit(
        symbol: &'a str,
        side: OrderSide,
        price: f64,
        quantity: f64,
        stop_price: f64,
    ) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type: OrderType::TakeProfitLimit,
            price: Some(price),
            quantity: Some(quantity),
            stop_price: Some(stop_price),
            time_in_force: Some(TimeInForce::Gtc),
            ..Default::default()
        }
    }

    pub fn limit_maker(symbol: &'a str, side: OrderSide, price: f64, quantity: f64) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type: OrderType::LimitMaker,
            price: Some(price),
            quantity: Some(quantity),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderParams<'a> {
    pub symbol: &'a str,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<&'a str>,
    pub recv_window: Option<u16>,
}

impl<'a> GetOrderParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        GetOrderParams {
            symbol,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: &'a str) -> Self {
        self.orig_client_order_id = Some(orig_client_order_id);
        self
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParams<'a> {
    pub symbol: &'a str,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<&'a str>,
    pub new_client_order_id: Option<&'a str>,
    pub cancel_restrictions: Option<CancelRestrictions>,
    pub recv_window: Option<u16>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersParms<'a> {
    pub symbol: &'a str,
    pub recv_window: Option<u16>,
}

impl<'a> CancelAllOrdersParms<'a> {
    pub fn new(symbol: &'a str) -> Self {
        CancelAllOrdersParms {
            symbol,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersParams<'a> {
    pub symbol: &'a str,
    pub recv_window: Option<u16>,
}

impl<'a> OpenOrdersParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        OpenOrdersParams {
            symbol,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOrderParams<'a> {
    pub symbol: &'a str,
    pub order_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u16>,
    pub recv_window: Option<u16>,
}

impl<'a> AllOrderParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        AllOrderParams {
            symbol,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}
