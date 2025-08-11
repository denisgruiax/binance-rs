use binance_common::{
    error::BinanceError,
    futures::{
        endpoint::route::Account,
        model::{
            params::account::{
                CommissionRateParams, FuturesAccountParams, FuturesBalanceParams,
                IncomeHistoryParams, PositionSideParams,
            },
            response::account::{
                CommissionRateResponse, FuturesAccountResponse, FuturesBalanceResponse,
                IncomeHistoryResponse, PositionSideResponse,
            },
        },
    },
};
use binance_core::{client::asynchronous::Client, signer::signature::Signature};
use reqwest::Method;

pub struct AccountApi<'a, S>
where
    S: Signature<'a>,
{
    pub client: Client<'a, S>,
}

impl<'a, S> AccountApi<'a, S>
where
    S: Signature<'a>,
{
    pub fn new(client: Client<'a, S>) -> Self {
        AccountApi { client }
    }

    pub async fn get_futures_balance(
        &self,
        params: &FuturesBalanceParams,
    ) -> Result<Vec<FuturesBalanceResponse>, BinanceError> {
        self.client
            .send(Account::FuturesBalance, params, Method::GET)
            .await
    }

    pub async fn get_futures_balance_v3(
        &self,
        params: &FuturesBalanceParams,
    ) -> Result<Vec<FuturesBalanceResponse>, BinanceError> {
        self.client
            .send(Account::FuturesBalanceV3, params, Method::GET)
            .await
    }

    pub async fn get_position_side(
        &self,
        params: &PositionSideParams,
    ) -> Result<PositionSideResponse, BinanceError> {
        self.client
            .send(Account::PositionSide, params, Method::GET)
            .await
    }

    pub async fn get_futures_account(
        &self,
        params: &FuturesAccountParams,
    ) -> Result<FuturesAccountResponse, BinanceError> {
        self.client
            .send(Account::FuturesAccount, params, Method::GET)
            .await
    }

    pub async fn get_comission_rate(
        &self,
        params: &CommissionRateParams<'a>,
    ) -> Result<CommissionRateResponse, BinanceError> {
        self.client
            .send(Account::CommissionRate, params, Method::GET)
            .await
    }

    pub async fn get_income_history(
        &self,
        params: &IncomeHistoryParams<'a>,
    ) -> Result<Vec<IncomeHistoryResponse>, BinanceError> {
        self.client
            .send(Account::IncomeHistory, params, Method::GET)
            .await
    }
}
