#[cfg(test)]
mod futures_market_api_integration_tests {
    use binance_common::futures::model::params::market::{DepthParams, TradesParams};
    use binance_common::futures::model::response::market::{
        DepthResponse, HistoricalTradesResponse, TradesResponse,
    };
    use binance_common::futures::{
        endpoint::host::Host,
        model::{params::market::ExchangeInformationParams, response::market::EmptyResponse},
    };
    use binance_common::spot::model::params::market::HistoricalTradesParams;
    use binance_core::{
        client::synchronous::Client,
        signer::{hmacsha256::HmacSha256, signature::Signature},
    };
    use binance_futures::market::MarketApi;
    use binance_futures::secret::{API_KEY, SECRET_KEY};
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
                    HmacSha256::new(API_KEY, SECRET_KEY),
                )))
            })
            .clone()
    }

    #[test]
    fn test_ping() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>().clone();

        assert_eq!(market_api.ping().unwrap(), EmptyResponse {});
    }
    #[test]
    fn test_get_server_time() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>().clone();

        assert!(market_api.get_server_time().unwrap().server_time > 0);
    }

    #[test]
    fn test_get_exchange_info() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params = ExchangeInformationParams::new().symbol("BTCUSDT");

        let exchange_info = market_api.get_exchange_info(params).unwrap();

        assert_eq!(exchange_info.timezone, "UTC");
        assert!(exchange_info.server_time > 0);

        assert_eq!(exchange_info.symbols[0].symbol, "BTCUSDT");
        assert_eq!(exchange_info.symbols[0].status, "TRADING");
        assert_eq!(exchange_info.symbols[0].base_asset, "BTC");
        assert_eq!(exchange_info.symbols[0].quote_asset, "USDT");
        assert_eq!(exchange_info.symbols[0].base_asset_precision, 8);
        assert!(exchange_info.symbols[0].order_types.len() == 7);
    }

    #[test]
    fn test_get_depth() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params = DepthParams::new("BTCUSDT").limit(5);

        let depth: DepthResponse = market_api.get_depth(params).unwrap();

        assert!(depth.last_update_id > 0);
        assert!(depth.e > 0);
        assert!(depth.t > 0);
        assert_eq!(depth.bids.len(), 5);
        assert_eq!(depth.asks.len(), 5);
    }

    #[test]
    fn test_get_trades() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: TradesParams = TradesParams::new("ICPUSDT").limit(5);

        let trades: Vec<TradesResponse> = market_api.get_trades(params).unwrap();

        assert_eq!(trades.len(), 5);
    }

    #[test]
    fn test_get_historical_trades() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: HistoricalTradesParams = HistoricalTradesParams::new("EGLDUSDT").limit(10);

        let historical_trades: Vec<HistoricalTradesResponse> =
            market_api.get_historical_trades(params).unwrap();

        assert_eq!(historical_trades.len(), 10);
    }
}
