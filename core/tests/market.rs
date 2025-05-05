#[cfg(test)]
mod market_integration {
    use binance_api::endpoint::host::Host;
    use binance_api::endpoint::route::Market;
    use binance_api::model::params::interval::Interval;
    use binance_api::model::params::market::*;
    use binance_api::model::response::market::*;
    use binance_core::client::{asynchronous::Client, signer::hmacsha256::HmacSha256};
    use serde_json::Value;

    #[tokio::test]
    async fn test_depth() {
        let params = DepthParams {
            symbol: "EGLDUSDC",
            limit: Some(10),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::Depth, params);
        let bytes = response.await.unwrap().bytes().await.unwrap();

        let depth = serde_json::from_slice::<DepthResponse>(&bytes).unwrap();

        assert!(depth.last_update_id > 0);
        assert_eq!(depth.bids.len(), 10);
        assert_eq!(depth.asks.len(), 10);
    }

    #[tokio::test]
    async fn test_trades() {
        let params = TradesParams {
            symbol: "SOLUSDC",
            limit: Some(20),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::Trades, params);
        let bytes = response.await.unwrap().bytes().await.unwrap();

        let trades = serde_json::from_slice::<Vec<TradesResponse>>(&bytes).unwrap();

        let check_trade = |trade: &TradesResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(trades.len(), 20);
        assert!(trades.iter().all(check_trade));
    }

    #[tokio::test]
    async fn test_historical_trades() {
        let params = HistoricalTradesParams {
            symbol: "SOLUSDC",
            limit: Some(17),
            from_id: None,
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::HistoricalTrades, params);
        let bytes = response.await.unwrap().bytes().await.unwrap();

        let trades = serde_json::from_slice::<Vec<HistoricalTradesResponse>>(&bytes).unwrap();

        let check_trade = |trade: &HistoricalTradesResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(trades.len(), 17);
        assert!(trades.iter().all(check_trade));
    }

    #[tokio::test]
    async fn test_klines() {
        let params = KlinesParams {
            symbol: "ETHUSDC",
            interval: Interval::Minutes5.into(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: Some(30),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::Klines, params);
        let bytes = response.await.unwrap().bytes().await.unwrap();

        let klines = serde_json::from_slice::<Vec<Value>>(&bytes)
            .unwrap()
            .into_iter()
            .map(|kline| serde_json::from_value::<KlinesResponse>(kline))
            .map(|result| result.unwrap())
            .collect::<Vec<KlinesResponse>>();

        let check_kline = |kline: &KlinesResponse| {
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
    async fn test_uiklines() {
        let params = KlinesParams {
            symbol: "ETHUSDC",
            interval: Interval::Hours1.into(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: Some(50),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::UIKlines, params);
        let bytes = response.await.unwrap().bytes().await.unwrap();

        let uiklines = serde_json::from_slice::<Vec<Value>>(&bytes)
            .unwrap()
            .into_iter()
            .map(|kline| serde_json::from_value::<KlinesResponse>(kline))
            .map(|result| result.unwrap())
            .collect::<Vec<KlinesResponse>>();

        let check_kline = |uikline: &KlinesResponse| {
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
    async fn test_average_price() {
        let params = AvgPriceParams { symbol: "FETUSDC" };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));
        let response = client.get(Market::AvgPrice, params);
        let bytes = response.await.unwrap().bytes().await.unwrap();
        let price = serde_json::from_slice::<AvgPriceResponse>(&bytes).unwrap();

        assert!(price.mins > 0);
        assert!(price.price > 0.0);
        assert!(price.close_time > 0);
    }

    #[tokio::test]
    async fn test_ticker24h() {
        let params = Ticker24hParams {
            symbol: Some("BTCUSDC"),
            symbols: None,
            r#type: Some("FULL"),
        };

        let params2 = Ticker24hParams {
            symbol: None,
            symbols: Some("[\"BTCUSDC\",\"BNBUSDC\"]"),
            r#type: Some("MINI"),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::Ticker24h, params);
        let response2 = client.get(Market::Ticker24h, params2);

        let bytes = response.await.unwrap().bytes().await.unwrap();
        let bytes2 = response2.await.unwrap().bytes().await.unwrap();

        let ticker24h_full = serde_json::from_slice::<Ticker24hFullResponse>(&bytes).unwrap();

        let ticker24h_mini_list =
            serde_json::from_slice::<Vec<Ticker24hMiniResponse>>(&bytes2).unwrap();

        let check_full_ticker24h = |ticker_statistics: &Ticker24hFullResponse| {
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

        let check_mini_ticker24h = |ticker_statistics: &Ticker24hMiniResponse| {
            ticker_statistics.open_price > 0.0
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

        assert_eq!(ticker24h_full.symbol, "BTCUSDC");
        assert!(check_full_ticker24h(&ticker24h_full));

        assert_eq!(ticker24h_mini_list[0].symbol, "BNBUSDC");
        assert_eq!(ticker24h_mini_list[1].symbol, "BTCUSDC");
        assert!(ticker24h_mini_list.iter().all(check_mini_ticker24h));
    }

    #[tokio::test]
    async fn test_ticker_day() {
        let params = TickerDayParams {
            symbol: Some("DOTUSDC"),
            symbols: None,
            time_zone: None,
            r#type: Some("FULL"),
        };

        let params2 = TickerDayParams {
            symbol: None,
            symbols: Some("[\"BTCUSDC\",\"SOLUSDC\"]"),
            time_zone: None,
            r#type: Some("MINI"),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::TickerDay, params);
        let response2 = client.get(Market::TickerDay, params2);

        let bytes = response.await.unwrap().bytes().await.unwrap();
        let bytes2 = response2.await.unwrap().bytes().await.unwrap();

        let ticker_day_full = serde_json::from_slice::<TickerDayFullResponse>(&bytes).unwrap();

        let ticker_day_mini_list =
            serde_json::from_slice::<Vec<TickerDayMiniResponse>>(&bytes2).unwrap();
        let symbols = vec!["BTCUSDC", "SOLUSDC"];
        let ticker_symbol = ticker_day_mini_list
            .into_iter()
            .zip(symbols)
            .collect::<Vec<(TickerDayMiniResponse, &str)>>();

        let check_ticker_day_full = |ticker_day: &TickerDayFullResponse, symbol: &str| {
            ticker_day.symbol == symbol
                && ticker_day.weighted_avg_price > 0.0
                && ticker_day.open_price > 0.0
                && ticker_day.high_price > 0.0
                && ticker_day.low_price > 0.0
                && ticker_day.last_price > 0.0
                && ticker_day.volume > 0.0
                && ticker_day.quote_volume > 0.0
                && ticker_day.open_time > 0
                && ticker_day.close_time > 0
                && ticker_day.first_id > 0
                && ticker_day.last_id >= ticker_day.first_id
                && ticker_day.count > 0
        };

        let check_trading_day_mini = |ticker_day: &TickerDayMiniResponse, symbol: &str| {
            ticker_day.symbol == symbol
                && ticker_day.open_price > 0.0
                && ticker_day.high_price > 0.0
                && ticker_day.low_price > 0.0
                && ticker_day.last_price > 0.0
                && ticker_day.volume > 0.0
                && ticker_day.quote_volume > 0.0
                && ticker_day.open_time > 0
                && ticker_day.close_time > 0
                && ticker_day.first_id > 0
                && ticker_day.last_id >= ticker_day.first_id
                && ticker_day.count > 0
        };

        assert!(check_ticker_day_full(&ticker_day_full, "DOTUSDC"));
        assert!(
            ticker_symbol
                .iter()
                .all(|(td, s)| check_trading_day_mini(td, s))
        );
    }

    #[tokio::test]
    async fn test_price_ticker() {
        let params = PriceTickerParams {
            symbol: None,
            symbols: Some("[\"BTCUSDC\",\"SOLUSDC\"]"),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::PriceTicker, params);

        let bytes = response.await.unwrap().bytes().await.unwrap();
        let prices = serde_json::from_slice::<Vec<PriceTickerResponse>>(&bytes).unwrap();

        assert_eq!(prices.len(), 2);
        assert!(prices.iter().all(|p| p.price > 0.0));
    }
}
