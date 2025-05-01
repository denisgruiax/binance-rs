use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug)]
pub enum BinanceError {
    Error(ApiError),
    BuildRequestError(String),
    HttpRequest(reqwest::Error),
    Other(String),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    code: i16,
    msg: String,
}

impl Display for BinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinanceError::Error(ApiError { code, msg }) => {
                write!(f, "code: {},\nmsg: {},\n", code, msg)
            }
            BinanceError::BuildRequestError(msg) => write!(f, "Build request error: {}", msg),
            BinanceError::HttpRequest(request) => write!(f, "Http response error: {}", request),
            BinanceError::Other(sig) => write!(f, "Signature build error: {}", sig),
        }
    }
}
