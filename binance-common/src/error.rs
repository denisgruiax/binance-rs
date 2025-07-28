use hmac::digest::InvalidLength;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug)]
pub enum BinanceError {
    Api(ApiError),
    BuildRequest(String),
    Channel(String),
    Deserialize(serde_json::Error),
    IpBanned,
    InternalServer,
    Request(reqwest::Error),
    RequestTimeout,
    Signature(InvalidLength),
    TooManyRequest,
    Unknown(String),
    WebSocket(ApiError),
    WebSocketInternal(String),
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
                write!(f, "[BinanceError::Api] code: {}, message: {}", code, msg)
            }

            BinanceError::BuildRequest(msg) => {
                write!(f, "[BinanceError::BuildRequest] {}", msg)
            }
            BinanceError::Channel(msg) => {
                write!(f, "[BinanceError::Channel] {}", msg)
            }
            BinanceError::Deserialize(e) => {
                write!(f, "[BinanceError::Deserialize] {}", e)
            }
            BinanceError::IpBanned => {
                write!(
                    f,
                    "[BinanceError::IpBanned] IP Auto-banned due to repeated 429 responses."
                )
            }
            BinanceError::InternalServer => {
                write!(
                    f,
                    "[BinanceError::InternalServer] Binance internal server error."
                )
            }
            BinanceError::Request(e) => {
                write!(f, "[BinanceError::Request] {}", e)
            }
            BinanceError::RequestTimeout => {
                write!(f, "[BinanceError::RequestTimeout] Request timed out.")
            }
            BinanceError::Signature(e) => {
                write!(f, "[BinanceError::Signature] Signature error: {}", e)
            }
            BinanceError::TooManyRequest => {
                write!(
                    f,
                    "[BinanceError::TooManyRequest] Rate limit exceeded (HTTP 429)."
                )
            }
            BinanceError::Unknown(e) => {
                write!(f, "[BinanceError::Unknown] {}", e)
            }
            BinanceError::WebSocket(ApiError { code, msg }) => {
                write!(
                    f,
                    "[BinanceError::WebSocket] code: {}, message: {}",
                    code, msg
                )
            }
            BinanceError::WebSocketInternal(msg) => {
                write!(f, "[BinnaceError::WebSocketInternal] {}", msg)
            }
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
