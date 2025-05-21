use binance_api::{
    endpoint::route::Trading,
    model::{BinanceError, params::trading::NewOrderParams},
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

    pub fn post_new_order(
        &self,
        params: NewOrderParams,
    ) -> Result<serde_json::Value, BinanceError> {
        self.client.post(Trading::TestOrder, params)
    }
}

#[cfg(test)]
mod trading_api {
    use crate::client::{signer::hmacsha256::HmacSha256, synchronous::Client};

    use super::TradingApi;
    use crate::spot::secret::{API_KEY, SECRET_KEY};
    use binance_api::{
        endpoint::host::Host,
        model::params::{
            binance::{OrderResponseType, OrderSide, OrderType},
            trading::NewOrderParams,
        },
    };
    use std::sync::{Arc, OnceLock};

    static CLIENT: OnceLock<Arc<TradingApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_trading() -> Arc<TradingApi<'static, HmacSha256<'static>>> {
        CLIENT
            .get_or_init(|| {
                Arc::new(TradingApi::new(Client::new(
                    Host::Api.as_ref(),
                    HmacSha256::new(API_KEY, SECRET_KEY),
                )))
            })
            .clone()
    }

    #[test]
    fn test_post_new_order() {
        let trading_api = shared_test_trading();

        let params = NewOrderParams::market("BTCUSDC", OrderSide::Buy, OrderType::Market, 150.0)
            .new_order_resp_type(OrderResponseType::Ack);

        let response = trading_api.post_new_order(params);

        match response {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
