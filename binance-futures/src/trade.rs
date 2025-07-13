use binance_common::{
    error::BinanceError,
    futures::{
        endpoint::route::Trade,
        model::{
            params::trade::{CancelAllOrdersParams, NewOrderParams, SetLeverageParams},
            response::trade::{
                CancelAllOrdersResponse, NewOrderResponse, SetLeverageResponse,
                TestNewOrderResponse,
            },
        },
    },
};
use binance_core::{client::synchronous::Client, signer::signature::Signature};
use reqwest::Method;

pub struct TradeApi<'a, S>
where
    S: Signature<'a>,
{
    pub client: Client<'a, S>,
}

impl<'a, S> TradeApi<'a, S>
where
    S: Signature<'a>,
{
    pub fn new(client: Client<'a, S>) -> Self {
        TradeApi { client }
    }

    pub fn send_new_test_order(
        &self,
        params: NewOrderParams,
    ) -> Result<TestNewOrderResponse, BinanceError> {
        self.client.send(Trade::TestNewOrder, params, Method::POST)
    }

    pub fn send_new_order(&self, params: NewOrderParams) -> Result<NewOrderResponse, BinanceError> {
        self.client.send(Trade::NewOrder, params, Method::POST)
    }

    pub fn send_set_leverage(
        &self,
        params: SetLeverageParams,
    ) -> Result<SetLeverageResponse, BinanceError> {
        self.client.send(Trade::SetLeverage, params, Method::POST)
    }

    pub fn send_cancel_all_orders(
        &self,
        params: CancelAllOrdersParams,
    ) -> Result<CancelAllOrdersResponse, BinanceError> {
        self.client
            .send(Trade::CancelAllOrders, params, Method::GET)
    }
}
