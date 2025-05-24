use binance_api::{
    endpoint::route::Trading,
    model::{
        BinanceError,
        params::{
            binance::OrderResponseType,
            trading::{CancelAllOrdersParms, GetOrderParams, NewOrderParams},
        },
        response::trading::{
            AckResponse, CancelOrderResponse, FullResponse, OrderIdResponse, OrderResponse,
            ResultResponse,
        },
    },
};

use crate::client::{signer::signature::Signature, synchronous::Client};

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

    pub fn post_new_order(&self, params: NewOrderParams) -> Result<OrderResponse, BinanceError> {
        if let Some(order_response_type) = &params.new_order_resp_type {
            match order_response_type {
                OrderResponseType::Ack => {
                    return Ok(OrderResponse::Ack(
                        self.client.post::<AckResponse>(Trading::NewOrder, params)?,
                    ));
                }
                OrderResponseType::Result => {
                    return Ok(OrderResponse::Result(
                        self.client
                            .post::<ResultResponse>(Trading::NewOrder, params)?,
                    ));
                }
                OrderResponseType::Full => {
                    return Ok(OrderResponse::Full(
                        self.client
                            .post::<FullResponse>(Trading::NewOrder, params)?,
                    ));
                }
            }
        }

        Ok(OrderResponse::Ack(
            self.client.post::<AckResponse>(Trading::NewOrder, params)?,
        ))
    }

    pub fn post_new_test_order(
        &self,
        params: NewOrderParams,
    ) -> Result<serde_json::Value, BinanceError> {
        self.client.post(Trading::TestOrder, params)
    }

    pub fn get_order(&self, params: GetOrderParams) -> Result<OrderIdResponse, BinanceError> {
        self.client.get_signed(Trading::GetOrder, params)
    }

    pub fn cancel_open_orders(
        &self,
        params: CancelAllOrdersParms,
    ) -> Result<Vec<CancelOrderResponse>, BinanceError> {
        self.client.delete(Trading::CancelAllOrders, params)
    }
}

#[cfg(test)]
mod trading_api {
    use crate::{
        client::{signer::hmacsha256::HmacSha256, synchronous::Client},
        spot::market::MarketApi,
    };

    use super::TradingApi;
    use crate::spot::secret::{API_KEY, SECRET_KEY};
    use binance_api::{
        endpoint::host::Host,
        model::params::{
            binance::{OrderResponseType, OrderSide},
            trading::NewOrderParams,
        },
    };
    use std::sync::{Arc, OnceLock};

    static MARKET_CLIENT: OnceLock<Arc<MarketApi<'static, HmacSha256<'static>>>> = OnceLock::new();
    static TRADING_CLIENT: OnceLock<Arc<TradingApi<'static, HmacSha256<'static>>>> =
        OnceLock::new();
    static SYMBOL: &'static str = "BTCUSDC";

    fn shared_test_trading() -> Arc<TradingApi<'static, HmacSha256<'static>>> {
        TRADING_CLIENT
            .get_or_init(|| {
                Arc::new(TradingApi::new(Client::new(
                    Host::Api.as_ref(),
                    HmacSha256::new(API_KEY, SECRET_KEY),
                )))
            })
            .clone()
    }

    fn shared_test_market() -> Arc<MarketApi<'static, HmacSha256<'static>>> {
        MARKET_CLIENT
            .get_or_init(|| {
                Arc::new(MarketApi::new(Client::new(
                    Host::Api.as_ref(),
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
