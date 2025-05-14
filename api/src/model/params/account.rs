use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoParams {
    pub omit_zero_balances: Option<bool>,
    pub recv_window: Option<u64>,
}
