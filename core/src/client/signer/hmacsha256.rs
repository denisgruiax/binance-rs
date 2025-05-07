use binance_api::model::BinanceError;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::client::signer::signature::Signature;

pub struct HmacSha256<'a> {
    api_key: &'a str,
    secret_key: &'a str,
}

impl<'a> HmacSha256<'a> {
    pub fn new(api_key: &'a str, secret_key: &'a str) -> HmacSha256<'a> {
        HmacSha256 {
            api_key,
            secret_key,
        }
    }
}

impl<'a> Signature<'a> for HmacSha256<'a> {
    fn build_request(&self, client: &reqwest::Client, host: &str, path: &str, params: &str) -> Result<reqwest::RequestBuilder, BinanceError>{
        let url = self.sign(host, path, params)?;

        Ok(client.get(url).header("X-MBX-APIKEY", self.api_key))
    } 

    fn sign(&self, host: &str, path: &str, params: &str) -> Result<String, BinanceError> {
        let mut hasher = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())?;
        let params = self.add_timestamp(params);

        hasher.update(params.as_bytes());

        let signature = hex::encode(hasher.finalize().into_bytes());
        let endpoint = format!("{}&signature={}", params, signature);

        Ok(format!("{}{}{}", host, path, endpoint))
    }
}
