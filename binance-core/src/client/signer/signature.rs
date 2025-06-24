use binance_common::model::BinanceError;
use chrono::Utc;
use reqwest::Method;

pub trait Signature<'a> {
    fn add_timestamp(&self, params: &str) -> String {
        format!("{}&timestamp={}", params, Utc::now().timestamp_millis())
    }

    fn build_blocking_request(
        &self,
        client: &reqwest::blocking::Client,
        host: &str,
        path: &str,
        params: &str,
        method: Method,
    ) -> Result<reqwest::blocking::RequestBuilder, BinanceError>;

    fn build_request(
        &self,
        client: &reqwest::Client,
        host: &str,
        path: &str,
        params: &str,
        method: Method,
    ) -> Result<reqwest::RequestBuilder, BinanceError>;

    fn sign(&self, host: &str, path: &str, params: &str) -> Result<String, BinanceError>;
}
