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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineCandlestickDataParams<'a> {
    pub symbol: &'a str,
    pub interval: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}
