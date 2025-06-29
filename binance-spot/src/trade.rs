use binance_common::enums::OrderResponseType;
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

use binance_core::{client::synchronous::Client, signer::signature::Signature};

pub struct TradingApi<'a, S>
where
    S: Signature<'a>,
{
    client: Client<'a, S>,
}

impl<'a, S> TradingApi<'a, S>
where
    S: Signature<'a>,
{
    pub fn new(client: Client<'a, S>) -> Self {
        TradingApi { client }
    }

    pub fn post_new_order(&self, params: NewOrderParams) -> Result<NewOrderResponse, BinanceError> {
        if let Some(order_response_type) = &params.new_order_resp_type {
            match order_response_type {
                OrderResponseType::Ack => {
                    return Ok(NewOrderResponse::Ack(self.client.send::<AckResponse>(
                        Trade::NewOrder,
                        params,
                        Method::POST,
                    )?));
                }
                OrderResponseType::Result => {
                    return Ok(NewOrderResponse::Result(
                        self.client.send::<ResultResponse>(
                            Trade::NewOrder,
                            params,
                            Method::POST,
                        )?,
                    ));
                }
                OrderResponseType::Full => {
                    return Ok(NewOrderResponse::Full(self.client.send::<FullResponse>(
                        Trade::NewOrder,
                        params,
                        Method::POST,
                    )?));
                }
            }
        }

        Ok(NewOrderResponse::Ack(self.client.send::<AckResponse>(
            Trade::NewOrder,
            params,
            Method::POST,
        )?))
    }

    pub fn post_new_test_order(
        &self,
        params: NewOrderParams,
    ) -> Result<serde_json::Value, BinanceError> {
        self.client.send(Trade::TestOrder, params, Method::POST)
    }

    pub fn get_order(&self, params: GetOrderParams) -> Result<OrderIdResponse, BinanceError> {
        self.client.send(Trade::GetOrder, params, Method::GET)
    }

    pub fn cancel_order(
        &self,
        params: CancelOrderParams,
    ) -> Result<CancelOrderResponse, BinanceError> {
        self.client.send(Trade::CancelOrder, params, Method::DELETE)
    }

    pub fn cancel_open_orders(
        &self,
        params: CancelAllOrdersParms,
    ) -> Result<Vec<CancelOrderResponse>, BinanceError> {
        self.client
            .send(Trade::CancelAllOrders, params, Method::DELETE)
    }

    pub fn get_open_orders(
        &self,
        params: OpenOrdersParams,
    ) -> Result<Vec<OrderResponse>, BinanceError> {
        self.client.send(Trade::OpenOrders, params, Method::GET)
    }

    pub fn get_all_order(
        &self,
        params: AllOrderParams,
    ) -> Result<Vec<OrderResponse>, BinanceError> {
        self.client.send(Trade::AllOrders, params, Method::GET)
    }
}

#[cfg(test)]
mod trading_api {
    use super::TradingApi;
    use crate::market::MarketApi;
    use crate::secret::{API_KEY, SECRET_KEY};

    use binance_common::enums::{OrderResponseType, OrderSide};
    use binance_common::spot::endpoint::host::Host;
    use binance_common::spot::model::params::trade::NewOrderParams;
    use binance_core::{client::synchronous::Client, signer::hmacsha256::HmacSha256};
    use std::sync::{Arc, OnceLock};

    static MARKET_CLIENT: OnceLock<Arc<MarketApi<'static, HmacSha256<'static>>>> = OnceLock::new();
    static TRADING_CLIENT: OnceLock<Arc<TradingApi<'static, HmacSha256<'static>>>> =
        OnceLock::new();
    static SYMBOL: &'static str = "BTCUSDC";

    fn shared_test_trading() -> Arc<TradingApi<'static, HmacSha256<'static>>> {
        TRADING_CLIENT
            .get_or_init(|| {
                Arc::new(TradingApi::new(Client::new(
                    &Host::Api,
                    HmacSha256::new(API_KEY, SECRET_KEY),
                )))
            })
            .clone()
    }

    fn shared_test_market() -> Arc<MarketApi<'static, HmacSha256<'static>>> {
        MARKET_CLIENT
            .get_or_init(|| {
                Arc::new(MarketApi::new(Client::new(
                    &Host::Api,
                    HmacSha256::new(API_KEY, SECRET_KEY),
                )))
            })
            .clone()
    }

    #[test]
    fn test_post_new_market_order() {
        let trading_api = shared_test_trading();

        let params = NewOrderParams::market(SYMBOL, OrderSide::Buy, 1000.0)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trading_api.post_new_test_order(params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_post_new_limit_order() {
        let market_api = shared_test_market();
        let trading_api = shared_test_trading();

        let btc_usdc_price = market_api.get_price_ticker(SYMBOL).unwrap().price;
        let limit_price = (btc_usdc_price - (btc_usdc_price * 0.05)).round();

        let params = NewOrderParams::limit(SYMBOL, OrderSide::Buy, limit_price, 1.0)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trading_api.post_new_test_order(params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_post_new_stop_loss_order() {
        let market_api = shared_test_market();
        let trading_api = shared_test_trading();

        let btc_usdc_price = market_api.get_price_ticker(SYMBOL).unwrap().price;
        let stop_price = (btc_usdc_price - (btc_usdc_price * 0.02)).round();

        let params = NewOrderParams::stop_loss(SYMBOL, OrderSide::Buy, 0.1, stop_price)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trading_api.post_new_test_order(params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_post_new_take_profit_order() {
        let market_api = shared_test_market();
        let trading_api = shared_test_trading();

        let btc_usdc_price = market_api.get_price_ticker(SYMBOL).unwrap().price;
        let stop_price = (btc_usdc_price + (btc_usdc_price * 0.1)).round();

        let params = NewOrderParams::take_profit(SYMBOL, OrderSide::Sell, 0.1, stop_price)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trading_api.post_new_test_order(params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_post_new_take_profit_limit_order() {
        let market_api = shared_test_market();
        let trading_api = shared_test_trading();

        let btc_usdc_price = market_api.get_price_ticker(SYMBOL).unwrap().price;
        let stop_price = (btc_usdc_price + (btc_usdc_price * 0.08)).round();
        let sell_price = (btc_usdc_price + (btc_usdc_price * 0.1)).round();

        let params =
            NewOrderParams::take_profit_limit(SYMBOL, OrderSide::Sell, sell_price, 0.1, stop_price)
                .new_order_resp_type(OrderResponseType::Ack);

        let response = trading_api.post_new_test_order(params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
