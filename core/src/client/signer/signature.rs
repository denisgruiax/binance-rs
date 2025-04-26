use chrono::Utc;

pub trait Signature<'a> {
    fn add_timestamp(&self, params: &str) -> String {
        format!("{}&timestamp={}", params, Utc::now().timestamp_millis())
    }

    fn build_request(&self, host: &str, path: &str, params: &str) -> reqwest::RequestBuilder;
    fn sign(&self, host: &str, path: &str, params: &str) -> String;
}
