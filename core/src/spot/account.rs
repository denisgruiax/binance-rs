use binance_api::{
    endpoint::route::Account,
    model::{
        BinanceError,
        params::account::{InfoParams, MyTradesParams},
        response::account::{InfoResponse, MyTradesResponse},
    },
};

use crate::client::{signer::signature::Signature, synchronous::Client};

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
            .get_signed::<InfoResponse>(Account::Info.as_ref(), params)
    }

    pub fn get_my_trades(
        &self,
        params: MyTradesParams,
    ) -> Result<Vec<MyTradesResponse>, BinanceError> {
        self.client
            .get_signed::<Vec<MyTradesResponse>>(Account::MyTrades.as_ref(), params)
    }
}

#[cfg(test)]
mod market_api {
    use std::sync::{Arc, OnceLock};

    use super::*;
    use crate::client::{signer::hmacsha256::HmacSha256, synchronous::Client};
    use crate::spot::secret::{API_KEY, SECRET_KEY};
    use binance_api::endpoint::host::Host;

    static CLIENT: OnceLock<Arc<AccountApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_account() -> Arc<AccountApi<'static, HmacSha256<'static>>> {
        CLIENT
            .get_or_init(|| {
                Arc::new(AccountApi::new(Client::new(
                    Host::Api.as_ref(),
                    HmacSha256::new(API_KEY, SECRET_KEY),
                )))
            })
            .clone()
    }

    #[test]
    fn test_get_info() {
        let account_api = shared_test_account();
        let params = InfoParams {
            omit_zero_balances: Some(true),
            recv_window: Some(5000),
        };

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
        let params = MyTradesParams {
            symbol: "BNBUSDC",
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
        };

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
}
