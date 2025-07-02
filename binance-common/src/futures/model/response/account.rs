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

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub total_initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_maint_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_wallet_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_unrealized_profit: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_margin_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_position_initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_open_order_initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_cross_wallet_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_cross_un_pnl: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub available_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub max_withdraw_amount: f64,

    pub assets: Vec<FuturesAssetBalance>,
    pub positions: Vec<FuturesPosition>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAssetBalance {
    pub asset: String,

    #[serde_as(as = "DisplayFromStr")]
    pub wallet_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub unrealized_profit: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub margin_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub maint_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub position_initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub open_order_initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub cross_wallet_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub cross_un_pnl: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub available_balance: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub max_withdraw_amount: f64,

    pub update_time: u64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesPosition {
    pub symbol: String,
    pub position_side: String,

    #[serde_as(as = "DisplayFromStr")]
    pub position_amt: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub unrealized_profit: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub isolated_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub notional: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub isolated_wallet: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub initial_margin: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub maint_margin: f64,

    pub update_time: u64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRateResponse {
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub maker_commission_rate: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub taker_commission_rate: f64,
}
