#[cfg(test)]
mod spot_account_api_integration_tests {
    use binance_common::spot::endpoint::host::Host;
    use binance_common::spot::model::params::account::{
        InfoParams, MyTradesParams, UnfilledOrderCountParams,
    };
    use binance_common::spot::model::response::account::{
        InfoResponse, MyTradesResponse, UnfilledOrderCountResponse,
    };
    use binance_core::{client::synchronous::Client, signer::hmacsha256::HmacSha256};
    use binance_spot::{
        account::AccountApi,
        secret::{API_KEY, SECRET_KEY},
    };
    use std::sync::{Arc, OnceLock};
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

        let info: InfoResponse = account_api.get_info(&params).unwrap();

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

        let my_trades: Vec<MyTradesResponse> = account_api.get_my_trades(&params).unwrap();

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
        let unffiled_order_count: Vec<UnfilledOrderCountResponse> = account_api
            .get_unfilled_order_count(&UnfilledOrderCountParams::new())
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
