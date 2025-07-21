#[cfg(test)]
mod futures_market_api_integration_tests {
    use binance_common::enums::{ContractType, Interval};
    use binance_common::futures::model::params::market::{
        BasisParams, DepthParams, FundingRateHistoryParams, GlobalLongShortAccountRatioParams,
        KlinesParams, OpenInterestHistoryParams, Pair, Symbol, TakerBuySellVolumeParams,
        TopLongShortAccountRatioParams, TopLongShortPositionRatioParams, TradesParams,
    };
    use binance_common::futures::model::response::market::{
        BasisResponse, BookTickerResponse, DeliveryPriceResponse, DepthResponse,
        FundingRateHistoryResponse, GlobalLongShortAccountRatioResponse, HistoricalTradesResponse,
        KlinesResponse, MarkPriceResponse, OpenInterestHistoryResponse, OpenInterestResponse,
        TakerBuySellVolumeResponse, Ticker24hResponse, TickerPriceResponse,
        TopLongShortAccountRatioResponse, TopLongShortPositionRatioResponse, TradesResponse,
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

        let exchange_info = market_api.get_exchange_info(&params).unwrap();

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

        let depth: DepthResponse = market_api.get_depth(&params).unwrap();

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

        let trades: Vec<TradesResponse> = market_api.get_trades(&params).unwrap();

        assert_eq!(trades.len(), 5);
    }

    #[test]
    fn test_get_historical_trades() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: HistoricalTradesParams = HistoricalTradesParams::new("EGLDUSDT").limit(10);

        let historical_trades: Vec<HistoricalTradesResponse> =
            market_api.get_historical_trades(&params).unwrap();

        assert_eq!(historical_trades.len(), 10);
    }

    fn check_kline(kline: &KlinesResponse) -> bool {
        kline.open_time > 0
            && kline.open > 0.0
            && kline.high > 0.0
            && kline.low > 0.0
            && kline.close > 0.0
            && kline.volume > 0.0
            && kline.close_time > 0
            && kline.quote_asset_volume > 0.0
            && kline.number_of_trades > 0
            && kline.taker_buy_base_asset_volume > 0.0
            && kline.taker_buy_quote_asset_volume > 0.0
    }

    #[test]
    fn test_get_klines() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params = KlinesParams::new("ETHUSDT", &Interval::Minutes5).limit(30);

        let klines: Vec<KlinesResponse> = market_api.get_klines(&params).unwrap();

        assert_eq!(klines.len(), 30);
        assert!(klines.iter().all(check_kline));
    }

    #[test]
    fn test_get_mark_price() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: Symbol = Symbol::new("ETHUSDT");

        let mark_price: MarkPriceResponse = market_api.get_mark_price(&params).unwrap();

        assert_eq!(mark_price.symbol, "ETHUSDT");
        assert!(mark_price.mark_price > 0.0);
        assert!(mark_price.time > 0);
    }

    #[test]
    fn test_get_mark_price_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let mark_price_list: Vec<MarkPriceResponse> = market_api.get_mark_price_list().unwrap();

        assert!(mark_price_list.len() > 0);
        assert!(
            mark_price_list
                .iter()
                .all(|mark_price| { mark_price.mark_price > 0.0 && mark_price.time > 0 })
        );
    }

    #[test]
    fn test_get_funding_rate_history() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: FundingRateHistoryParams = FundingRateHistoryParams {
            symbol: Some("BTCUSDT"),
            start_time: None,
            end_time: None,
            limit: Some(8),
        };

        let funding_rate_history: Vec<FundingRateHistoryResponse> =
            market_api.get_funding_rate_history(&params).unwrap();

        assert_eq!(funding_rate_history.len(), 8);
        assert!(funding_rate_history.iter().all(|funding_rate| {
            funding_rate.mark_price > 0.0 && funding_rate.funding_time > 0
        }));
    }

    #[test]
    fn test_get_ticker24() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: Symbol = Symbol::new("BTCUSDT");

        let ticker24h: Ticker24hResponse = market_api.get_ticker24h(&params).unwrap();

        assert_eq!(ticker24h.symbol, "BTCUSDT");
        assert!(ticker24h.last_price > 0.0);
        assert!(ticker24h.open_price > 0.0);
        assert!(ticker24h.high_price > 0.0);
        assert!(ticker24h.low_price > 0.0);
    }

    #[test]
    fn test_get_ticker24_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let ticker24h_list: Vec<Ticker24hResponse> = market_api.get_ticker24h_list().unwrap();

        assert!(ticker24h_list.len() > 0);
    }

    #[test]
    fn test_get_ticker_price() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: Symbol = Symbol::new("BTCUSDT");

        let ticker24h: TickerPriceResponse = market_api.get_ticker_price(&params).unwrap();

        assert_eq!(ticker24h.symbol, "BTCUSDT");
        assert!(ticker24h.price > 0.0);
        assert!(ticker24h.time > 0)
    }

    #[test]
    fn test_get_ticker_price_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let ticker24h_list: Vec<TickerPriceResponse> = market_api.get_ticker_price_list().unwrap();

        assert!(ticker24h_list.len() > 0);
    }

    #[test]
    fn test_get_ticker_price_v2() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: Symbol = Symbol::new("BTCUSDT");

        let ticker24h: TickerPriceResponse = market_api.get_ticker_price_v2(&params).unwrap();

        assert_eq!(ticker24h.symbol, "BTCUSDT");
        assert!(ticker24h.price > 0.0);
        assert!(ticker24h.time > 0)
    }

    #[test]
    fn test_get_ticker_price_v2_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let ticker24h_list: Vec<TickerPriceResponse> =
            market_api.get_ticker_price_v2_list().unwrap();

        assert!(ticker24h_list.len() > 0);
    }

    #[test]
    fn test_get_book_ticker() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: Symbol = Symbol::new("ETHUSDT");

        let book_ticker = market_api.get_book_ticker(&params).unwrap();

        assert_eq!(book_ticker.symbol, "ETHUSDT");
        assert!(book_ticker.bid_price > 0.0);
        assert!(book_ticker.ask_qty > 0.0);
    }

    #[test]
    fn test_get_book_ticker_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let book_ticker: Vec<BookTickerResponse> = market_api.get_book_ticker_list().unwrap();

        assert!(book_ticker.len() > 0);
    }

    #[test]
    fn test_get_delivery_price_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let book_ticker: Vec<DeliveryPriceResponse> = market_api
            .get_delivery_price_list(&Pair::new("BTCUSDT"))
            .unwrap();

        assert!(book_ticker.len() > 0);
    }

    #[test]
    fn test_get_open_interest() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let open_interest: OpenInterestResponse = market_api
            .get_open_interest(&Symbol::new("SOLUSDT"))
            .unwrap();

        assert!(open_interest.open_interest > 0.0);
        assert_eq!(open_interest.symbol, "SOLUSDT");
        assert!(open_interest.time > 0);
    }

    #[test]
    fn test_get_open_interest_history_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: OpenInterestHistoryParams =
            OpenInterestHistoryParams::new("SOLUSDT", &Interval::Minutes5);

        let open_interet_history: Vec<OpenInterestHistoryResponse> =
            market_api.get_open_interest_history_list(&params).unwrap();

        assert!(open_interet_history.len() > 0);
    }

    #[test]
    fn test_get_top_long_position_ratio_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: TopLongShortPositionRatioParams =
            TopLongShortPositionRatioParams::new("SOLUSDT", &Interval::Minutes5);

        let data: Vec<TopLongShortPositionRatioResponse> = market_api
            .get_top_long_position_ratio_list(&params)
            .unwrap();

        assert!(data.len() > 0);
    }

    #[test]
    fn test_get_top_long_account_ratio_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: TopLongShortAccountRatioParams =
            TopLongShortAccountRatioParams::new("SOLUSDT", &Interval::Minutes5);

        let data: Vec<TopLongShortAccountRatioResponse> =
            market_api.get_top_long_account_ratio_list(&params).unwrap();

        assert!(data.len() > 0);
    }

    #[test]
    fn test_get_global_long_account_ratio_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: GlobalLongShortAccountRatioParams =
            GlobalLongShortAccountRatioParams::new("SOLUSDT", &Interval::Minutes5);

        let data: Vec<GlobalLongShortAccountRatioResponse> = market_api
            .get_global_long_account_ratio_list(&params)
            .unwrap();

        assert!(data.len() > 0);
    }

    #[test]
    fn test_get_taker_buy_sell_volume_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: TakerBuySellVolumeParams =
            TakerBuySellVolumeParams::new("SOLUSDT", &Interval::Minutes5);

        let data: Vec<TakerBuySellVolumeResponse> =
            market_api.get_taker_buy_sell_volume_list(&params).unwrap();

        assert!(data.len() > 0);
    }

    #[test]
    fn test_get_basis_list() {
        let market_api: Arc<MarketApi<HmacSha256>> = shared_test_client::<HmacSha256>();

        let params: BasisParams =
            BasisParams::new("SOLUSDT", ContractType::Perpetual, &Interval::Minutes5, 17);

        let data: Vec<BasisResponse> = market_api.get_basis_list(&params).unwrap();

        assert_eq!(data.len(), 17);
    }
}
