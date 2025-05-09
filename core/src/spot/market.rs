use crate::client::{signer::signature::Signature, synchronous::Client};
use binance_api::endpoint::route::Market;
use binance_api::model::params::market::*;
use binance_api::model::{BinanceError, response::market::DepthResponse};

pub struct MarketApi<'a, S>
where
    S: Signature<'a>,
{
    client: Client<'a, S>,
}

impl<'a, S> MarketApi<'a, S>
where
    S: Signature<'a>,
{
    pub fn new(client: Client<'a, S>) -> Self {
        MarketApi { client }
    }

    pub fn depth(&self, params: DepthParams) -> Result<DepthResponse, BinanceError> {
        self.client.get(Market::Depth.into(), params)
    }
}

#[cfg(test)]
mod market_api {
    use binance_api::{
        endpoint::host::Host,
        model::{params::market::DepthParams, response::market::DepthResponse},
    };
    use futures::stream::Once;

    use std::sync::{Arc, OnceLock};

    use crate::client::{signer::hmacsha256::HmacSha256, synchronous::Client};

    use super::MarketApi;

    static CLIENT: OnceLock<Arc<MarketApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_market() -> Arc<MarketApi<'static, HmacSha256<'static>>> {
        CLIENT
            .get_or_init(|| {
                Arc::new(MarketApi::new(Client::new(
                    Host::Api.into(),
                    HmacSha256::new("api_key", "secret_key"),
                )))
            })
            .clone()
    }

    #[test]
    fn test_depth() {
        let market_api = shared_test_market();
        let params = DepthParams {
            symbol: "BTCUSDC",
            limit: Some(20),
        };

        let depth: DepthResponse = market_api.depth(params).unwrap();

        assert!(depth.last_update_id > 0);
        assert_eq!(depth.bids.len(), 20);
        assert_eq!(depth.asks.len(), 20);
    }
}
