use serde::Serialize;

#[derive(Serialize)]
pub struct ExchangeInformationParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<&'a str>,
}