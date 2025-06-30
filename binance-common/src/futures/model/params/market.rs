pub use crate::spot::model::params::market::{
    DepthParams, HistoricalTradesParams, KlinesParams, TradesParams,
};
pub use crate::spot::model::params::{EmptyParams, general::ExchangeInformationParams};
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct MarkPriceParams<'a> {
    pub symbol: Option<&'a str>,
}

impl<'a> MarkPriceParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        MarkPriceParams {
            symbol: Some(symbol),
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }
}
