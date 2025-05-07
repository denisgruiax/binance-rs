use hmac::digest::InvalidLength;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug)]
pub enum BinanceError {
    Api(ApiError),
    BuildRequest(String),
    Deserialize(serde_json::Error),
    IpBanned,
    InternalServer,
    Request(reqwest::Error),
    RequestTimeout,
    Signature(InvalidLength),
    TooManyRequest,
    Unknown(String),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    code: i16,
    msg: String,
}

impl Display for BinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinanceError::Api(ApiError { code, msg }) => {
                write!(f, "code: {},\nmsg: {},\n", code, msg)
            }
            BinanceError::BuildRequest(msg) => write!(f, "Build request error: {}", msg),
            BinanceError::Deserialize(e) => write!(f, "Deserialize error: {}", e),
            BinanceError::IpBanned => write!(
                f,
                "IP Auto Banned after receiving the HTTP 429 error code multiple times!"
            ),
            BinanceError::InternalServer => write!(f, "Internal Binance server error!"),
            BinanceError::Request(e) => write!(f, "Request error: {}", e),
            BinanceError::RequestTimeout => write!(f, "Request timeout!"),
            BinanceError::Signature(e) => write!(f, "Secret key error: {}", e),
            BinanceError::TooManyRequest => write!(f, "To many requests sent by the client!"),
            BinanceError::Unknown(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl From<InvalidLength> for BinanceError {
    fn from(value: InvalidLength) -> Self {
        BinanceError::Signature(value)
    }
}

impl From<reqwest::Error> for BinanceError {
    fn from(value: reqwest::Error) -> Self {
        BinanceError::Request(value)
    }
}

impl From<serde_json::Error> for BinanceError {
    fn from(value: serde_json::Error) -> Self {
        BinanceError::Deserialize(value)
    }
}
