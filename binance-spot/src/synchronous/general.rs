use binance_common::error::BinanceError;
use binance_common::spot::endpoint::route::General;
use binance_common::spot::model::params::{EmptyParams, general::ExchangeInformationParams};
use binance_common::spot::model::response::general::{
    EmptyResponse, ExchangeInformationResponse, ServerTimeResponse,
};
use binance_core::{client::synchronous::Client, signer::signature::Signature};

pub struct GeneralApi<'a, S>
where
    S: Signature<'a>,
{
    client: Client<'a, S>,
}

impl<'a, S> GeneralApi<'a, S>
where
    S: Signature<'a>,
{
    pub fn new(client: Client<'a, S>) -> GeneralApi<'a, S> {
        GeneralApi { client }
    }

    pub fn ping(&self) -> Result<EmptyResponse, BinanceError> {
        self.client.get(General::Ping, EmptyParams)
    }

    pub fn get_server_time(&self) -> Result<ServerTimeResponse, BinanceError> {
        self.client.get(General::ServerTime, EmptyParams)
    }

    pub fn get_exchange_info(
        &self,
        params: &ExchangeInformationParams,
    ) -> Result<ExchangeInformationResponse, BinanceError> {
        self.client.get(General::ExchangeInfo, params)
    }
}
