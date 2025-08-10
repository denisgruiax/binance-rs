#[cfg(test)]
mod futures_account_api_integration_tests {
    use std::sync::{Arc, OnceLock};

    use binance_common::futures::{
        endpoint::host::Host,
        model::{
            params::account::{
                CommissionRateParams, FuturesAccountParams, FuturesBalanceParams,
                IncomeHistoryParams, PositionSideParams,
            },
            response::account::{
                CommissionRateResponse, FuturesBalanceResponse, IncomeHistoryResponse,
            },
        },
    };
    use binance_core::{
        client::synchronous::Client,
        signer::{ed25519::Ed25519Dalek, hmacsha256::HmacSha256},
    };
    use binance_futures::account::AccountApi;

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
        println!("{}", private_key);
        private_key.replace("\\n", "\n")
    });

    static CLIENT: OnceLock<Arc<AccountApi<'static, HmacSha256<'static>>>> = OnceLock::new();

    fn shared_test_client<'a, S>() -> Arc<AccountApi<'static, HmacSha256<'static>>> {
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

    fn shared_test_client2<'a, S>() -> Arc<AccountApi<'static, Ed25519Dalek>> {
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
    fn test_get_futures_balance_v3_ed25519() {
        let account_api: Arc<AccountApi<Ed25519Dalek>> = shared_test_client2::<Ed25519Dalek>();

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

    #[test]
    fn test_income_history() {
        let account_api: Arc<AccountApi<HmacSha256<'static>>> = shared_test_client::<HmacSha256>();

        let income_history: Vec<IncomeHistoryResponse> = account_api
            .get_income_history(&IncomeHistoryParams::new())
            .unwrap();

        assert!(income_history.len() > 0)
    }
}
