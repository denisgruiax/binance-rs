#[cfg(test)]
mod market_integration {
    use binance_api::endpoint::route::Market;
    use binance_api::{
        endpoint::host::Host,
        model::{params::market::OrderBookParams, response::market::OrderBookResponse},
    };
    use binance_core::client::{asynchronous::Client, signer::hmacsha256::HmacSha256};

    #[tokio::test]
    async fn test_order_book() {
        let params = OrderBookParams {
            symbol: "EGLDUSDC",
            limit: Some(10),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::OrderBook, params);
        let body = response.await.unwrap().text().await.unwrap();

        let order_book = serde_json::from_str::<OrderBookResponse>(&body).unwrap();

        assert!(order_book.last_update_id > 0);
        assert_eq!(order_book.bids.len(), 10);
        assert_eq!(order_book.asks.len(), 10);
    }
}
