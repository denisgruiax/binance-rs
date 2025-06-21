use crate::client::signer::signature::Signature;
use binance_api::model::{BinanceError, params::url::UrlEncoded};
use reqwest::{Method, RequestBuilder, Response};

pub struct Client<'a, S>
where
    S: Signature<'a>,
{
    host: &'a str,
    signature: S,
    inner_client: reqwest::Client,
}

impl<'a, S> Client<'a, S>
where
    S: Signature<'a>,
{
    pub fn new(host: &'a impl AsRef<str>, signature: S) -> Client<'a, S> {
        Client {
            host: host.as_ref(),
            signature,
            inner_client: reqwest::Client::new(),
        }
    }

    pub fn get(
        &self,
        path: &'a impl AsRef<str>,
        params: impl UrlEncoded,
    ) -> impl Future<Output = Result<Response, reqwest::Error>> {
        let endpoint = format!("{}{}{}", self.host, path.as_ref(), params.to_url_encoded());

        self.inner_client.get(endpoint).send()
    }

    pub fn send(
        &self,
        path: &'a impl AsRef<str>,
        params: impl UrlEncoded,
        method: Method,
    ) -> Result<impl Future<Output = Result<Response, reqwest::Error>>, BinanceError> {
        let request = self.signature.build_request(
            &self.inner_client,
            self.host,
            path.as_ref(),
            params.to_url_encoded().as_str(),
            method,
        )?;

        Ok(RequestBuilder::send(request))
    }
}
