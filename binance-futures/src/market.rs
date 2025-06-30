use binance_common::error::BinanceError;
use binance_common::futures::endpoint::route::Market;
use binance_common::futures::model::params::market::{
    FundingRateHistoryParams, Ticker24hParams, TickerPriceParams,
};
use binance_common::futures::model::response::market::{
    DepthResponse, FundingRateHistoryResponse, HistoricalTradesResponse, MarkPriceResponse,
    Ticker24hResponse, TickerPriceResponse, TradesResponse,
};
use binance_common::futures::model::{
    params::market::{
        DepthParams, EmptyParams, ExchangeInformationParams, HistoricalTradesParams,
        MarkPriceParams, TradesParams,
    },
    response::market::{EmptyResponse, ExchangeInformationResponse, ServerTimeResponse},
};
use binance_common::spot::model::params::market::KlinesParams;
use binance_common::spot::model::response::market::KlinesResponse;
use binance_core::{client::synchronous::Client, signer::signature::Signature};
use reqwest::Method;

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

    pub fn get_depth(&self, params: DepthParams) -> Result<DepthResponse, BinanceError> {
        self.client.get(Market::Depth, params)
    }

    pub fn get_trades(&self, params: TradesParams) -> Result<Vec<TradesResponse>, BinanceError> {
        self.client.get(Market::Trades, params)
    }

    pub fn get_historical_trades(
        &self,
        params: HistoricalTradesParams,
    ) -> Result<Vec<HistoricalTradesResponse>, BinanceError> {
        self.client
            .send(Market::HistoricalTrades, params, Method::GET)
    }

    pub fn get_klines(&self, params: KlinesParams) -> Result<Vec<KlinesResponse>, BinanceError> {
        self.client.get(Market::Klines, params)
    }

    pub fn get_mark_price(
        &self,
        params: MarkPriceParams,
    ) -> Result<MarkPriceResponse, BinanceError> {
        self.client.get(Market::MarkPrice, params)
    }

    pub fn get_mark_price_list(&self) -> Result<Vec<MarkPriceResponse>, BinanceError> {
        self.client
            .get(Market::MarkPrice, MarkPriceParams::default())
    }

    pub fn get_funding_rate_history(
        &self,
        params: FundingRateHistoryParams,
    ) -> Result<Vec<FundingRateHistoryResponse>, BinanceError> {
        self.client.get(Market::FundingRateHistory, params)
    }

    pub fn get_ticker24h(
        &self,
        params: Ticker24hParams,
    ) -> Result<Ticker24hResponse, BinanceError> {
        self.client.get(Market::Ticker24h, params)
    }

    pub fn get_ticker24h_list(&self) -> Result<Vec<Ticker24hResponse>, BinanceError> {
        self.client
            .get(Market::Ticker24h, Ticker24hParams::default())
    }

    pub fn get_ticker_price(
        &self,
        params: TickerPriceParams,
    ) -> Result<TickerPriceResponse, BinanceError> {
        self.client.get(Market::TickerPrice, params)
    }

    pub fn get_ticker_price_list(&self) -> Result<Vec<TickerPriceResponse>, BinanceError> {
        self.client
            .get(Market::TickerPrice, TickerPriceParams::default())
    }
}
