use serde::Serialize;

#[derive(Serialize)]
pub struct DepthParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

#[derive(Serialize)]
pub struct TradesParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTradesParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlinesParams<'a> {
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

#[derive(Serialize)]
pub struct AvgPriceParams<'a> {
    pub symbol: &'a str,
}

#[derive(Serialize)]
pub struct Ticker24hParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<&'a str>,
}

#[derive(Serialize)]
pub struct TickerDayParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<&'a str>,
}

#[derive(Serialize)]
pub struct PriceTickerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,
}

#[derive(Serialize)]
pub struct BookTickerParams<'a>{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,
}