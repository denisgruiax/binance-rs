#[cfg(test)]
mod spot_account_api_integration_tests {
    use binance_common::spot::endpoint::host::Host;
    use binance_common::spot::model::params::account::{
        InfoParams, MyTradesParams, UnfilledOrderCountParams,
    };
    use binance_common::spot::model::response::account::{
        InfoResponse, MyTradesResponse, UnfilledOrderCountResponse,
    };
    use binance_core::signer::ed25519::Ed25519Dalek;
    use binance_core::{client::synchronous::Client, signer::hmacsha256::HmacSha256};
    use binance_spot::synchronous::account::AccountApi;
    use std::sync::{Arc, OnceLock};

    use dotenv::dotenv;
    use once_cell::sync::Lazy;
    use std::env;

    pub static API_KEY: Lazy<String> = Lazy::new(|| {
        dotenv().ok(); // load .env if present (only first call counts)
        env::var("API_KEY").expect("API_KEY must be set")
    });

    pub static SECRET_KEY: Lazy<String> = Lazy::new(|| {
        dotenv().ok();
        env::var("SECRET_KEY").expect("SECRET_KEY must be set")
    });

    pub static ED25519_API_KEY: Lazy<String> = Lazy::new(|| {
        dotenv().ok(); // load .env if present (only first call counts)
        env::var("ED25519_API_KEY").expect("ED25519_API_KEY must be set")
    });

    pub static ED25519_PRIVATE_KEY: Lazy<String> = Lazy::new(|| {
        dotenv().ok();
        let private_key = env::var("ED25519_PRIVATE_KEY").expect("ED25519_PRIVATE_KEY must be set");
        private_key.replace("\\n", "\n")
    });

    static CLIENT: OnceLock<Arc<AccountApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_account() -> Arc<AccountApi<'static, HmacSha256<'static>>> {
        CLIENT
            .get_or_init(|| {
                Arc::new(AccountApi::new(Client::new(
                    &Host::Api,
                    HmacSha256::new(API_KEY.as_str(), SECRET_KEY.as_str()),
                )))
            })
            .clone()
    }

    static CLIENT2: OnceLock<Arc<AccountApi<'static, Ed25519Dalek>>> = OnceLock::new();

    fn shared_test_account2<'a, S>() -> Arc<AccountApi<'static, Ed25519Dalek>> {
        CLIENT2
            .get_or_init(|| {
                Arc::new(AccountApi::new(Client::new(
                    &Host::Api,
                    Ed25519Dalek::new(ED25519_API_KEY.to_string(), ED25519_PRIVATE_KEY.as_str())
                        .unwrap(),
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
    fn test_get_info2() {
        let account_api = shared_test_account2::<Arc<AccountApi<'static, Ed25519Dalek>>>();

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
