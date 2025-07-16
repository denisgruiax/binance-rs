#[cfg(test)]
mod spot_general_api_integration_tests {
    use binance_spot::general::GeneralApi;
    use binance_common::spot::{
        endpoint::host::Host,
        model::{params::general::ExchangeInformationParams, response::general::EmptyResponse},
    };
    use binance_core::{
        client::synchronous::Client,
        signer::{hmacsha256::HmacSha256, signature::Signature},
    };
    use std::sync::{Arc, OnceLock};

    static CLIENT: OnceLock<Arc<GeneralApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_client<'a, S>() -> Arc<GeneralApi<'static, HmacSha256<'static>>>
    where
        S: Signature<'a>,
    {
        CLIENT
            .get_or_init(|| {
                Arc::new(GeneralApi::new(Client::new(
                    &Host::Api,
                    HmacSha256::new("api_key", "secret_key"),
                )))
            })
            .clone()
    }

    #[test]
    fn test_ping() {
        let general_api: Arc<GeneralApi<HmacSha256>> = shared_test_client::<HmacSha256>().clone();

        assert_eq!(general_api.ping().unwrap(), EmptyResponse {});
    }
    #[test]
    fn test_get_server_time() {
        let general_api: Arc<GeneralApi<HmacSha256>> = shared_test_client::<HmacSha256>().clone();

        assert!(general_api.get_server_time().unwrap().server_time > 0);
    }

    #[test]
    fn test_get_exchange_info() {
        let general_api: Arc<GeneralApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params = ExchangeInformationParams::new().symbol("BTCUSDC");

        let exchange_info = general_api.get_exchange_info(&params).unwrap();

        assert_eq!(exchange_info.timezone, "UTC");
        assert!(exchange_info.server_time > 0);

        assert_eq!(exchange_info.symbols[0].symbol, "BTCUSDC");
        assert_eq!(exchange_info.symbols[0].status, "TRADING");
        assert_eq!(exchange_info.symbols[0].base_asset, "BTC");
        assert_eq!(exchange_info.symbols[0].quote_asset, "USDC");
        assert_eq!(exchange_info.symbols[0].base_asset_precision, 8);
        assert!(exchange_info.symbols[0].order_types.len() == 7);
        assert!(exchange_info.symbols[0].iceberg_allowed);
        assert!(exchange_info.symbols[0].is_spot_trading_allowed);
        assert!(exchange_info.symbols[0].is_margin_trading_allowed);
    }
}