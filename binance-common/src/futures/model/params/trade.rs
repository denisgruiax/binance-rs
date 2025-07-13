use serde::Serialize;

use crate::enums::{
    OrderResponseType, OrderSide, OrderType, PositionSide, PriceMatch, StpModes, TimeInForce,
    WorkingType,
};

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderParams<'a> {
    pub symbol: &'a str,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    pub r#type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub reduce_only: Option<&'a str>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<&'a str>,
    pub stop_price: Option<f64>,
    pub close_position: Option<&'a str>,
    pub activation_price: Option<f64>,
    pub callback_rate: Option<f64>,
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<&'a str>,
    pub new_order_resp_type: Option<OrderResponseType>,
    pub price_match: Option<PriceMatch>,
    pub self_trade_prevention_mode: Option<StpModes>,
    pub good_till_date: Option<u64>,
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

    pub fn position_side(mut self, position_side: PositionSide) -> Self {
        self.position_side = Some(position_side);
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

    pub fn reduce_only(mut self, reduce_only: &'a str) -> Self {
        self.reduce_only = Some(reduce_only);
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

    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn close_position(mut self, close_position: &'a str) -> Self {
        self.close_position = Some(close_position);
        self
    }

    pub fn activation_price(mut self, activation_price: f64) -> Self {
        self.activation_price = Some(activation_price);
        self
    }

    pub fn callback_rate(mut self, callback_rate: f64) -> Self {
        self.callback_rate = Some(callback_rate);
        self
    }

    pub fn working_type(mut self, working_type: WorkingType) -> Self {
        self.working_type = Some(working_type);
        self
    }

    pub fn price_protect(mut self, price_protect: &'a str) -> Self {
        self.price_protect = Some(price_protect);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn price_match(mut self, price_match: PriceMatch) -> Self {
        self.price_match = Some(price_match);
        self
    }

    pub fn self_trade_prevention_mode(mut self, self_trade_prevention_mode: StpModes) -> Self {
        self.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }

    pub fn good_till_date(mut self, good_till_date: u64) -> Self {
        self.good_till_date = Some(good_till_date);
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

    pub fn market(symbol: &'a str, side: OrderSide, quantity: f64) -> Self {
        NewOrderParams {
            symbol,
            side,
            r#type: OrderType::Market,
            quantity: Some(quantity),
            ..Default::default()
        }
    }

    pub fn stop(
        symbol: &'a str,
        side: OrderSide,
        stop_price: f64,
        price: f64,
        quantity: f64,
    ) -> Self {
        Self {
            symbol,
            side,
            r#type: OrderType::Stop,
            price: Some(price),
            stop_price: Some(stop_price),
            quantity: Some(quantity),
            time_in_force: Some(TimeInForce::Gtc),
            ..Default::default()
        }
    }

    pub fn take_profit(
        symbol: &'a str,
        side: OrderSide,
        stop_price: f64,
        price: f64,
        quantity: f64,
    ) -> Self {
        Self {
            symbol,
            side,
            r#type: OrderType::TakeProfit,
            price: Some(price),
            stop_price: Some(stop_price),
            quantity: Some(quantity),
            time_in_force: Some(TimeInForce::Gtc),
            ..Default::default()
        }
    }

    pub fn stop_market(symbol: &'a str, side: OrderSide, stop_price: f64, quantity: f64) -> Self {
        Self {
            symbol,
            side,
            r#type: OrderType::StopMarket,
            stop_price: Some(stop_price),
            quantity: Some(quantity),
            ..Default::default()
        }
    }

    pub fn take_profit_market(
        symbol: &'a str,
        side: OrderSide,
        stop_price: f64,
        quantity: f64,
    ) -> Self {
        Self {
            symbol,
            side,
            r#type: OrderType::TakeProfitMarket,
            stop_price: Some(stop_price),
            quantity: Some(quantity),
            ..Default::default()
        }
    }

    pub fn trailing_stop_market(
        symbol: &'a str,
        side: OrderSide,
        callback_rate: f64,
        activation_price: Option<f64>,
    ) -> Self {
        Self {
            symbol,
            side,
            r#type: OrderType::TrailingStopMarket,
            callback_rate: Some(callback_rate),
            activation_price: activation_price,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageParams<'a> {
    pub symbol: &'a str,
    pub leverage: u8,
    pub recv_window: Option<u16>,
}

impl<'a> SetLeverageParams<'a> {
    pub fn new(symbol: &'a str, leverage: u8) -> Self {
        SetLeverageParams {
            symbol,
            leverage,
            recv_window: None,
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn leverage(mut self, leverage: u8) -> Self {
        self.leverage = leverage;
        self
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersParams<'a> {
    pub symbol: &'a str,
    pub recv_window: Option<u16>,
}

impl<'a> CancelAllOrdersParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        CancelAllOrdersParams {
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
pub struct PositionRiskV3Params<'a> {
    pub symbol: &'a str,
    pub recv_window: Option<u16>,
}

impl<'a> PositionRiskV3Params<'a> {
    pub fn new(symbol: &'a str) -> Self {
        PositionRiskV3Params {
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
pub struct CancelOrderParams<'a> {
    pub symbol: &'a str,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<&'a str>,
    pub recv_window: Option<u16>,
}

impl<'a> CancelOrderParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        CancelOrderParams {
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
