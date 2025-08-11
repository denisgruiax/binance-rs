use binance_common::error::BinanceError;
use binance_common::futures::endpoint::route::Market;
use binance_common::futures::model::params::market::{
    BasisParams, FundingRateHistoryParams, GlobalLongShortAccountRatioParams,
    OpenInterestHistoryParams, Pair, Symbol, TakerBuySellVolumeParams,
    TopLongShortAccountRatioParams, TopLongShortPositionRatioParams,
};
use binance_common::futures::model::response::market::{
    BasisResponse, BookTickerResponse, DeliveryPriceResponse, DepthResponse,
    FundingRateHistoryResponse, GlobalLongShortAccountRatioResponse, HistoricalTradesResponse,
    MarkPriceResponse, OpenInterestHistoryResponse, OpenInterestResponse,
    TakerBuySellVolumeResponse, Ticker24hResponse, TickerPriceResponse,
    TopLongShortAccountRatioResponse, TopLongShortPositionRatioResponse, TradesResponse,
};
use binance_common::futures::model::{
    params::market::{
        DepthParams, EmptyParams, ExchangeInformationParams, HistoricalTradesParams, TradesParams,
    },
    response::market::{EmptyResponse, ExchangeInformationResponse, ServerTimeResponse},
};
use binance_common::spot::model::params::market::KlinesParams;
use binance_common::spot::model::response::market::KlinesResponse;
use binance_core::{client::asynchronous::Client, signer::signature::Signature};
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

    pub async fn ping(&self) -> Result<EmptyResponse, BinanceError> {
        self.client.get(Market::Ping, EmptyParams).await
    }

    pub async fn get_server_time(&self) -> Result<ServerTimeResponse, BinanceError> {
        self.client.get(Market::ServerTime, EmptyParams).await
    }

    pub async fn get_exchange_info(
        &self,
        params: &ExchangeInformationParams<'a>,
    ) -> Result<ExchangeInformationResponse, BinanceError> {
        self.client.get(Market::ExchangeInfo, params).await
    }

    pub async fn get_depth(&self, params: &DepthParams<'a>) -> Result<DepthResponse, BinanceError> {
        self.client.get(Market::Depth, params).await
    }

    pub async fn get_trades(
        &self,
        params: &TradesParams<'a>,
    ) -> Result<Vec<TradesResponse>, BinanceError> {
        self.client.get(Market::Trades, params).await
    }

    pub async fn get_historical_trades(
        &self,
        params: &HistoricalTradesParams<'a>,
    ) -> Result<Vec<HistoricalTradesResponse>, BinanceError> {
        self.client
            .send(Market::HistoricalTrades, params, Method::GET)
            .await
    }

    pub async fn get_klines(
        &self,
        params: &KlinesParams<'a>,
    ) -> Result<Vec<KlinesResponse>, BinanceError> {
        self.client.get(Market::Klines, params).await
    }

    pub async fn get_mark_price(
        &self,
        params: &Symbol<'a>,
    ) -> Result<MarkPriceResponse, BinanceError> {
        self.client.get(Market::MarkPrice, params).await
    }

    pub async fn get_mark_price_list(&self) -> Result<Vec<MarkPriceResponse>, BinanceError> {
        self.client.get(Market::MarkPrice, Symbol::default()).await
    }

    pub async fn get_funding_rate_history(
        &self,
        params: &FundingRateHistoryParams<'a>,
    ) -> Result<Vec<FundingRateHistoryResponse>, BinanceError> {
        self.client.get(Market::FundingRateHistory, params).await
    }

    pub async fn get_ticker24h(
        &self,
        params: &Symbol<'a>,
    ) -> Result<Ticker24hResponse, BinanceError> {
        self.client.get(Market::Ticker24h, params).await
    }

    pub async fn get_ticker24h_list(&self) -> Result<Vec<Ticker24hResponse>, BinanceError> {
        self.client.get(Market::Ticker24h, Symbol::default()).await
    }

    pub async fn get_ticker_price(
        &self,
        params: &Symbol<'a>,
    ) -> Result<TickerPriceResponse, BinanceError> {
        self.client.get(Market::TickerPrice, params).await
    }

    pub async fn get_ticker_price_list(&self) -> Result<Vec<TickerPriceResponse>, BinanceError> {
        self.client
            .get(Market::TickerPrice, Symbol::default())
            .await
    }

    pub async fn get_ticker_price_v2(
        &self,
        params: &Symbol<'a>,
    ) -> Result<TickerPriceResponse, BinanceError> {
        self.client.get(Market::TickerPriceV2, params).await
    }

    pub async fn get_ticker_price_v2_list(&self) -> Result<Vec<TickerPriceResponse>, BinanceError> {
        self.client
            .get(Market::TickerPriceV2, Symbol::default())
            .await
    }

    pub async fn get_book_ticker(
        &self,
        params: &Symbol<'a>,
    ) -> Result<BookTickerResponse, BinanceError> {
        self.client.get(Market::BookTicker, params).await
    }

    pub async fn get_book_ticker_list(&self) -> Result<Vec<BookTickerResponse>, BinanceError> {
        self.client.get(Market::BookTicker, Symbol::default()).await
    }

    pub async fn get_delivery_price_list(
        &self,
        params: &Pair<'a>,
    ) -> Result<Vec<DeliveryPriceResponse>, BinanceError> {
        self.client.get(Market::DeliveryPrice, params).await
    }

    pub async fn get_open_interest(
        &self,
        params: &Symbol<'a>,
    ) -> Result<OpenInterestResponse, BinanceError> {
        self.client.get(Market::OpenInterest, params).await
    }

    pub async fn get_open_interest_history_list(
        &self,
        params: &OpenInterestHistoryParams<'a>,
    ) -> Result<Vec<OpenInterestHistoryResponse>, BinanceError> {
        self.client.get(Market::OpenInterestHistory, params).await
    }

    pub async fn get_top_long_position_ratio_list(
        &self,
        params: &TopLongShortPositionRatioParams<'a>,
    ) -> Result<Vec<TopLongShortPositionRatioResponse>, BinanceError> {
        self.client
            .get(Market::TopLongShortPositionRatio, params)
            .await
    }

    pub async fn get_top_long_account_ratio_list(
        &self,
        params: &TopLongShortAccountRatioParams<'a>,
    ) -> Result<Vec<TopLongShortAccountRatioResponse>, BinanceError> {
        self.client
            .get(Market::TopLongShortAccountRatio, params)
            .await
    }

    pub async fn get_global_long_account_ratio_list(
        &self,
        params: &GlobalLongShortAccountRatioParams<'a>,
    ) -> Result<Vec<GlobalLongShortAccountRatioResponse>, BinanceError> {
        self.client
            .get(Market::GlobalLongShortAccountRatio, params)
            .await
    }

    pub async fn get_taker_buy_sell_volume_list(
        &self,
        params: &TakerBuySellVolumeParams<'a>,
    ) -> Result<Vec<TakerBuySellVolumeResponse>, BinanceError> {
        self.client.get(Market::TakerBuySellVolume, params).await
    }

    pub async fn get_basis_list(
        &self,
        params: &BasisParams<'a>,
    ) -> Result<Vec<BasisResponse>, BinanceError> {
        self.client.get(Market::Basis, params).await
    }
}
