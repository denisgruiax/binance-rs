#[cfg(test)]
mod market_integration {
    use binance_api::endpoint::route::Market;
    use binance_api::model::params::market::{CurrentAveragePriceParams, TickerStatisticsParams};
    use binance_api::model::params::{
        interval::Interval,
        market::{KlineParams, OldTradeLookupParams, RecentTradeListParams},
    };
    use binance_api::model::response::market::{
        CurrentAveragePriceResponse, Kline, OldTradeLookupResponse, RecentTradeResponse,
        TickerStatisticsFullResponse, TickerStatisticsMiniResponse,
    };
    use binance_api::{
        endpoint::host::Host,
        model::{params::market::OrderBookParams, response::market::OrderBookResponse},
    };
    use binance_core::client::{asynchronous::Client, signer::hmacsha256::HmacSha256};
    use serde_json::Value;

    #[tokio::test]
    async fn test_order_book() {
        let params = OrderBookParams {
            symbol: "EGLDUSDC",
            limit: Some(10),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::OrderBook, params);
        let body = response.await.unwrap().text().await.unwrap();

        let order_book = serde_json::from_str::<OrderBookResponse>(&body).unwrap();

        assert!(order_book.last_update_id > 0);
        assert_eq!(order_book.bids.len(), 10);
        assert_eq!(order_book.asks.len(), 10);
    }

    #[tokio::test]
    async fn test_recent_trade_list() {
        let params = RecentTradeListParams {
            symbol: "SOLUSDC",
            limit: Some(20),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::RecentTradeList, params);
        let body = response.await.unwrap().text().await.unwrap();

        let recent_trade_list = serde_json::from_str::<Vec<RecentTradeResponse>>(&body).unwrap();

        let check_trade = |trade: &RecentTradeResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(recent_trade_list.len(), 20);
        assert!(recent_trade_list.iter().all(check_trade));
    }

    #[tokio::test]
    async fn test_old_trade_lookup() {
        let params = OldTradeLookupParams {
            symbol: "SOLUSDC",
            limit: Some(17),
            from_id: None,
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::OldTradeLookup, params);
        let body = response.await.unwrap().text().await.unwrap();

        let recent_trade_list = serde_json::from_str::<Vec<OldTradeLookupResponse>>(&body).unwrap();

        let check_trade = |trade: &OldTradeLookupResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(recent_trade_list.len(), 17);
        assert!(recent_trade_list.iter().all(check_trade));
    }

    #[tokio::test]
    async fn test_kline_candlestick_data() {
        let params = KlineParams {
            symbol: "ETHUSDC",
            interval: Interval::Minutes5.into(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: Some(30),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::KlineCandlestickData, params);
        let body = response.await.unwrap().text().await.unwrap();

        let klines = serde_json::from_str::<Vec<Value>>(&body)
            .unwrap()
            .into_iter()
            .map(|kline| serde_json::from_value::<Kline>(kline))
            .map(|result| result.unwrap())
            .collect::<Vec<Kline>>();

        let check_kline = |kline: &Kline| {
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
        };

        assert_eq!(klines.len(), 30);
        assert!(klines.iter().all(check_kline));
    }

    #[tokio::test]
    async fn test_uikline_candlestick_data() {
        let params = KlineParams {
            symbol: "ETHUSDC",
            interval: Interval::Hours1.into(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: Some(50),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::UIKlines, params);
        let body = response.await.unwrap().text().await.unwrap();

        let uiklines = serde_json::from_str::<Vec<Value>>(&body)
            .unwrap()
            .into_iter()
            .map(|kline| serde_json::from_value::<Kline>(kline))
            .map(|result| result.unwrap())
            .collect::<Vec<Kline>>();

        let check_kline = |uikline: &Kline| {
            uikline.open_time > 0
                && uikline.open > 0.0
                && uikline.high > 0.0
                && uikline.low > 0.0
                && uikline.close > 0.0
                && uikline.volume > 0.0
                && uikline.close_time > 0
                && uikline.quote_asset_volume > 0.0
                && uikline.number_of_trades > 0
                && uikline.taker_buy_base_asset_volume > 0.0
                && uikline.taker_buy_quote_asset_volume > 0.0
        };

        assert_eq!(uiklines.len(), 50);
        assert!(uiklines.iter().all(check_kline));
    }

    #[tokio::test]
    async fn test_current_average_price() {
        let params = CurrentAveragePriceParams { symbol: "FETUSDC" };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));
        let response = client.get(Market::CurrentAveragePrice, params);
        let body = response.await.unwrap().text().await.unwrap();
        let current_average_price =
            serde_json::from_str::<CurrentAveragePriceResponse>(&body).unwrap();

        assert!(current_average_price.mins > 0);
        assert!(current_average_price.price > 0.0);
        assert!(current_average_price.close_time > 0);
    }

    #[tokio::test]
    async fn test_ticker_price_change_statistics_24h() {
        let params = TickerStatisticsParams {
            symbol: Some("BTCUSDC"),
            symbols: None,
            r#type: Some("FULL"),
        };

        let params2 = TickerStatisticsParams {
            symbol: None,
            symbols: Some("[\"BTCUSDC\",\"BNBUSDC\"]"),
            r#type: Some("MINI"),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::TickerStatistics, params);
        let response2 = client.get(Market::TickerStatistics, params2);

        let body = response.await.unwrap().text().await.unwrap();
        let body2 = response2.await.unwrap().text().await.unwrap();

        let ticker_statistics_full =
            serde_json::from_str::<TickerStatisticsFullResponse>(&body).unwrap();

        let ticker_statistics_mini_list =
            serde_json::from_str::<Vec<TickerStatisticsMiniResponse>>(&body2).unwrap();

        let check_full_ticker_statistics = |ticker_statistics: &TickerStatisticsFullResponse| {
            ticker_statistics.price_change != 0.0
                && ticker_statistics.price_change_percent != 0.0
                && ticker_statistics.weighted_avg_price > 0.0
                && ticker_statistics.prev_close_price > 0.0
                && ticker_statistics.last_price > 0.0
                && ticker_statistics.last_qty > 0.0
                && ticker_statistics.bid_price > 0.0
                && ticker_statistics.bid_qty > 0.0
                && ticker_statistics.ask_price > 0.0
                && ticker_statistics.ask_qty > 0.0
                && ticker_statistics.open_price > 0.0
                && ticker_statistics.high_price > 0.0
                && ticker_statistics.low_price > 0.0
                && ticker_statistics.volume > 0.0
                && ticker_statistics.quote_volume > 0.0
                && ticker_statistics.open_time > 0
                && ticker_statistics.close_time > 0
                && ticker_statistics.first_id > 0
                && ticker_statistics.last_id > 0
                && ticker_statistics.count > 0
        };

        let check_mini_ticker_statistics = |ticker_statistics: &TickerStatisticsMiniResponse|{
            ticker_statistics.open_price> 0.0
                && ticker_statistics.high_price > 0.0
                && ticker_statistics.low_price > 0.0
                && ticker_statistics.last_price > 0.0
                && ticker_statistics.volume > 0.0
                && ticker_statistics.quote_volume > 0.0
                && ticker_statistics.open_time > 0
                && ticker_statistics.close_time > 0
                && ticker_statistics.first_id > 0
                && ticker_statistics.last_id > 0
                && ticker_statistics.count > 0
        };

        assert_eq!(ticker_statistics_full.symbol, "BTCUSDC");
        assert!(check_full_ticker_statistics(&ticker_statistics_full));

        assert_eq!(ticker_statistics_mini_list[0].symbol, "BNBUSDC");
        assert_eq!(ticker_statistics_mini_list[1].symbol, "BTCUSDC");
        assert!(ticker_statistics_mini_list.iter().all(check_mini_ticker_statistics));
    }
}
