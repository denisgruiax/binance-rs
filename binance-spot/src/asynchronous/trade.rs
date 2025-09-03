use binance_common::enums::spot::OrderResponseType;
use binance_common::error::BinanceError;
use binance_common::spot::{
    endpoint::route::Trade,
    model::{
        params::trade::{
            AllOrderParams, CancelAllOrdersParms, CancelOrderParams, GetOrderParams,
            NewOrderParams, OpenOrdersParams,
        },
        response::trade::{
            AckResponse, CancelOrderResponse, FullResponse, NewOrderResponse, OrderIdResponse,
            OrderResponse, ResultResponse,
        },
    },
};
use reqwest::Method;

use binance_core::{client::asynchronous::Client, signer::signature::Signature};

pub struct TradeApi<'a, S>
where
    S: Signature<'a>,
{
    client: Client<'a, S>,
}

impl<'a, S> TradeApi<'a, S>
where
    S: Signature<'a>,
{
    pub async fn new(client: Client<'a, S>) -> Self {
        TradeApi { client }
    }

    pub async fn send_new_order(
        &self,
        params: &NewOrderParams<'a>,
    ) -> Result<NewOrderResponse, BinanceError> {
        if let Some(order_response_type) = &params.new_order_resp_type {
            match order_response_type {
                OrderResponseType::Ack => {
                    return Ok(NewOrderResponse::Ack(
                        self.client
                            .send::<AckResponse>(Trade::NewOrder, params, Method::POST)
                            .await?,
                    ));
                }
                OrderResponseType::Result => {
                    return Ok(NewOrderResponse::Result(
                        self.client
                            .send::<ResultResponse>(Trade::NewOrder, params, Method::POST)
                            .await?,
                    ));
                }
                OrderResponseType::Full => {
                    return Ok(NewOrderResponse::Full(
                        self.client
                            .send::<FullResponse>(Trade::NewOrder, params, Method::POST)
                            .await?,
                    ));
                }
            }
        }

        Ok(NewOrderResponse::Ack(
            self.client
                .send::<AckResponse>(Trade::NewOrder, params, Method::POST)
                .await?,
        ))
    }

    pub async fn send_new_test_order(
        &self,
        params: &NewOrderParams<'a>,
    ) -> Result<serde_json::Value, BinanceError> {
        self.client
            .send(Trade::TestOrder, params, Method::POST)
            .await
    }

    pub async fn get_order(
        &self,
        params: &GetOrderParams<'a>,
    ) -> Result<OrderIdResponse, BinanceError> {
        self.client.send(Trade::GetOrder, params, Method::GET).await
    }

    pub async fn send_cancel_order(
        &self,
        params: &CancelOrderParams<'a>,
    ) -> Result<CancelOrderResponse, BinanceError> {
        self.client
            .send(Trade::CancelOrder, params, Method::DELETE)
            .await
    }

    pub async fn send_cancel_open_orders(
        &self,
        params: &CancelAllOrdersParms<'a>,
    ) -> Result<Vec<CancelOrderResponse>, BinanceError> {
        self.client
            .send(Trade::CancelAllOrders, params, Method::DELETE)
            .await
    }

    pub async fn get_open_orders(
        &self,
        params: &OpenOrdersParams<'a>,
    ) -> Result<Vec<OrderResponse>, BinanceError> {
        self.client
            .send(Trade::OpenOrders, params, Method::GET)
            .await
    }

    pub async fn get_all_order(
        &self,
        params: &AllOrderParams<'a>,
    ) -> Result<Vec<OrderResponse>, BinanceError> {
        self.client
            .send(Trade::AllOrders, params, Method::GET)
            .await
    }
}
