use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize)]
pub struct OrderBookParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}