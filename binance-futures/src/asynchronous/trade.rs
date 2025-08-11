use binance_common::{
    error::BinanceError,
    futures::{
        endpoint::route::Trade,
        model::{
            params::{
                market::Symbol,
                trade::{
                    CancelAllOrdersParams, CancelOrderParams, GetOpenOrderParams, GetOrderParams,
                    NewOrderParams, PositionRiskV3Params, SetLeverageParams,
                },
            },
            response::trade::{
                CancelAllOrdersResponse, GetOrderResponse, OrderResponse, PositionRiskV3Response,
                SetLeverageResponse, TestOrderResponse,
            },
        },
    },
};
use binance_core::{client::asynchronous::Client, signer::signature::Signature};
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

    pub async fn send_new_test_order(
        &self,
        params: &NewOrderParams<'a>,
    ) -> Result<TestOrderResponse, BinanceError> {
        self.client
            .send(Trade::TestNewOrder, params, Method::POST)
            .await
    }

    pub async fn send_new_order(
        &self,
        params: &NewOrderParams<'a>,
    ) -> Result<OrderResponse, BinanceError> {
        self.client
            .send(Trade::NewOrder, params, Method::POST)
            .await
    }

    pub async fn send_set_leverage(
        &self,
        params: &SetLeverageParams<'a>,
    ) -> Result<SetLeverageResponse, BinanceError> {
        self.client
            .send(Trade::SetLeverage, params, Method::POST)
            .await
    }

    pub async fn send_cancel_order(
        &self,
        params: &CancelOrderParams<'a>,
    ) -> Result<OrderResponse, BinanceError> {
        self.client
            .send(Trade::CancelOrder, params, Method::DELETE)
            .await
    }

    pub async fn send_cancel_all_orders(
        &self,
        params: &CancelAllOrdersParams<'a>,
    ) -> Result<CancelAllOrdersResponse, BinanceError> {
        self.client
            .send(Trade::CancelAllOrders, params, Method::GET)
            .await
    }

    pub async fn get_order(
        &self,
        params: &GetOrderParams<'a>,
    ) -> Result<GetOrderResponse, BinanceError> {
        self.client.send(Trade::GetOrder, params, Method::GET).await
    }

    pub async fn get_open_order(
        &self,
        params: &GetOpenOrderParams<'a>,
    ) -> Result<GetOrderResponse, BinanceError> {
        self.client
            .send(Trade::GetOpenOrder, params, Method::GET)
            .await
    }

    pub async fn get_all_open_orders(
        &self,
        params: &Symbol<'a>,
    ) -> Result<Vec<GetOrderResponse>, BinanceError> {
        self.client
            .send(Trade::OpenOrders, params, Method::GET)
            .await
    }
    pub async fn get_position_risk_v3(
        &self,
        params: &PositionRiskV3Params<'a>,
    ) -> Result<Vec<PositionRiskV3Response>, BinanceError> {
        self.client
            .send(Trade::PositionRiskV3, params, Method::GET)
            .await
    }
}
