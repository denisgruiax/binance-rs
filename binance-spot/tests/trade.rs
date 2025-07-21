#[cfg(test)]
mod spot_trade_api_integration_tests {
    use binance_spot::market::MarketApi;
    use binance_spot::secret::{API_KEY, SECRET_KEY};
    use binance_spot::trade::TradeApi;

    use binance_common::enums::{OrderResponseType, OrderSide};
    use binance_common::spot::endpoint::host::Host;
    use binance_common::spot::model::params::trade::NewOrderParams;
    use binance_core::{client::synchronous::Client, signer::hmacsha256::HmacSha256};
    use std::sync::{Arc, OnceLock};

    static MARKET_CLIENT: OnceLock<Arc<MarketApi<'static, HmacSha256<'static>>>> = OnceLock::new();
    static TRADE_CLIENT: OnceLock<Arc<TradeApi<'static, HmacSha256<'static>>>> = OnceLock::new();
    static SYMBOL: &'static str = "BTCUSDC";

    fn shared_test_trade() -> Arc<TradeApi<'static, HmacSha256<'static>>> {
        TRADE_CLIENT
            .get_or_init(|| {
                Arc::new(TradeApi::new(Client::new(
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
        let trade_api = shared_test_trade();

        let params = NewOrderParams::market(SYMBOL, OrderSide::Buy, 1000.0)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trade_api.send_new_test_order(&params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_post_new_limit_order() {
        let market_api = shared_test_market();
        let trade_api = shared_test_trade();

        let btc_usdc_price = market_api.get_price_ticker(SYMBOL).unwrap().price;
        let limit_price = (btc_usdc_price - (btc_usdc_price * 0.05)).round();

        let params = NewOrderParams::limit(SYMBOL, OrderSide::Buy, limit_price, 1.0)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trade_api.send_new_test_order(&params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_post_new_stop_loss_order() {
        let market_api = shared_test_market();
        let trade_api = shared_test_trade();

        let btc_usdc_price = market_api.get_price_ticker(SYMBOL).unwrap().price;
        let stop_price = (btc_usdc_price - (btc_usdc_price * 0.02)).round();

        let params = NewOrderParams::stop_loss(SYMBOL, OrderSide::Buy, 0.1, stop_price)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trade_api.send_new_test_order(&params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_post_new_take_profit_order() {
        let market_api = shared_test_market();
        let trade_api = shared_test_trade();

        let btc_usdc_price = market_api.get_price_ticker(SYMBOL).unwrap().price;
        let stop_price = (btc_usdc_price + (btc_usdc_price * 0.1)).round();

        let params = NewOrderParams::take_profit(SYMBOL, OrderSide::Sell, 0.1, stop_price)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trade_api.send_new_test_order(&params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_post_new_take_profit_limit_order() {
        let market_api = shared_test_market();
        let trade_api = shared_test_trade();

        let btc_usdc_price = market_api.get_price_ticker(SYMBOL).unwrap().price;
        let stop_price = (btc_usdc_price + (btc_usdc_price * 0.08)).round();
        let sell_price = (btc_usdc_price + (btc_usdc_price * 0.1)).round();

        let params =
            NewOrderParams::take_profit_limit(SYMBOL, OrderSide::Sell, sell_price, 0.1, stop_price)
                .new_order_resp_type(OrderResponseType::Ack);

        let response = trade_api.send_new_test_order(&params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
