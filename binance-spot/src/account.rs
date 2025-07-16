use binance_common::error::BinanceError;
use binance_common::spot::{
    endpoint::route::Account,
    model::{
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

    pub fn get_info(&self, params: &InfoParams) -> Result<InfoResponse, BinanceError> {
        self.client
            .send::<InfoResponse>(Account::Info, params, Method::GET)
    }

    pub fn get_my_trades(
        &self,
        params: &MyTradesParams,
    ) -> Result<Vec<MyTradesResponse>, BinanceError> {
        self.client
            .send::<Vec<MyTradesResponse>>(Account::MyTrades, params, Method::GET)
    }

    pub fn get_unfilled_order_count(
        &self,
        params: &UnfilledOrderCountParams,
    ) -> Result<Vec<UnfilledOrderCountResponse>, BinanceError> {
        self.client
            .send(Account::UnfilledOrderCount, params, Method::GET)
    }
}
