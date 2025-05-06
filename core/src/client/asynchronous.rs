use crate::client::signer::signature::Signature;
use binance_api::model::{BinanceError, params::url::UrlEncoded};
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;

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
    pub fn new(host: impl Into<&'a str>, signature: S) -> Client<'a, S> {
        Client {
            host: host.into(),
            signature,
            inner_client: reqwest::Client::new(),
        }
    }

    pub fn get<Path, Params>(
        &self,
        path: Path,
        params: Params,
    ) -> impl Future<Output = Result<Response, reqwest::Error>>
    where
        Path: Into<&'a str>,
        Params: UrlEncoded,
    {
        let endpoint = format!("{}{}{}", self.host, path.into(), params.to_url_encoded());

        self.inner_client.get(endpoint).send()
    }

    pub fn get_signed<Path, Params>(
        &self,
        path: Path,
        params: Params,
    ) -> impl Future<Output = Result<Response, reqwest::Error>>
    where
        Path: Into<&'a str>,
        Params: UrlEncoded,
    {
        let request = self.signature.build_request(
            &self.inner_client,
            self.host,
            path.into(),
            params.to_url_encoded().as_str(),
        );

        RequestBuilder::send(request)
    }

    pub async fn handle<T: DeserializeOwned>(
        response: impl Future<Output = Result<Response, reqwest::Error>>,
    ) -> Result<T, BinanceError> {
        match response.await {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(serde_json::from_str::<T>(&response.text().await.unwrap())
                    .map_err(|_| {
                        BinanceError::Other(String::from(
                            "Deserialize error from String to data type T",
                        ))
                    }))?,

                StatusCode::BAD_REQUEST => {
                    let res = response.text().await.unwrap();
                    let api_error: binance_api::model::error::ApiError =
                        serde_json::from_str(&res).unwrap();
                    Err(BinanceError::Error(api_error))
                }
                status_code => Err(BinanceError::Other(format!(
                    "Response Status Code: {}",
                    status_code
                ))),
            },
            Err(error) => Err(BinanceError::HttpRequest(error)),
        }
    }
}
