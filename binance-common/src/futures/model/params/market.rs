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
