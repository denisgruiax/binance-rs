use super::signer::signature::Signature;

pub struct Client<'a, S>
where
    S: Signature<'a>,
{
    host: &'a str,
    signature: S,
    inner_client: reqwest::blocking::Client,
}

impl<'a, S> Client<'a, S> where S: Signature<'a> {
    pub fn new(host: &'a str, signature: S) -> Client<'a, S> {
        Client {
            host,
            signature,
            inner_client: reqwest::blocking::Client::new(),
        }
    }
}