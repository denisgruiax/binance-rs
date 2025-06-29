use binance_common::error::BinanceError;
use binance_common::futures::endpoint::route::Market;
use binance_common::futures::model::{
    params::market::{EmptyParams, ExchangeInformationParams},
    response::market::{EmptyResponse, ExchangeInformationResponse, ServerTimeResponse},
};
use binance_core::{client::synchronous::Client, signer::signature::Signature};

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

    pub fn ping(&self) -> Result<EmptyResponse, BinanceError> {
        self.client.get(Market::Ping, EmptyParams)
    }

    pub fn get_server_time(&self) -> Result<ServerTimeResponse, BinanceError> {
        self.client.get(Market::ServerTime, EmptyParams)
    }

    pub fn get_exchange_info(
        &self,
        params: ExchangeInformationParams,
    ) -> Result<ExchangeInformationResponse, BinanceError> {
        self.client.get(Market::ExchangeInfo, params)
    }
}

#[cfg(test)]
mod futures_market_api {
    use super::MarketApi;
    use binance_common::futures::{
        endpoint::host::Host,
        model::{params::market::ExchangeInformationParams, response::market::EmptyResponse},
    };
    use binance_core::{
        client::synchronous::Client,
        signer::{hmacsha256::HmacSha256, signature::Signature},
    };
    use std::sync::{Arc, OnceLock};

    static CLIENT: OnceLock<Arc<MarketApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_client<'a, S>() -> Arc<MarketApi<'static, HmacSha256<'static>>>
    where
        S: Signature<'a>,
    {
        CLIENT
            .get_or_init(|| {
                Arc::new(MarketApi::new(Client::new(
                    &Host::Api,
                    HmacSha256::new("api_key", "secret_key"),
                )))
            })
            .clone()
    }

    #[test]
    fn test_ping() {
        let general_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>().clone();

        assert_eq!(general_api.ping().unwrap(), EmptyResponse {});
    }
    #[test]
    fn test_get_server_time() {
        let general_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>().clone();

        assert!(general_api.get_server_time().unwrap().server_time > 0);
    }

    #[test]
    fn test_get_exchange_info() {
        let general_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params = ExchangeInformationParams::new().symbol("BTCUSDT");

        let exchange_info = general_api.get_exchange_info(params).unwrap();

        assert_eq!(exchange_info.timezone, "UTC");
        assert!(exchange_info.server_time > 0);

        assert_eq!(exchange_info.symbols[0].symbol, "BTCUSDT");
        assert_eq!(exchange_info.symbols[0].status, "TRADING");
        assert_eq!(exchange_info.symbols[0].base_asset, "BTC");
        assert_eq!(exchange_info.symbols[0].quote_asset, "USDT");
        assert_eq!(exchange_info.symbols[0].base_asset_precision, 8);
        assert!(exchange_info.symbols[0].order_types.len() == 7);
    }
}
