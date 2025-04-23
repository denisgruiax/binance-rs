use crate::client::signer::signature::Signature;
use reqwest::Response;

pub struct Client<'a, S>
where
    S: Signature,
{
    host: &'a str,
    api_key: &'a str,
    secret_key: &'a str,
    signature: &'a S,
    inner_client: reqwest::Client,
}

impl<'a, S> Client<'a, S>
where
    S: Signature,
{
    pub fn new(
        host: &'a str,
        api_key: &'a str,
        secret_key: &'a str,
        signature: &'a S,
    ) -> Client<'a, S> {
        Client {
            host,
            api_key,
            secret_key,
            signature,
            inner_client: reqwest::Client::new(),
        }
    }

    pub async fn get<T>(&self, path: T) -> impl Future<Output = Result<Response, reqwest::Error>>
    where
        T: Into<&'static str>,
    {
        let endpoint = format!("{}{}", self.host, path.into());
        self.inner_client.get(endpoint).send()
    }
}
