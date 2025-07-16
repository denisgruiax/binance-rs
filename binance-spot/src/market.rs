use binance_common::spot::endpoint::route::Market;
use binance_common::spot::model::params::market::*;
use binance_common::spot::model::response::market::{
    AvgPriceResponse, BookTickerResponse, HistoricalTradesResponse, KlinesResponse,
    PriceTickerResponse, RollingTickerMiniResponse, Ticker24hFullResponse, Ticker24hMiniResponse,
    TickerDayFullResponse, TickerDayMiniResponse, TradesResponse,
};
use binance_common::{error::BinanceError, spot::model::response::market::DepthResponse};
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

    pub fn get_depth(&self, params: &DepthParams) -> Result<DepthResponse, BinanceError> {
        self.client.get(Market::Depth, params)
    }

    pub fn get_trades(&self, params: &TradesParams) -> Result<Vec<TradesResponse>, BinanceError> {
        self.client.get(Market::Trades, params)
    }

    pub fn get_historical_trades(
        &self,
        params: &HistoricalTradesParams,
    ) -> Result<Vec<HistoricalTradesResponse>, BinanceError> {
        self.client.get(Market::HistoricalTrades, params)
    }

    pub fn get_klines(&self, params: &KlinesParams) -> Result<Vec<KlinesResponse>, BinanceError> {
        self.client.get(Market::Klines, params)
    }

    pub fn get_uiklines(&self, params: &KlinesParams) -> Result<Vec<KlinesResponse>, BinanceError> {
        self.client.get(Market::UIKlines, params)
    }

    pub fn get_average_price(
        &self,
        params: &AvgPriceParams,
    ) -> Result<AvgPriceResponse, BinanceError> {
        self.client.get(Market::AvgPrice, params)
    }

    pub fn get_ticker24h_mini(
        &self,
        params: &Ticker24hParams,
    ) -> Result<Ticker24hMiniResponse, BinanceError> {
        self.client.get(Market::Ticker24h, params)
    }

    pub fn get_ticker24h_full(
        &self,
        params: &Ticker24hParams,
    ) -> Result<Ticker24hFullResponse, BinanceError> {
        self.client.get(Market::Ticker24h, params)
    }

    pub fn get_ticker24h_mini_list(
        &self,
        params: &Ticker24hParams,
    ) -> Result<Vec<Ticker24hMiniResponse>, BinanceError> {
        self.client.get(Market::Ticker24h, params)
    }

    pub fn get_ticker24h_full_list(
        &self,
        params: &Ticker24hParams,
    ) -> Result<Vec<Ticker24hFullResponse>, BinanceError> {
        self.client.get(Market::Ticker24h, params)
    }

    pub fn get_ticker_day_mini(
        &self,
        params: &TickerDayParams,
    ) -> Result<TickerDayMiniResponse, BinanceError> {
        self.client.get(Market::TickerDay, params)
    }

    pub fn get_ticker_day_full(
        &self,
        params: &TickerDayParams,
    ) -> Result<TickerDayFullResponse, BinanceError> {
        self.client.get(Market::TickerDay, params)
    }

    pub fn get_ticker_day_mini_list(
        &self,
        params: &TickerDayParams,
    ) -> Result<Vec<TickerDayMiniResponse>, BinanceError> {
        self.client.get(Market::TickerDay, params)
    }

    pub fn get_ticker_day_full_list(
        &self,
        params: &TickerDayParams,
    ) -> Result<Vec<TickerDayFullResponse>, BinanceError> {
        self.client.get(Market::TickerDay, params)
    }

    pub fn get_price_ticker(&self, symbol: &str) -> Result<PriceTickerResponse, BinanceError> {
        let params = PriceTickerParams::new().symbol(symbol);

        self.client.get(Market::TickerPrice, params)
    }

    pub fn get_price_ticker_list(
        &self,
        symbols: &str,
    ) -> Result<Vec<PriceTickerResponse>, BinanceError> {
        let params = PriceTickerParams::new().symbols(symbols);

        self.client.get(Market::TickerPrice, params)
    }

    pub fn get_book_ticker(&self, symbol: &str) -> Result<BookTickerResponse, BinanceError> {
        let params = BookTickerParams::new().symbol(symbol);

        self.client.get(Market::BookTicker, params)
    }

    pub fn get_book_ticker_list(
        &self,
        symbols: &str,
    ) -> Result<Vec<BookTickerResponse>, BinanceError> {
        let params = BookTickerParams::new().symbols(symbols);

        self.client.get(Market::BookTicker, params)
    }

    pub fn get_rolling_ticker_mini(
        &self,
        params: &RollingTickerParams,
    ) -> Result<RollingTickerMiniResponse, BinanceError> {
        self.client.get(Market::RollingTicker, params)
    }

    pub fn get_rolling_ticker_mini_list(
        &self,
        params: &RollingTickerParams,
    ) -> Result<Vec<RollingTickerMiniResponse>, BinanceError> {
        self.client.get(Market::RollingTicker, params)
    }

    pub fn get_rolling_ticker_full(
        &self,
        params: &RollingTickerParams,
    ) -> Result<RollingTickerMiniResponse, BinanceError> {
        self.client.get(Market::RollingTicker, params)
    }

    pub fn get_rolling_ticker_full_list(
        &self,
        params: &RollingTickerParams,
    ) -> Result<Vec<RollingTickerMiniResponse>, BinanceError> {
        self.client.get(Market::RollingTicker, params)
    }
}
