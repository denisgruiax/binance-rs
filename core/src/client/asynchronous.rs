use crate::client::signer::signature::Signature;
use binance_api::model::params::url::UrlEncoded;
use reqwest::Response;

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
    pub fn new(host: &'a str, signature: S) -> Client<'a, S> {
        Client {
            host,
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
        let request = self
            .signature
            .build_request(self.host, path.into(), params.to_url_encoded().as_str())
            .build()
            .expect("Invalid signed request!");

        self.inner_client.execute(request)
    }
}
