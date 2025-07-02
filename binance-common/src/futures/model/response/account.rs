use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesBalanceResponse {
    pub account_alias: String,
    pub asset: String,

    #[serde_as(as = "DisplayFromStr")]
    pub balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub cross_wallet_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub cross_un_pnl: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub available_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub max_withdraw_amount: f64,

    pub margin_available: bool,
    pub update_time: u64,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSideResponse {
    pub dual_side_position: bool,
}
