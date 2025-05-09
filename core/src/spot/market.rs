use crate::client::{signer::signature::Signature, synchronous::Client};
use binance_api::endpoint::route::Market;
use binance_api::model::params::market::*;
use binance_api::model::response::market::{
    AvgPriceResponse, HistoricalTradesResponse, KlinesResponse, Ticker24hFullResponse,
    Ticker24hMiniResponse, TickerDayFullResponse, TickerDayMiniResponse, TradesResponse,
};
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

    pub fn get_depth(&self, params: DepthParams) -> Result<DepthResponse, BinanceError> {
        self.client.get(Market::Depth.into(), params)
    }

    pub fn get_trades(&self, params: TradesParams) -> Result<Vec<TradesResponse>, BinanceError> {
        self.client.get(Market::Trades.into(), params)
    }

    pub fn get_historical_trades(
        &self,
        params: HistoricalTradesParams,
    ) -> Result<Vec<HistoricalTradesResponse>, BinanceError> {
        self.client.get(Market::HistoricalTrades.into(), params)
    }

    pub fn get_klines(&self, params: KlinesParams) -> Result<Vec<KlinesResponse>, BinanceError> {
        self.client.get(Market::Klines.into(), params)
    }

    pub fn get_uiklines(&self, params: KlinesParams) -> Result<Vec<KlinesResponse>, BinanceError> {
        self.client.get(Market::UIKlines.into(), params)
    }

    pub fn get_average_price(
        &self,
        params: AvgPriceParams,
    ) -> Result<AvgPriceResponse, BinanceError> {
        self.client.get(Market::AvgPrice.into(), params)
    }

    pub fn get_ticker24h_mini(
        &self,
        params: Ticker24hParams,
    ) -> Result<Ticker24hMiniResponse, BinanceError> {
        self.client.get(Market::Ticker24h.into(), params)
    }

    pub fn get_ticker24h_full(
        &self,
        params: Ticker24hParams,
    ) -> Result<Ticker24hFullResponse, BinanceError> {
        self.client.get(Market::Ticker24h.into(), params)
    }

    pub fn get_ticker24h_mini_list(
        &self,
        params: Ticker24hParams,
    ) -> Result<Vec<Ticker24hMiniResponse>, BinanceError> {
        self.client.get(Market::Ticker24h.into(), params)
    }

    pub fn get_ticker24h_full_list(
        &self,
        params: Ticker24hParams,
    ) -> Result<Vec<Ticker24hFullResponse>, BinanceError> {
        self.client.get(Market::Ticker24h.into(), params)
    }

    pub fn get_ticker_day_mini(
        &self,
        params: TickerDayParams,
    ) -> Result<TickerDayMiniResponse, BinanceError> {
        self.client.get(Market::TickerDay.into(), params)
    }

    pub fn get_ticker_day_full(
        &self,
        params: TickerDayParams,
    ) -> Result<TickerDayFullResponse, BinanceError> {
        self.client.get(Market::TickerDay.into(), params)
    }

    pub fn get_ticker_day_mini_list(
        &self,
        params: TickerDayParams,
    ) -> Result<Vec<TickerDayMiniResponse>, BinanceError> {
        self.client.get(Market::TickerDay.into(), params)
    }

    pub fn get_ticker_day_full_list(
        &self,
        params: TickerDayParams,
    ) -> Result<Vec<TickerDayFullResponse>, BinanceError> {
        self.client.get(Market::TickerDay.into(), params)
    }
}

#[cfg(test)]
mod market_api {
    use super::MarketApi;
    use crate::client::{signer::hmacsha256::HmacSha256, synchronous::Client};
    use binance_api::{
        endpoint::host::Host,
        model::{
            params::{
                interval::Interval,
                market::{
                    AvgPriceParams, DepthParams, HistoricalTradesParams, KlinesParams,
                    Ticker24hParams, TickerDayParams, TradesParams,
                },
            },
            response::market::{
                AvgPriceResponse, DepthResponse, HistoricalTradesResponse, KlinesResponse,
                Ticker24hFullResponse, Ticker24hMiniResponse, TickerDayFullResponse,
                TickerDayMiniResponse, TradesResponse,
            },
        },
    };
    use std::sync::{Arc, OnceLock};

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
    fn test_get_depth() {
        let market_api = shared_test_market();
        let params = DepthParams {
            symbol: "BTCUSDC",
            limit: Some(20),
        };

        let depth: DepthResponse = market_api.get_depth(params).unwrap();

        assert!(depth.last_update_id > 0);
        assert_eq!(depth.bids.len(), 20);
        assert_eq!(depth.asks.len(), 20);
    }

    #[test]
    fn test_get_trades() {
        let market_api = shared_test_market();
        let params = TradesParams {
            symbol: "SOLUSDC",
            limit: Some(25),
        };

        let trades: Vec<TradesResponse> = market_api.get_trades(params).unwrap();

        let check_trade = |trade: &TradesResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(trades.len(), 25);
        assert!(trades.iter().all(check_trade));
    }

    #[test]
    fn test_get_historical_trades() {
        let market_api = shared_test_market();
        let params = HistoricalTradesParams {
            symbol: "SOLUSDC",
            limit: Some(40),
            from_id: None,
        };

        let trades: Vec<HistoricalTradesResponse> =
            market_api.get_historical_trades(params).unwrap();

        let check_trade = |trade: &HistoricalTradesResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(trades.len(), 40);
        assert!(trades.iter().all(check_trade));
    }

    #[test]
    fn test_get_klines() {
        let market_api = shared_test_market();
        let params = KlinesParams {
            symbol: "ETHUSDC",
            interval: Interval::Minutes5.into(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: Some(30),
        };

        let klines: Vec<KlinesResponse> = market_api.get_klines(params).unwrap();

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

    #[test]
    fn test_get_uiklines() {
        let market_api = shared_test_market();
        let params = KlinesParams {
            symbol: "ETHUSDC",
            interval: Interval::Minutes5.into(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: Some(30),
        };

        let klines: Vec<KlinesResponse> = market_api.get_uiklines(params).unwrap();

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

    #[test]
    fn test_get_average_price() {
        let market_api = shared_test_market();
        let params = AvgPriceParams { symbol: "FETUSDC" };

        let average_price: AvgPriceResponse = market_api.get_average_price(params).unwrap();

        assert!(average_price.mins > 0);
        assert!(average_price.price > 0.0);
        assert!(average_price.close_time > 0);
    }

    #[test]
    fn test_get_ticker24h_mini() {
        let market_api = shared_test_market();
        let params = Ticker24hParams {
            symbol: Some("BTCUSDC"),
            symbols: None,
            r#type: Some("MINI"),
        };

        let ticker24h_mini: Ticker24hMiniResponse = market_api.get_ticker24h_mini(params).unwrap();

        let check_ticker24h_mini = |ticker_statistics: &Ticker24hMiniResponse| {
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

        assert_eq!(ticker24h_mini.symbol, "BTCUSDC");
        assert!(check_ticker24h_mini(&ticker24h_mini));
    }

    #[test]
    fn test_get_ticker24h_full() {
        let market_api = shared_test_market();
        let params = Ticker24hParams {
            symbol: Some("DOTUSDC"),
            symbols: None,
            r#type: Some("FULL"),
        };

        let ticker24h_full: Ticker24hFullResponse = market_api.get_ticker24h_full(params).unwrap();

        let check_ticker24h_full = |ticker_statistics: &Ticker24hFullResponse| {
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

        assert_eq!(ticker24h_full.symbol, "DOTUSDC");
        assert!(check_ticker24h_full(&ticker24h_full));
    }

    #[test]
    fn test_get_ticker24h_mini_list() {
        let market_api = shared_test_market();
        let params = Ticker24hParams {
            symbol: None,
            symbols: Some("[\"BTCUSDC\",\"SOLUSDC\"]"),
            r#type: Some("MINI"),
        };

        let ticker24h_mini_list: Vec<Ticker24hMiniResponse> =
            market_api.get_ticker24h_mini_list(params).unwrap();

        let check_ticker24h_mini = |ticker_statistics: &Ticker24hMiniResponse| {
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

        assert_eq!(ticker24h_mini_list[0].symbol, "BTCUSDC");
        assert_eq!(ticker24h_mini_list[1].symbol, "SOLUSDC");
        assert!(ticker24h_mini_list.iter().all(check_ticker24h_mini));
    }

    #[test]
    fn test_get_ticker24h_full_list() {
        let market_api = shared_test_market();
        let params = Ticker24hParams {
            symbol: None,
            symbols: Some("[\"BTCUSDC\",\"SOLUSDC\"]"),
            r#type: Some("FULL"),
        };

        let ticker24h_full_list: Vec<Ticker24hFullResponse> =
            market_api.get_ticker24h_full_list(params).unwrap();

        let check_ticker24h_full = |ticker_statistics: &Ticker24hFullResponse| {
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

        assert_eq!(ticker24h_full_list[0].symbol, "BTCUSDC");
        assert_eq!(ticker24h_full_list[1].symbol, "SOLUSDC");
        assert!(ticker24h_full_list.iter().all(check_ticker24h_full));
    }

    #[test]
    fn test_get_ticker_day_mini() {
        let market_api = shared_test_market();
        let params = TickerDayParams {
            symbol: Some("SOLUSDC"),
            symbols: None,
            time_zone: None,
            r#type: Some("MINI"),
        };

        let ticker_day_mini: TickerDayMiniResponse =
            market_api.get_ticker_day_mini(params).unwrap();

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

        assert!(check_trading_day_mini(&ticker_day_mini, "SOLUSDC"));
    }

    #[test]
    fn test_get_ticker_day_full() {
        let market_api = shared_test_market();
        let params = TickerDayParams {
            symbol: Some("DOTUSDC"),
            symbols: None,
            time_zone: None,
            r#type: Some("FULL"),
        };

        let ticker_day_full: TickerDayFullResponse =
            market_api.get_ticker_day_full(params).unwrap();

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
        assert!(check_ticker_day_full(&ticker_day_full, "DOTUSDC"));
    }

    #[test]
    fn test_get_ticker_day_mini_list() {
        let market_api = shared_test_market();
        let symbols = vec!["BTCUSDC", "SOLUSDC"];
        let params = TickerDayParams {
            symbol: None,
            symbols: Some("[\"BTCUSDC\",\"SOLUSDC\"]"),
            time_zone: None,
            r#type: Some("MINI"),
        };

        let ticker_day_mini_list: Vec<TickerDayMiniResponse> =
            market_api.get_ticker_day_mini_list(params).unwrap();

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

        assert!(
            ticker_day_mini_list
                .into_iter()
                .zip(symbols)
                .collect::<Vec<(TickerDayMiniResponse, &str)>>()
                .iter()
                .all(|(td, s)| check_trading_day_mini(td, s))
        );
    }

    #[test]
    fn test_get_ticker_day_full_list() {
        let market_api = shared_test_market();
        let symbols = vec!["BTCUSDC", "SOLUSDC"];
        let params = TickerDayParams {
            symbol: None,
            symbols: Some("[\"BTCUSDC\",\"SOLUSDC\"]"),
            time_zone: None,
            r#type: Some("FULL"),
        };

        let ticker_day_mini_list: Vec<TickerDayFullResponse> =
            market_api.get_ticker_day_full_list(params).unwrap();

        let check_trading_day_mini = |ticker_day: &TickerDayFullResponse, symbol: &str| {
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

        assert!(
            ticker_day_mini_list
                .into_iter()
                .zip(symbols)
                .collect::<Vec<(TickerDayFullResponse, &str)>>()
                .iter()
                .all(|(td, s)| check_trading_day_mini(td, s))
        );
    }
}
