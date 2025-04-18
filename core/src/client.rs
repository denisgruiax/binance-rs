use std::path;

use crate::api::Api;
use reqwest::Response;
pub struct Client<'a> {
    host: &'a str,
    api_key: &'a str,
    secret_key: &'a str,
    inner_client: reqwest::Client,
}

impl<'a> Client<'a> {
    pub fn new(host: &'a str, api_key: &'a str, secret_key: &'a str) -> Client<'a> {
        Client {
            host,
            api_key,
            secret_key,
            inner_client: reqwest::Client::new(),
        }
    }

    pub async fn get<T>(&self, path: T) -> reqwest::Result<Response>
    where
        T: Into<&'static str>,
    {
        let endpoint = format!("{}{}", self.host, path.into());

        self.inner_client.get(endpoint).send().await
    }
}
