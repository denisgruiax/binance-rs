use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize)]
pub struct OrderBookParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

impl<'a> Display for OrderBookParams<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_urlencoded::to_string(self).unwrap_or_else(|_| String::new())
        )
    }
}
