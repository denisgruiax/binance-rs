#[cfg(test)]
mod general_integration {
    use binance_common::endpoint::host::Host;
    use binance_common::endpoint::route::General;
    use binance_common::model::params::{EmptyParams, general::ExchangeInformationParams};
    use binance_common::model::response::general::{
        EmptyResponse, ExchangeInformationResponse, ServerTimeResponse,
    };
    use binance_core::client::asynchronous::Client;
    use binance_core::client::signer::hmacsha256::HmacSha256;

    #[tokio::test]
    async fn test_ping() {
        let client = Client::new(
            &Host::Api,
            HmacSha256::new("api_key", "secret_key"),
        );

        let response = client.get(&General::Ping, EmptyParams);
        let body = response.await.unwrap().text().await.unwrap();
        let empty: EmptyResponse = serde_json::from_str(&body).unwrap();

        assert_eq!(EmptyResponse {}, empty);
    }

    #[tokio::test]
    async fn test_server_time() {
        let client = Client::new(
            &Host::Api,
            HmacSha256::new("api_key", "secret_key"),
        );

        let response = client.get(&General::ServerTime, EmptyParams);
        let body = response.await.unwrap().text().await.unwrap();
        let server_time: ServerTimeResponse = serde_json::from_str(&body).unwrap();

        assert!(server_time.server_time > 0);
    }

    #[tokio::test]
    async fn test_exchange_info() {
        let client = Client::new(
            &Host::Api,
            HmacSha256::new("api_key", "secret_key"),
        );

        let params = ExchangeInformationParams {
            symbol: Some("BTCUSDC"),
            symbols: None,
            permissions: None,
        };

        let response = client.get(&General::ExchangeInfo, params);
        let body = response.await.unwrap().text().await.unwrap();
        let exchange_information: ExchangeInformationResponse =
            serde_json::from_str(&body).unwrap();

        assert_eq!(exchange_information.timezone, "UTC");
        assert!(exchange_information.server_time > 0);

        assert_eq!(exchange_information.symbols[0].symbol, "BTCUSDC");
        assert_eq!(exchange_information.symbols[0].status, "TRADING");
        assert_eq!(exchange_information.symbols[0].base_asset, "BTC");
        assert_eq!(exchange_information.symbols[0].quote_asset, "USDC");
        assert_eq!(exchange_information.symbols[0].base_asset_precision, 8);
        assert!(exchange_information.symbols[0].order_types.len() == 7);
        assert!(exchange_information.symbols[0].iceberg_allowed);
        assert!(exchange_information.symbols[0].is_spot_trading_allowed);
        assert!(exchange_information.symbols[0].is_margin_trading_allowed);
    }
}
