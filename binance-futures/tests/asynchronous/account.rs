#[cfg(test)]
mod futures_account_api_integration_tests {
    use binance_common::{
        error::BinanceError,
        futures::{
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
        },
    };
    use binance_core::{client::asynchronous::Client, signer::hmacsha256::HmacSha256};
    use binance_futures::asynchronous::account::AccountApi;

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

    // pub static ED25519_API_KEY: Lazy<String> = Lazy::new(|| {
    //     dotenv().ok(); // load .env if present (only first call counts)
    //     env::var("ED25519_API_KEY").expect("ED25519_API_KEY must be set")
    // });

    // pub static ED25519_PRIVATE_KEY: Lazy<String> = Lazy::new(|| {
    //     dotenv().ok();
    //     let private_key = env::var("ED25519_PRIVATE_KEY").expect("ED25519_PRIVATE_KEY must be set");
    //     println!("{}", private_key);
    //     private_key.replace("\\n", "\n")
    // });

    pub fn new_test_client() -> AccountApi<'static, HmacSha256<'static>> {
        AccountApi::new(Client::new(
            &Host::Api,
            HmacSha256::new(API_KEY.as_str(), SECRET_KEY.as_str()),
        ))
    }

    // pub fn new_test_client2() -> AccountApi<'static, Ed25519Dalek> {
    //     AccountApi::new(Client::new(
    //         &Host::Api,
    //         Ed25519Dalek::new(ED25519_API_KEY.to_string(), ED25519_PRIVATE_KEY.as_str()).unwrap(),
    //     ))
    // }

    #[tokio::test]
    async fn test_get_futures_balance() {
        let account_api: AccountApi<HmacSha256<'static>> = new_test_client();

        let params: FuturesBalanceParams = FuturesBalanceParams::new(5000);

        let futures_balance: Vec<FuturesBalanceResponse> =
            account_api.get_futures_balance(&params).await.unwrap();

        let wallet = futures_balance
            .iter()
            .find(|wallet| wallet.asset == "BNFCR")
            .unwrap();

        assert!(wallet.update_time > 0);
    }

    #[tokio::test]
    async fn test_get_futures_balance_v3() {
        let account_api: AccountApi<HmacSha256<'static>> = new_test_client();

        let params: FuturesBalanceParams = FuturesBalanceParams::new(5000);

        let futures_balance: Vec<FuturesBalanceResponse> =
            account_api.get_futures_balance_v3(&params).await.unwrap();

        let wallet = futures_balance
            .iter()
            .find(|wallet| wallet.asset == "BNFCR")
            .unwrap();

        assert!(wallet.update_time > 0);
    }

    // #[tokio::test]
    // async fn test_get_futures_balance_v3_ed25519() {
    //     let account_api: Arc<AccountApi<Ed25519Dalek>> = new_test_client2::<Ed25519Dalek>();

    //     let params: FuturesBalanceParams = FuturesBalanceParams::new(5000);

    //     let futures_balance: Vec<FuturesBalanceResponse> =
    //         account_api.get_futures_balance_v3(&params).await.unwrap();

    //     let wallet = futures_balance
    //         .iter()
    //         .find(|wallet| wallet.asset == "BNFCR")
    //         .unwrap();

    //     assert!(wallet.update_time > 0);
    // }

    #[tokio::test]
    async fn test_get_position_side() {
        let account_api: AccountApi<HmacSha256<'static>> = new_test_client();

        let params: PositionSideParams = PositionSideParams::default();

        let position_side = account_api.get_position_side(&params).await.unwrap();

        assert!(!position_side.dual_side_position);
    }

    #[tokio::test]
    async fn test_get_futures_account() {
        let account_api: AccountApi<HmacSha256<'static>> = new_test_client();

        let params: FuturesAccountParams = FuturesAccountParams::default();

        let futures_account = account_api.get_futures_account(&params).await;

        assert!(futures_account.is_ok());
    }

    #[tokio::test]
    async fn test_get_comission_rate() {
        let account_api: AccountApi<HmacSha256<'static>> = new_test_client();

        let params: CommissionRateParams = CommissionRateParams::new("SOLUSDT");

        let comission_rates: CommissionRateResponse =
            account_api.get_comission_rate(&params).await.unwrap();

        assert_eq!(comission_rates.symbol, "SOLUSDT");
    }

    #[tokio::test]
    async fn test_income_history() {
        let account_api: AccountApi<HmacSha256<'static>> = new_test_client();

        let income_history: Result<Vec<IncomeHistoryResponse>, BinanceError> = account_api
            .get_income_history(&IncomeHistoryParams::new())
            .await;

        assert!(income_history.is_ok());
    }
}
