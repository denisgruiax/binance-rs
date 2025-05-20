use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AckResponse {
    symbol: String,
    order_id: u64,
    order_list_id: i64,
    client_order_id: String,
    transact_time: u64,
}
