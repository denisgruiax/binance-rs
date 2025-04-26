use serde::Serialize;
use std::fmt::Display;

use super::url::UrlEncoded;

#[derive(Serialize)]
pub struct ExchangeInformationParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<&'a str>,
}