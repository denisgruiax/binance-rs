use serde::Serialize;

#[derive(Serialize)]
pub struct OrderBookParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

#[derive(Serialize)]
pub struct RecentTradeListParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OldTradeLookupParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
}
