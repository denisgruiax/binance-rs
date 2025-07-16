#[cfg(test)]
mod futures_trade_api_integration_test {
    use std::sync::{Arc, OnceLock};

    use binance_common::{
        enums::OrderSide,
        error::BinanceError,
        futures::{
            endpoint::host::Host,
            model::{
                params::trade::{
                    CancelOrderParams, GetOpenOrderParams, GetOrderParams, NewOrderParams,
                    PositionRiskV3Params, SetLeverageParams,
                },
                response::trade::{
                    GetOrderResponse, OrderResponse, PositionRiskV3Response, SetLeverageResponse,
                    TestOrderResponse,
                },
            },
        },
    };
    use binance_core::{client::synchronous::Client, signer::hmacsha256::HmacSha256};
    use binance_futures::{
        secret::{API_KEY_TESTNET, SECRET_KEY_TESTNET},
        trade::TradeApi,
    };

    static TRADE_CLIENT: OnceLock<Arc<TradeApi<'static, HmacSha256<'static>>>> = OnceLock::new();
    static SYMBOL: &'static str = "SOLUSDT";

    fn shared_test_trade() -> Arc<TradeApi<'static, HmacSha256<'static>>> {
        TRADE_CLIENT
            .get_or_init(|| {
                Arc::new(TradeApi::new(Client::new(
                    &Host::Test,
                    HmacSha256::new(API_KEY_TESTNET, SECRET_KEY_TESTNET),
                )))
            })
            .clone()
    }

    #[test]
    fn test_new_limit_test_order() {
        let trade_api = shared_test_trade();

        let params: NewOrderParams = NewOrderParams::limit(SYMBOL, OrderSide::Sell, 300.0, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_ok())
    }

    #[test]
    fn test_new_market_test_order() {
        let trade_api = shared_test_trade();

        let params: NewOrderParams = NewOrderParams::market(SYMBOL, OrderSide::Buy, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_ok())
    }

    #[test]
    fn test_new_stop_loss_test_order() {
        let trade_api = shared_test_trade();

        let params: NewOrderParams =
            NewOrderParams::stop(SYMBOL, OrderSide::Buy, 120.0, 100.0, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_ok())
    }

    #[test]
    fn test_new_take_profit_test_order() {
        let trade_api = shared_test_trade();

        let params: NewOrderParams =
            NewOrderParams::take_profit(SYMBOL, OrderSide::Sell, 250.0, 255.0, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_ok())
    }

    #[test]
    fn test_new_stop_market_test_order() {
        let trade_api = shared_test_trade();

        let params: NewOrderParams =
            NewOrderParams::stop_market(SYMBOL, OrderSide::Buy, 100.0, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_ok())
    }

    #[test]
    fn test_take_profit_market_test_order() {
        let trade_api = shared_test_trade();

        let params: NewOrderParams =
            NewOrderParams::take_profit_market(SYMBOL, OrderSide::Sell, 300.0, 1.0);

        let new_order: Result<TestOrderResponse, BinanceError> =
            trade_api.send_new_test_order(&params);

        assert!(new_order.is_ok())
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
        let trade_api = shared_test_trade();

        let params: NewOrderParams = NewOrderParams::limit("EGLDUSDT", OrderSide::Buy, 13.5, 5.0);
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
        let trade_api = shared_test_trade();

        let params: NewOrderParams = NewOrderParams::limit("EGLDUSDT", OrderSide::Buy, 13.5, 5.0);
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
