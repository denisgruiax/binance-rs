use binance_api::model::BinanceError;
use chrono::Utc;

pub trait Signature<'a> {
    fn add_timestamp(&self, params: &str) -> String {
        format!("{}&timestamp={}", params, Utc::now().timestamp_millis())
    }

    fn build_request(&self, client: &reqwest::Client, host: &str, path: &str, params: &str) -> Result<reqwest::RequestBuilder, BinanceError>;
    fn sign(&self, host: &str, path: &str, params: &str) -> Result<String, BinanceError>;
}
