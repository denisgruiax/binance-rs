#[cfg(test)]
mod futures_trade_api_integration_test {
    use std::sync::{Arc, OnceLock};

    use binance_common::{
        enums::OrderSide,
        error::BinanceError,
        futures::{endpoint::host::Host, model::params::trade::NewOrderParams},
    };
    use binance_core::{client::synchronous::Client, signer::hmacsha256::HmacSha256};
    use binance_futures::{
        secret::{API_KEY, SECRET_KEY},
        trade::TradeApi,
    };

    static TRADE_CLIENT: OnceLock<Arc<TradeApi<'static, HmacSha256<'static>>>> = OnceLock::new();
    static SYMBOL: &'static str = "SOLUSDT";

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

    #[test]
    fn test_market() {
        let trade_api = shared_test_trade();

        let params: NewOrderParams = NewOrderParams::market(SYMBOL, OrderSide::Buy, 0.1);

        let response: Result<serde_json::Value, BinanceError> =
            trade_api.post_new_test_order(params);

        println!("{:#?}", response);
    }
}
