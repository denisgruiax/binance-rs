use crate::client::{signer::signature::Signature, synchronous::Client};
use binance_api::endpoint::host::Host;
use binance_api::endpoint::route::General;
use binance_api::model::BinanceError;
use binance_api::model::params::{EmptyParams, general::ExchangeInformationParams};
use binance_api::model::response::general::{EmptyResponse, ExchangeInformationResponse, ServerTimeResponse};

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
        self.client.get(General::Ping.into(), EmptyParams)
    }

    pub fn server_time(&self) -> Result<ServerTimeResponse, BinanceError> {
        self.client.get(General::ServerTime.into(), EmptyParams)
    }

    pub fn exchange_info(&self) -> Result<ExchangeInformationResponse, BinanceError>{
        self.client.get(General::ExchangeInfo.into(), EmptyParams)
    }
˝}

#[cfg(test)]
mod general_api {
    use binance_api::{endpoint::host::Host, model::response::general::EmptyResponse};

    use super::GeneralApi;
    use crate::client::{signer::hmacsha256::HmacSha256, synchronous::Client};

    #[test]
    fn test_ping() {
        let general_api = GeneralApi::new(Client::new(
            Host::Api.into(),
            HmacSha256::new("api_key", "secret_key"),
        ));

        assert_eq!(general_api.ping().unwrap(), EmptyResponse {});
    }
    #[test]
    fn test_server_time() {
        let general_api = GeneralApi::new(Client::new(
            Host::Api.into(),
            HmacSha256::new("api_key", "secret_key"),
        ));

        assert!(general_api.server_time().unwrap().server_time > 0);
    }
}
