use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoParams {
    pub omit_zero_balances: Option<bool>,
    pub recv_window: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTradesParams<'a> {
    pub symbol: &'a str,
    pub order_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>,
    pub limit: Option<u16>,
    pub recv_window: Option<u16>,
}
