use binance_common::{
    error::BinanceError,
    futures::{
        endpoint::route::Account,
        model::{
            params::account::{FuturesBalanceParams, PositionSideParams},
            response::account::{FuturesBalanceResponse, PositionSideResponse},
        },
    },
};
use binance_core::{client::synchronous::Client, signer::signature::Signature};
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

    pub fn get_futures_balance(
        &self,
        params: FuturesBalanceParams,
    ) -> Result<Vec<FuturesBalanceResponse>, BinanceError> {
        self.client
            .send(Account::FuturesBalance, params, Method::GET)
    }

    pub fn get_futures_balance_v3(
        &self,
        params: FuturesBalanceParams,
    ) -> Result<Vec<FuturesBalanceResponse>, BinanceError> {
        self.client
            .send(Account::FuturesBalanceV3, params, Method::GET)
    }

    pub fn get_position_side(
        &self,
        params: PositionSideParams,
    ) -> Result<PositionSideResponse, BinanceError> {
        self.client.send(Account::PositionSide, params, Method::GET)
    }
}
