use binance_common::error::BinanceError;
use binance_common::spot::endpoint::route::General;
use binance_common::spot::model::params::{EmptyParams, general::ExchangeInformationParams};
use binance_common::spot::model::response::general::{
    EmptyResponse, ExchangeInformationResponse, ServerTimeResponse,
};
use binance_core::{client::asynchronous::Client, signer::signature::Signature};

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
    pub async fn new(client: Client<'a, S>) -> GeneralApi<'a, S> {
        GeneralApi { client }
    }

    pub async fn ping(&self) -> Result<EmptyResponse, BinanceError> {
        self.client.get(General::Ping, EmptyParams).await
    }

    pub async fn get_server_time(&self) -> Result<ServerTimeResponse, BinanceError> {
        self.client.get(General::ServerTime, EmptyParams).await
    }

    pub async fn get_exchange_info(
        &self,
        params: &ExchangeInformationParams<'a>,
    ) -> Result<ExchangeInformationResponse, BinanceError> {
        self.client.get(General::ExchangeInfo, params).await
    }
}
