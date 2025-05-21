use crate::client::{signer::signature::Signature, synchronous::Client};
use binance_api::endpoint::route::Market;
use binance_api::model::params::market::*;
use binance_api::model::response::market::{
    AvgPriceResponse, BookTickerResponse, HistoricalTradesResponse, KlinesResponse,
    PriceTickerResponse, RollingTickerMiniResponse, Ticker24hFullResponse, Ticker24hMiniResponse,
    TickerDayFullResponse, TickerDayMiniResponse, TradesResponse,
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
        self.client.get(Market::Depth.as_ref(), params)
    }

    pub fn get_trades(&self, params: TradesParams) -> Result<Vec<TradesResponse>, BinanceError> {
        self.client.get(Market::Trades.as_ref(), params)
    }

    pub fn get_historical_trades(
        &self,
        params: HistoricalTradesParams,
    ) -> Result<Vec<HistoricalTradesResponse>, BinanceError> {
        self.client.get(Market::HistoricalTrades.as_ref(), params)
    }

    pub fn get_klines(&self, params: KlinesParams) -> Result<Vec<KlinesResponse>, BinanceError> {
        self.client.get(Market::Klines.as_ref(), params)
    }

    pub fn get_uiklines(&self, params: KlinesParams) -> Result<Vec<KlinesResponse>, BinanceError> {
        self.client.get(Market::UIKlines.as_ref(), params)
    }

    pub fn get_average_price(
        &self,
        params: AvgPriceParams,
    ) -> Result<AvgPriceResponse, BinanceError> {
        self.client.get(Market::AvgPrice.as_ref(), params)
    }

    pub fn get_ticker24h_mini(
        &self,
        params: Ticker24hParams,
    ) -> Result<Ticker24hMiniResponse, BinanceError> {
        self.client.get(Market::Ticker24h.as_ref(), params)
    }

    pub fn get_ticker24h_full(
        &self,
        params: Ticker24hParams,
    ) -> Result<Ticker24hFullResponse, BinanceError> {
        self.client.get(Market::Ticker24h.as_ref(), params)
    }

    pub fn get_ticker24h_mini_list(
        &self,
        params: Ticker24hParams,
    ) -> Result<Vec<Ticker24hMiniResponse>, BinanceError> {
        self.client.get(Market::Ticker24h.as_ref(), params)
    }

    pub fn get_ticker24h_full_list(
        &self,
        params: Ticker24hParams,
    ) -> Result<Vec<Ticker24hFullResponse>, BinanceError> {
        self.client.get(Market::Ticker24h.as_ref(), params)
    }

    pub fn get_ticker_day_mini(
        &self,
        params: TickerDayParams,
    ) -> Result<TickerDayMiniResponse, BinanceError> {
        self.client.get(Market::TickerDay.as_ref(), params)
    }

    pub fn get_ticker_day_full(
        &self,
        params: TickerDayParams,
    ) -> Result<TickerDayFullResponse, BinanceError> {
        self.client.get(Market::TickerDay.as_ref(), params)
    }

    pub fn get_ticker_day_mini_list(
        &self,
        params: TickerDayParams,
    ) -> Result<Vec<TickerDayMiniResponse>, BinanceError> {
        self.client.get(Market::TickerDay.as_ref(), params)
    }

    pub fn get_ticker_day_full_list(
        &self,
        params: TickerDayParams,
    ) -> Result<Vec<TickerDayFullResponse>, BinanceError> {
        self.client.get(Market::TickerDay.as_ref(), params)
    }

    pub fn get_price_ticker(&self, symbol: &str) -> Result<PriceTickerResponse, BinanceError> {
        let params = PriceTickerParams::new().symbol(symbol);

        self.client.get(Market::PriceTicker.as_ref(), params)
    }

    pub fn get_price_ticker_list(
        &self,
        symbols: &str,
    ) -> Result<Vec<PriceTickerResponse>, BinanceError> {
        let params = PriceTickerParams::new().symbols(symbols);

        self.client.get(Market::PriceTicker.as_ref(), params)
    }

    pub fn get_book_ticker(&self, symbol: &str) -> Result<BookTickerResponse, BinanceError> {
        let params = BookTickerParams::new().symbol(symbol);

        self.client.get(Market::BookTicker.as_ref(), params)
    }

    pub fn get_book_ticker_list(
        &self,
        symbols: &str,
    ) -> Result<Vec<BookTickerResponse>, BinanceError> {
        let params = BookTickerParams::new().symbols(symbols);

        self.client.get(Market::BookTicker.as_ref(), params)
    }

    pub fn get_rolling_ticker_mini(
        &self,
        params: RollingTickerParams,
    ) -> Result<RollingTickerMiniResponse, BinanceError> {
        self.client.get(Market::RollingTicker.as_ref(), params)
    }

    pub fn get_rolling_ticker_mini_list(
        &self,
        params: RollingTickerParams,
    ) -> Result<Vec<RollingTickerMiniResponse>, BinanceError> {
        self.client.get(Market::RollingTicker.as_ref(), params)
    }

    pub fn get_rolling_ticker_full(
        &self,
        params: RollingTickerParams,
    ) -> Result<RollingTickerMiniResponse, BinanceError> {
        self.client.get(Market::RollingTicker.as_ref(), params)
    }

    pub fn get_rolling_ticker_full_list(
        &self,
        params: RollingTickerParams,
    ) -> Result<Vec<RollingTickerMiniResponse>, BinanceError> {
        self.client.get(Market::RollingTicker.as_ref(), params)
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
                binance::Interval,
                market::{
                    AvgPriceParams, DepthParams, HistoricalTradesParams, KlinesParams,
                    Ticker24hParams, TickerDayParams, TradesParams,
                },
            },
            response::market::{
                AvgPriceResponse, BookTickerResponse, DepthResponse, HistoricalTradesResponse,
                KlinesResponse, PriceTickerResponse, Ticker24hFullResponse, Ticker24hMiniResponse,
                TickerDayFullResponse, TickerDayMiniResponse, TradesResponse,
            },
        },
    };
    use std::sync::{Arc, OnceLock};

    static CLIENT: OnceLock<Arc<MarketApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_market() -> Arc<MarketApi<'static, HmacSha256<'static>>> {
        CLIENT
            .get_or_init(|| {
                Arc::new(MarketApi::new(Client::new(
                    Host::Api.as_ref(),
                    HmacSha256::new("api_key", "secret_key"),
                )))
            })
            .clone()
    }

    #[test]
    fn test_get_depth() {
        let market_api = shared_test_market();

        let params = DepthParams::new("BTCUSDC").limit(20);

        let depth: DepthResponse = market_api.get_depth(params).unwrap();

        assert!(depth.last_update_id > 0);
        assert_eq!(depth.bids.len(), 20);
        assert_eq!(depth.asks.len(), 20);
    }

    #[test]
    fn test_get_trades() {
        let market_api = shared_test_market();

        let params = TradesParams::new("SOLUSDC").limit(25);

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

        let params = HistoricalTradesParams::new("SOLUSDC").limit(40);

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
        let market_api = shared_test_market();

        let params = KlinesParams::new("ETHUSDC", Interval::Minutes5.as_ref()).limit(30);

        let klines: Vec<KlinesResponse> = market_api.get_klines(params).unwrap();

        assert_eq!(klines.len(), 30);
        assert!(klines.iter().all(check_kline));
    }

    #[test]
    fn test_get_uiklines() {
        let market_api = shared_test_market();

        let params = KlinesParams::new("ETHUSDC", Interval::Minutes5.as_ref()).limit(30);

        let klines: Vec<KlinesResponse> = market_api.get_uiklines(params).unwrap();

        assert_eq!(klines.len(), 30);
        assert!(klines.iter().all(check_kline));
    }

    #[test]
    fn test_get_average_price() {
        let market_api = shared_test_market();
        let params = AvgPriceParams::new("FETUSDC");

        let average_price: AvgPriceResponse = market_api.get_average_price(params).unwrap();

        assert!(average_price.mins > 0);
        assert!(average_price.price > 0.0);
        assert!(average_price.close_time > 0);
    }

    fn check_ticker24h_mini(ticker_statistics: &Ticker24hMiniResponse) -> bool {
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
    }

    fn check_ticker24h_full(ticker_statistics: &Ticker24hFullResponse) -> bool {
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
    }

    #[test]
    fn test_get_ticker24h_mini() {
        let market_api = shared_test_market();

        let params = Ticker24hParams::new().symbol("BTCUSDC").r#type("MINI");

        let ticker24h_mini: Ticker24hMiniResponse = market_api.get_ticker24h_mini(params).unwrap();

        assert_eq!(ticker24h_mini.symbol, "BTCUSDC");
        assert!(check_ticker24h_mini(&ticker24h_mini));
    }

    #[test]
    fn test_get_ticker24h_full() {
        let market_api = shared_test_market();
        let params = Ticker24hParams::new().symbol("DOTUSDC").r#type("FULL");

        let ticker24h_full: Ticker24hFullResponse = market_api.get_ticker24h_full(params).unwrap();

        assert_eq!(ticker24h_full.symbol, "DOTUSDC");
        assert!(check_ticker24h_full(&ticker24h_full));
    }

    #[test]
    fn test_get_ticker24h_mini_list() {
        let market_api = shared_test_market();
        let params = Ticker24hParams::new()
            .symbols("[\"BTCUSDC\",\"SOLUSDC\"]")
            .r#type("MINI");

        let ticker24h_mini_list: Vec<Ticker24hMiniResponse> =
            market_api.get_ticker24h_mini_list(params).unwrap();

        assert_eq!(ticker24h_mini_list[0].symbol, "BTCUSDC");
        assert_eq!(ticker24h_mini_list[1].symbol, "SOLUSDC");
        assert!(ticker24h_mini_list.iter().all(check_ticker24h_mini));
    }

    #[test]
    fn test_get_ticker24h_full_list() {
        let market_api = shared_test_market();
        let params = Ticker24hParams::new()
            .symbols("[\"BTCUSDC\",\"SOLUSDC\"]")
            .r#type("FULL");

        let ticker24h_full_list: Vec<Ticker24hFullResponse> =
            market_api.get_ticker24h_full_list(params).unwrap();

        assert_eq!(ticker24h_full_list[0].symbol, "BTCUSDC");
        assert_eq!(ticker24h_full_list[1].symbol, "SOLUSDC");
        assert!(ticker24h_full_list.iter().all(check_ticker24h_full));
    }

    fn check_trading_day_mini(ticker_day: &TickerDayMiniResponse, symbol: &str) -> bool {
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
    }

    fn check_ticker_day_full(ticker_day: &TickerDayFullResponse, symbol: &str) -> bool {
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
    }

    #[test]
    fn test_get_ticker_day_mini() {
        let market_api = shared_test_market();
        let params = TickerDayParams::new().symbol("SOLUSDC").r#type("MINI");

        let ticker_day_mini: TickerDayMiniResponse =
            market_api.get_ticker_day_mini(params).unwrap();

        assert!(check_trading_day_mini(&ticker_day_mini, "SOLUSDC"));
    }

    #[test]
    fn test_get_ticker_day_full() {
        let market_api = shared_test_market();
        let params = TickerDayParams::new().symbol("DOTUSDC").r#type("FULL");

        let ticker_day_full: TickerDayFullResponse =
            market_api.get_ticker_day_full(params).unwrap();

        assert!(check_ticker_day_full(&ticker_day_full, "DOTUSDC"));
    }

    #[test]
    fn test_get_ticker_day_mini_list() {
        let market_api = shared_test_market();
        let symbols = vec!["BTCUSDC", "SOLUSDC"];
        let params = TickerDayParams::new()
            .symbols("[\"BTCUSDC\",\"SOLUSDC\"]")
            .r#type("MINI");

        let ticker_day_mini_list: Vec<TickerDayMiniResponse> =
            market_api.get_ticker_day_mini_list(params).unwrap();

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
        let params = TickerDayParams::new()
            .symbols("[\"BTCUSDC\",\"SOLUSDC\"]")
            .r#type("FULL");

        let ticker_day_mini_list: Vec<TickerDayFullResponse> =
            market_api.get_ticker_day_full_list(params).unwrap();

        assert!(
            ticker_day_mini_list
                .into_iter()
                .zip(symbols)
                .collect::<Vec<(TickerDayFullResponse, &str)>>()
                .iter()
                .all(|(td, s)| check_ticker_day_full(td, s))
        );
    }

    #[test]
    fn test_get_price_ticker() {
        let market_api = shared_test_market();

        let egld_usdc: PriceTickerResponse = market_api.get_price_ticker("EGLDUSDC").unwrap();

        assert_eq!(egld_usdc.symbol, "EGLDUSDC");
        assert!(egld_usdc.price > 0.0);
    }

    #[test]
    fn test_get_price_ticker_list() {
        let market_api = shared_test_market();

        let price_ticker_list: Vec<PriceTickerResponse> = market_api
            .get_price_ticker_list("[\"BTCUSDC\",\"SOLUSDC\"]")
            .unwrap();

        assert!(price_ticker_list.iter().all(|p| p.price > 0.0));
    }

    fn check_book_ticker(book_ticker: &BookTickerResponse, symbol: &str) -> bool {
        book_ticker.symbol == symbol
            && book_ticker.bid_price > 0.0
            && book_ticker.bid_qty > 0.0
            && book_ticker.ask_price > 0.0
            && book_ticker.ask_qty > 0.0
    }

    #[test]
    fn test_get_book_ticker() {
        let market_api = shared_test_market();

        let book_ticker: BookTickerResponse = market_api.get_book_ticker("EGLDUSDC").unwrap();

        assert!(check_book_ticker(&book_ticker, "EGLDUSDC"));
    }

    #[test]
    fn test_get_book_ticker_list() {
        let market_api = shared_test_market();

        let book_ticker: Vec<BookTickerResponse> = market_api
            .get_book_ticker_list("[\"BTCUSDC\",\"SOLUSDC\"]")
            .unwrap();

        assert!(check_book_ticker(&book_ticker[0], "BTCUSDC"));
        assert!(check_book_ticker(&book_ticker[1], "SOLUSDC"));
    }
}
