use api::endpoint::host::Host;
use api::endpoint::route::General;
use api::model::params::EmptyParams;
use api::model::response::general::{EmptyResponse, ServerTimeResponse};
use binance_core::client::asynchronous::Client;
use binance_core::client::signer::hmacsha256::HmacSha256;
use std::sync::Arc;

#[cfg(test)]
mod general {
    use api::model::response;

    use super::*;

    #[tokio::test]
    async fn test_connectivity() {
        let client = Arc::new(Client::new(
            Host::Api.into(),
            HmacSha256::new("api_key", "secret_key"),
        ));

        let response = client.get(General::ExchangeInformation, EmptyParams);
        let body = response.await.unwrap().text().await.unwrap();
        let empty: EmptyResponse = serde_json::from_str(&body).unwrap();

        assert_eq!(EmptyResponse {}, empty);
    }

    #[tokio::test]
    async fn test_server_time() {
        let client = Arc::new(Client::new(
            Host::Api.into(),
            HmacSha256::new("api_key", "secret_key"),
        ));

        let response = client.get(General::CheckServerTime, EmptyParams);
        let body = response.await.unwrap().text().await.unwrap();
        let server_time: ServerTimeResponse = serde_json::from_str(&body).unwrap();

        assert!(server_time.server_time > 0);
    }
}
