#[cfg(test)]
mod futures_account_api_integration_tests {
    use std::sync::{Arc, OnceLock};

    use binance_common::futures::{
        endpoint::host::Host,
        model::{
            params::account::{
                CommissionRateParams, FuturesAccountParams, FuturesBalanceParams,
                PositionSideParams,
            },
            response::account::{CommissionRateResponse, FuturesBalanceResponse},
        },
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
            account_api.get_futures_balance(&params).unwrap();

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
            account_api.get_futures_balance_v3(&params).unwrap();

        let wallet = futures_balance
            .iter()
            .find(|wallet| wallet.asset == "BNFCR")
            .unwrap();

        assert!(wallet.update_time > 0);
    }

    #[test]
    fn test_get_position_side() {
        let account_api: Arc<AccountApi<HmacSha256<'static>>> = shared_test_client::<HmacSha256>();

        let params: PositionSideParams = PositionSideParams::default();

        let position_side = account_api.get_position_side(&params).unwrap();

        assert!(!position_side.dual_side_position);
    }

    #[test]
    fn test_get_futures_account() {
        let account_api: Arc<AccountApi<HmacSha256<'static>>> = shared_test_client::<HmacSha256>();

        let params: FuturesAccountParams = FuturesAccountParams::default();

        let futures_account = account_api.get_futures_account(&params).unwrap();

        assert!(futures_account.total_cross_wallet_balance > 0.0);
    }

    #[test]
    fn test_get_comission_rate() {
        let account_api: Arc<AccountApi<HmacSha256<'static>>> = shared_test_client::<HmacSha256>();

        let params: CommissionRateParams = CommissionRateParams::new("SOLUSDT");

        let comission_rates: CommissionRateResponse =
            account_api.get_comission_rate(&params).unwrap();

        assert_eq!(comission_rates.symbol, "SOLUSDT");
    }
}
