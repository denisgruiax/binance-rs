use binance_common::{
    error::BinanceError,
    futures::{
        endpoint::route::Trade,
        model::{
            params::trade::{
                CancelAllOrdersParams, CancelOrderParams, GetOrderParams, NewOrderParams,
                PositionRiskV3Params, SetLeverageParams,
            },
            response::trade::{
                CancelAllOrdersResponse, GetOrderResponse, OrderResponse, PositionRiskV3Response, SetLeverageResponse, TestOrderResponse
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
    ) -> Result<TestOrderResponse, BinanceError> {
        self.client.send(Trade::TestNewOrder, params, Method::POST)
    }

    pub fn send_new_order(&self, params: NewOrderParams) -> Result<OrderResponse, BinanceError> {
        self.client.send(Trade::NewOrder, params, Method::POST)
    }

    pub fn send_set_leverage(
        &self,
        params: SetLeverageParams,
    ) -> Result<SetLeverageResponse, BinanceError> {
        self.client.send(Trade::SetLeverage, params, Method::POST)
    }

    pub fn send_cancel_order(
        &self,
        params: CancelOrderParams,
    ) -> Result<OrderResponse, BinanceError> {
        self.client.send(Trade::CancelOrder, params, Method::DELETE)
    }

    pub fn send_cancel_all_orders(
        &self,
        params: CancelAllOrdersParams,
    ) -> Result<CancelAllOrdersResponse, BinanceError> {
        self.client
            .send(Trade::CancelAllOrders, params, Method::GET)
    }

    pub fn get_order(&self, params: GetOrderParams) -> Result<GetOrderResponse, BinanceError> {
        self.client.send(Trade::GetOrder, params, Method::GET)
    }

    pub fn get_position_risk_v3(
        &self,
        params: PositionRiskV3Params,
    ) -> Result<Vec<PositionRiskV3Response>, BinanceError> {
        self.client.send(Trade::PositionRiskV3, params, Method::GET)
    }
}
