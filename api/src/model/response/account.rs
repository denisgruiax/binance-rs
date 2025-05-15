use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct InfoResponse {
    pub maker_commission: u32,
    pub taker_commission: u32,
    pub buyer_commission: u32,
    pub seller_commission: u32,
    pub commission_rates: ComissionRatesResponse,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub prevent_sor: bool,
    pub update_time: u64,
    pub account_type: String,
    pub balances: Vec<AssetResponse>,
    pub permissions: Vec<String>,
    pub uid: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct ComissionRatesResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub maker: String,

    #[serde_as(as = "DisplayFromStr")]
    pub taker: String,

    #[serde_as(as = "DisplayFromStr")]
    pub buyer: String,

    #[serde_as(as = "DisplayFromStr")]
    pub seller: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct AssetResponse {
    pub asset: String,

    #[serde_as(as = "DisplayFromStr")]
    pub free: String,

    #[serde_as(as = "DisplayFromStr")]
    pub locked: String,
}
