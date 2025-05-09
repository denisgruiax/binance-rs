use crate::client::{signer::signature::Signature, synchronous::Client};
use binance_api::endpoint::route::Market;
use binance_api::model::params::market::*;
use binance_api::model::response::market::{HistoricalTradesResponse, TradesResponse};
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

    pub fn get_depth(&self, params: DepthParams) -> Result<DepthResponse, BinanceError> {
        self.client.get(Market::Depth.into(), params)
    }

    pub fn get_trades(&self, params: TradesParams) -> Result<Vec<TradesResponse>, BinanceError> {
        self.client.get(Market::Trades.into(), params)
    }

    pub fn get_historical_trades(
        &self,
        params: HistoricalTradesParams,
    ) -> Result<Vec<HistoricalTradesResponse>, BinanceError> {
        self.client.get(Market::HistoricalTrades.into(), params)
    }
}

#[cfg(test)]
mod market_api {
    use super::MarketApi;
    use crate::client::{signer::hmacsha256::HmacSha256, synchronous::Client};
    use binance_api::{
        endpoint::host::Host,
        model::{
            params::market::{DepthParams, HistoricalTradesParams, TradesParams},
            response::market::{DepthResponse, HistoricalTradesResponse, TradesResponse},
        },
    };
    use std::sync::{Arc, OnceLock};

    static CLIENT: OnceLock<Arc<MarketApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_market() -> Arc<MarketApi<'static, HmacSha256<'static>>> {
        CLIENT
            .get_or_init(|| {
                Arc::new(MarketApi::new(Client::new(
                    Host::Api.into(),
                    HmacSha256::new("api_key", "secret_key"),
                )))
            })
            .clone()
    }

    #[test]
    fn test_depth() {
        let market_api = shared_test_market();
        let params = DepthParams {
            symbol: "BTCUSDC",
            limit: Some(20),
        };

        let depth: DepthResponse = market_api.get_depth(params).unwrap();

        assert!(depth.last_update_id > 0);
        assert_eq!(depth.bids.len(), 20);
        assert_eq!(depth.asks.len(), 20);
    }

    #[test]
    fn test_trades() {
        let market_api = shared_test_market();
        let params = TradesParams {
            symbol: "SOLUSDC",
            limit: Some(25),
        };

        let trades: Vec<TradesResponse> = market_api.get_trades(params).unwrap();

        let check_trade = |trade: &TradesResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(trades.len(), 25);
        assert!(trades.iter().all(check_trade));
    }

    #[test]
    fn test_get_historical_trades(){
        let market_api = shared_test_market();
        let params = HistoricalTradesParams{
            symbol: "SOLUSDC",
            limit: Some(40),
            from_id: None,
        };

        let trades: Vec<HistoricalTradesResponse> = market_api.get_historical_trades(params).unwrap();

        let check_trade = |trade: &HistoricalTradesResponse| {
            trade.id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.quote_qty > 0.0
                && trade.time > 0
        };

        assert_eq!(trades.len(), 40);
        assert!(trades.iter().all(check_trade));
    }
}
