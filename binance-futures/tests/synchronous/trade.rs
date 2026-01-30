#[cfg(test)]
mod futures_trade_api_integration_test {
    use std::sync::{Arc, OnceLock};

    use binance_common::{
        enums::futures::OrderSide,
        error::BinanceError,
        futures::{
            endpoint::host::Host,
            model::{
                params::{
                    market::Symbol,
                    trade::{
                        CancelOrderParams, GetOpenOrderParams, GetOrderParams, NewOrderParams,
                        PositionRiskV3Params, SetLeverageParams,
                    },
                },
                response::trade::{
                    GetOrderResponse, OrderResponse, PositionRiskV3Response, SetLeverageResponse,
                    TestOrderResponse,
                },
            },
        },
    };
    use binance_core::{
        client::synchronous::Client, signer::hmacsha256::HmacSha256, utility::truncate_to_ticks,
    };
    use binance_futures::synchronous::{market::MarketApi, trade::TradeApi};

    use dotenv::dotenv;
    use once_cell::sync::Lazy;
    use std::env;

    pub static API_KEY_TESTNET: Lazy<String> = Lazy::new(|| {
        dotenv().ok(); // load .env if present (only first call counts)
        env::var("API_KEY_TESTNET").expect("API_KEY_TESTNET must be set")
    });

    pub static SECRET_KEY_TESTNET: Lazy<String> = Lazy::new(|| {
        dotenv().ok();
        env::var("SECRET_KEY_TESTNET").expect("SECRET_KEY_TESTNET must be set")
    });

    static MARKET_CLIENT: OnceLock<Arc<MarketApi<'static, HmacSha256<'static>>>> = OnceLock::new();
    static TRADE_CLIENT: OnceLock<Arc<TradeApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_market() -> Arc<MarketApi<'static, HmacSha256<'static>>> {
        MARKET_CLIENT
            .get_or_init(|| {
                Arc::new(MarketApi::new(Client::new(
                    &Host::Test,
                    HmacSha256::new(&API_KEY_TESTNET, &SECRET_KEY_TESTNET),
                )))
            })
            .clone()
    }

    fn shared_test_trade() -> Arc<TradeApi<'static, HmacSha256<'static>>> {
        TRADE_CLIENT
            .get_or_init(|| {
                Arc::new(TradeApi::new(Client::new(
                    &Host::Test,
                    HmacSha256::new(API_KEY_TESTNET.as_str(), SECRET_KEY_TESTNET.as_str()),
                )))
            })
            .clone()
    }

    #[test]
    fn test_new_limit_test_order() {
        let pair = Symbol::new("BTCUSDT");

        let trade_api = shared_test_trade();
        let market_api = shared_test_market();

        let price = market_api.get_mark_price(&pair).unwrap().mark_price;
        let price = truncate_to_ticks(price + price * 0.1, 1);

        let params: NewOrderParams =
            NewOrderParams::limit(pair.symbol, OrderSide::Sell, price, 0.02);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_err())
    }

    #[test]
    fn test_new_market_test_order() {
        let pair = Symbol::new("ETHUSDT");

        let trade_api = shared_test_trade();

        let params: NewOrderParams = NewOrderParams::market(pair.symbol, OrderSide::Buy, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_err())
    }

    #[test]
    fn test_new_stop_loss_test_order() {
        let pair = Symbol::new("SOLUSDT");

        let trade_api = shared_test_trade();
        let market_api = shared_test_market();

        let price = market_api.get_mark_price(&pair).unwrap().mark_price;

        let stop_price = truncate_to_ticks(price - price * 0.05, 2);
        let price = truncate_to_ticks(price - price * 0.1, 2);

        let params: NewOrderParams =
            NewOrderParams::stop(pair.symbol, OrderSide::Buy, stop_price, price, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_err())
    }

    #[test]
    fn test_new_take_profit_test_order() {
        let pair = Symbol::new("EGLDUSDT");

        let trade_api = shared_test_trade();
        let market_api = shared_test_market();

        let price = market_api.get_mark_price(&pair).unwrap().mark_price;

        let stop_price = truncate_to_ticks(price + price * 0.05, 2);
        let price = truncate_to_ticks(price + price * 0.1, 2);

        let params: NewOrderParams =
            NewOrderParams::take_profit(pair.symbol, OrderSide::Sell, stop_price, price, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_err())
    }

    #[test]
    fn test_new_stop_market_test_order() {
        let pair = Symbol::new("ICPUSDT");

        let trade_api = shared_test_trade();
        let market_api = shared_test_market();

        let price = market_api.get_mark_price(&pair).unwrap().mark_price;

        let stop_price = truncate_to_ticks(price + price * 0.05, 3);

        let params: NewOrderParams =
            NewOrderParams::stop_market(pair.symbol, OrderSide::Buy, stop_price, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_err())
    }

    #[test]
    fn test_take_profit_market_test_order() {
        let pair = Symbol::new("HBARUSDT");

        let trade_api = shared_test_trade();
        let market_api = shared_test_market();

        let price = market_api.get_mark_price(&pair).unwrap().mark_price;
        let stop_price = truncate_to_ticks(price + price * 0.1, 5);

        let params: NewOrderParams =
            NewOrderParams::take_profit_market(pair.symbol, OrderSide::Sell, stop_price, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_err())
    }

    #[test]
    fn test_send_set_leverage() {
        let trade_api = shared_test_trade();

        let params: SetLeverageParams = SetLeverageParams::new("ETHUSDT", 2);

        let eth_leverage: SetLeverageResponse = trade_api.send_set_leverage(&params).unwrap();

        assert_eq!(eth_leverage.leverage, 2);

        let params: SetLeverageParams = SetLeverageParams::new("ETHUSDT", 1);

        let eth_leverage: SetLeverageResponse = trade_api.send_set_leverage(&params).unwrap();

        assert_eq!(eth_leverage.leverage, 1);
    }

    #[test]
    fn test_get_position_risk() {
        let trade_api = shared_test_trade();

        let params: PositionRiskV3Params = PositionRiskV3Params::new("SOLUSDT");

        let positions: Vec<PositionRiskV3Response> =
            trade_api.get_position_risk_v3(&params).unwrap();

        if positions.len() > 0 {
            assert!(positions[0].entry_price > 0.0);
        }
    }

    #[test]
    fn test_send_new_order() {
        let pair = Symbol::new("SUIUSDT");

        let trade_api = shared_test_trade();
        let market_api = shared_test_market();

        let price = market_api.get_mark_price(&pair).unwrap().mark_price;
        let price = truncate_to_ticks(price - price * 0.06, 4);

        let params: NewOrderParams =
            NewOrderParams::limit(pair.symbol, OrderSide::Buy, price, 10.0);
        let new_order = trade_api.send_new_order(&params).unwrap();

        let params2: GetOrderParams =
            GetOrderParams::new(&new_order.symbol).order_id(new_order.order_id);
        let current_order: GetOrderResponse = trade_api.get_order(&params2).unwrap();

        let params3: CancelOrderParams =
            CancelOrderParams::new(&current_order.symbol).order_id(current_order.order_id);

        let canceled_order: Result<OrderResponse, BinanceError> =
            trade_api.send_cancel_order(&params3);

        assert!(canceled_order.is_ok());
    }

    #[test]
    fn test_send_new_order2() {
        let pair = Symbol::new("AVAXUSDT");

        let trade_api = shared_test_trade();
        let market_api = shared_test_market();

        let price = market_api.get_mark_price(&pair).unwrap().mark_price;
        let price = truncate_to_ticks(price + price * 0.07, 3);

        let params: NewOrderParams =
            NewOrderParams::limit(pair.symbol, OrderSide::Sell, price, 20.0);
        let new_order = trade_api.send_new_order(&params).unwrap();

        let params2: GetOpenOrderParams =
            GetOpenOrderParams::new(&new_order.symbol).order_id(new_order.order_id);

        let current_order: GetOrderResponse = trade_api.get_open_order(&params2).unwrap();

        let params3: CancelOrderParams =
            CancelOrderParams::new(&current_order.symbol).order_id(current_order.order_id);

        let canceled_order: Result<OrderResponse, BinanceError> =
            trade_api.send_cancel_order(&params3);

        assert!(canceled_order.is_ok());
    }
}
