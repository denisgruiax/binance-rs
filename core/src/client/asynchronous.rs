use crate::client::signer::signature::Signature;
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

    pub async fn get(&self, path: &str) -> impl Future<Output = Result<Response, reqwest::Error>> {
        let endpoint = format!("{}{}", self.host, path);
        self.inner_client.get(endpoint).send()
    }

    pub async fn get_signed(
        &self,
        path: &str,
        params: &str,
    ) -> impl Future<Output = Result<Response, reqwest::Error>> {
        let request = self
            .signature
            .build_request(self.host, path, params)
            .build()
            .expect("Invalid signed request!");

        self.inner_client.execute(request)
    }
}
