pub use crate::spot::model::params::market::{
    DepthParams, HistoricalTradesParams, KlinesParams, TradesParams,
};
pub use crate::spot::model::params::{EmptyParams, general::ExchangeInformationParams};
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct FundingRateHistoryParams<'a> {
    pub symbol: Option<&'a str>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u16>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Symbol<'a> {
    pub symbol: &'a str,
}

impl<'a> Symbol<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Symbol { symbol }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Pair<'a> {
    pub pair: &'a str,
}

impl<'a> Pair<'a> {
    pub fn new(pair: &'a str) -> Self {
        Pair { pair }
    }

    pub fn symbol(mut self, pair: &'a str) -> Self {
        self.pair = pair;
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenInterestHistoryParams<'a> {
    pub symbol: &'a str,
    pub period: &'a str,
    pub limit: Option<u16>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}

impl<'a> OpenInterestHistoryParams<'a> {
    pub fn new(symbol: &'a str, period: &'a impl AsRef<str>) -> Self {
        OpenInterestHistoryParams {
            symbol,
            period: period.as_ref(),
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn period(mut self, period: &'a impl AsRef<str>) -> Self {
        self.period = period.as_ref();
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }
}
