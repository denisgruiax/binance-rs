use crate::client::{signer::signature::Signature, synchronous::Client};
use binance_api::endpoint::host::Host;
use binance_api::endpoint::route::Market;
use binance_api::model::params::market::*;
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
}
