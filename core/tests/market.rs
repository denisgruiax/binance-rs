#[cfg(test)]
mod market_integration {
    use binance_api::endpoint::route::Market;
    use binance_api::model::params::market::RecentTradeListParams;
    use binance_api::model::response::market::RecentTradeResponse;
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

    #[tokio::test]
    async fn test_recent_trade_list() {
        let params = RecentTradeListParams {
            symbol: "SOLUSDC",
            limit: Some(20),
        };

        let client = Client::new(Host::Api.into(), HmacSha256::new("api_key", "secret_key"));

        let response = client.get(Market::RecentTradeList, params);
        let body = response.await.unwrap().text().await.unwrap();

        let recent_trade_list =
            serde_json::from_str::<Vec<RecentTradeResponse>>(&body).unwrap();

        let check_trade = |trade: &RecentTradeResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(recent_trade_list.len(), 20);
        assert!(recent_trade_list.iter().all(check_trade));
    }
}
