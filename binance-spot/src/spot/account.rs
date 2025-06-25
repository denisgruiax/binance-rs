use binance_common::spot::{
    endpoint::route::Account,
    model::{
        BinanceError,
        params::account::{InfoParams, MyTradesParams, UnfilledOrderCountParams},
        response::account::{InfoResponse, MyTradesResponse, UnfilledOrderCountResponse},
    },
};
use reqwest::Method;

use binance_core::{client::synchronous::Client, signer::signature::Signature};

pub struct AccountApi<'a, S>
where
    S: Signature<'a>,
{
    client: Client<'a, S>,
}

impl<'a, S> AccountApi<'a, S>
where
    S: Signature<'a>,
{
    pub fn new(client: Client<'a, S>) -> Self {
        AccountApi { client }
    }

    pub fn get_info(&self, params: InfoParams) -> Result<InfoResponse, BinanceError> {
        self.client
            .send::<InfoResponse>(Account::Info, params, Method::GET)
    }

    pub fn get_my_trades(
        &self,
        params: MyTradesParams,
    ) -> Result<Vec<MyTradesResponse>, BinanceError> {
        self.client
            .send::<Vec<MyTradesResponse>>(Account::MyTrades, params, Method::GET)
    }

    pub fn get_unfilled_order_count(
        &self,
        params: UnfilledOrderCountParams,
    ) -> Result<Vec<UnfilledOrderCountResponse>, BinanceError> {
        self.client
            .send(Account::UnfilledOrderCount, params, Method::GET)
    }
}

#[cfg(test)]
mod account_api {
    use std::sync::{Arc, OnceLock};

    use super::*;
    use crate::spot::secret::{API_KEY, SECRET_KEY};
    use binance_common::spot::endpoint::host::Host;
    use binance_core::{client::synchronous::Client, signer::hmacsha256::HmacSha256};
    static CLIENT: OnceLock<Arc<AccountApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_account() -> Arc<AccountApi<'static, HmacSha256<'static>>> {
        CLIENT
            .get_or_init(|| {
                Arc::new(AccountApi::new(Client::new(
                    &Host::Api,
                    HmacSha256::new(API_KEY, SECRET_KEY),
                )))
            })
            .clone()
    }

    #[test]
    fn test_get_info() {
        let account_api = shared_test_account();

        let params = InfoParams::new().omit_zero_balances(true).recv_window(5000);

        let info: InfoResponse = account_api.get_info(params).unwrap();

        assert!(info.can_trade);
        assert!(info.can_withdraw);
        assert!(info.can_deposit);
        assert!(info.update_time > 0);
        assert!(info.uid > 0);
    }

    #[test]
    fn test_get_my_trades() {
        let account_api = shared_test_account();

        let params = MyTradesParams::new("BNBUSDC");

        let my_trades = account_api.get_my_trades(params).unwrap();

        let check_trade = |trade: &MyTradesResponse| {
            trade.id > 0
                && trade.order_id > 0
                && trade.price > 0.0
                && trade.qty > 0.0
                && trade.time > 0
        };

        assert!(my_trades.iter().all(check_trade));
    }

    #[test]
    fn test_unfilled_order_count() {
        let account_api = shared_test_account();
        let unffiled_order_count = account_api
            .get_unfilled_order_count(UnfilledOrderCountParams::new())
            .unwrap();

        assert!(
            unffiled_order_count
                .iter()
                .all(|order| order.interval_num > 0
                    && order.limit > 0
                    && (order.count > 0 || order.count == 0))
        )
    }
}
