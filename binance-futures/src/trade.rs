use binance_common::{
    error::BinanceError,
    futures::{
        endpoint::route::Trade,
        model::{params::trade::NewOrderParams, response::trade::{NewOrderResponse, TestNewOrderResponse}},
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

    pub fn post_new_test_order(
        &self,
        params: NewOrderParams,
    ) -> Result<TestNewOrderResponse, BinanceError> {
        self.client.send(Trade::TestNewOrder, params, Method::POST)
    }

    pub fn post_new_order(&self, params: NewOrderParams) -> Result<NewOrderResponse, BinanceError> {
        self.client.send(Trade::TestNewOrder, params, Method::POST)
    }
}
