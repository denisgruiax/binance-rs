use binance_common::error::BinanceError;
use binance_common::url::UrlEncoded;
use reqwest::{
    Method, StatusCode,
    blocking::{RequestBuilder, Response},
};
use serde::de::DeserializeOwned;

use crate::signer::signature::Signature;

pub struct Client<'a, S>
where
    S: Signature<'a>,
{
    host: &'a str,
    signature: S,
    inner_client: reqwest::blocking::Client,
}

impl<'a, S> Client<'a, S>
where
    S: Signature<'a>,
{
    pub fn new(host: &'a impl AsRef<str>, signature: S) -> Client<'a, S> {
        Client {
            host: host.as_ref(),
            signature,
            inner_client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get<T>(
        &self,
        path: impl AsRef<str>,
        params: impl UrlEncoded,
    ) -> std::result::Result<T, BinanceError>
    where
        T: DeserializeOwned,
    {
        let endpoint = format!("{}{}{}", self.host, path.as_ref(), params.to_url_encoded());

        let response = self.inner_client.get(endpoint).send()?;

        self.handle::<T>(response)
    }

    pub fn send<T>(
        &self,
        path: impl AsRef<str>,
        params: impl UrlEncoded,
        method: Method,
    ) -> Result<T, BinanceError>
    where
        T: DeserializeOwned,
    {
        let request = self.signature.build_blocking_request(
            &self.inner_client,
            self.host,
            path.as_ref(),
            params.to_url_encoded().as_str(),
            method,
        )?;

        let response = RequestBuilder::send(request)?;

        self.handle(response)
    }

    pub fn handle<T: DeserializeOwned>(&self, response: Response) -> Result<T, BinanceError> {
        let status = response.status();
        let body = response.bytes()?;

        match status {
            StatusCode::BAD_REQUEST => {
                let api_error: binance_common::error::ApiError = serde_json::from_slice(&body)
                    .map_err(|error| BinanceError::Deserialize(error))?;
                Err(BinanceError::Api(api_error))
            }

            StatusCode::IM_A_TEAPOT => Err(BinanceError::IpBanned),

            StatusCode::INTERNAL_SERVER_ERROR => Err(BinanceError::InternalServer),

            StatusCode::OK => Ok(serde_json::from_slice::<T>(&body)
                .map_err(|error| BinanceError::Deserialize(error))?),

            StatusCode::REQUEST_TIMEOUT => Err(BinanceError::RequestTimeout),
            StatusCode::UNAUTHORIZED => {
                let api_error: binance_common::error::ApiError = serde_json::from_slice(&body)
                    .map_err(|error| BinanceError::Deserialize(error))?;

                Err(BinanceError::Api(api_error))
            }
            StatusCode::TOO_MANY_REQUESTS => Err(BinanceError::TooManyRequest),
            status_code => Err(BinanceError::Unknown(format!(
                "Response Status Code: {}",
                status_code
            ))),
        }
    }
}
