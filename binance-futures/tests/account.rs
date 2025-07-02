#[cfg(test)]
mod futures_account_api_integration_tests {
    use std::sync::{Arc, OnceLock};

    use binance_common::futures::{
        endpoint::host::Host,
        model::{params::account::FuturesBalanceParams, response::account::FuturesBalanceResponse},
    };
    use binance_core::{client::synchronous::Client, signer::hmacsha256::HmacSha256};
    use binance_futures::{
        account::AccountApi,
        secret::{API_KEY, SECRET_KEY},
    };

    static CLIENT: OnceLock<Arc<AccountApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_client<'a, S>() -> Arc<AccountApi<'static, HmacSha256<'static>>> {
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
    fn test_get_futures_balance() {
        let account_api: Arc<AccountApi<HmacSha256<'static>>> = shared_test_client::<HmacSha256>();

        let params: FuturesBalanceParams = FuturesBalanceParams::new(5000);

        let futures_balance: Vec<FuturesBalanceResponse> =
            account_api.get_futures_balance(params).unwrap();

        let wallet = futures_balance
            .iter()
            .find(|wallet| wallet.asset == "BNFCR")
            .unwrap();

        assert!(wallet.update_time > 0);
    }

    #[test]
    fn test_get_futures_balance_v3() {
        let account_api: Arc<AccountApi<HmacSha256<'static>>> = shared_test_client::<HmacSha256>();

        let params: FuturesBalanceParams = FuturesBalanceParams::new(5000);

        let futures_balance: Vec<FuturesBalanceResponse> =
            account_api.get_futures_balance_v3(params).unwrap();

        let wallet = futures_balance
            .iter()
            .find(|wallet| wallet.asset == "BNFCR")
            .unwrap();

        assert!(wallet.update_time > 0);
    }
}
