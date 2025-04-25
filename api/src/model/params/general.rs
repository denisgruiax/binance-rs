use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize)]
pub struct ExchangeInformationParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<&'a str>,

    #[serde(rename = "recvWindow")]
    pub recv_window: u32,
}

impl<'a> Display for ExchangeInformationParams<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_urlencoded::to_string(self).unwrap_or_else(|_| String::new())
        )
    }
}
