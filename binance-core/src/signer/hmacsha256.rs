use binance_common::error::BinanceError;
use hmac::{Hmac, Mac};
use reqwest::Method;
use sha2::Sha256;

use crate::signer::signature::Signature;

pub struct HmacSha256<'a> {
    pub api_key: &'a str,
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
    fn build_blocking_request(
        &self,
        client: &reqwest::blocking::Client,
        host: &str,
        path: &str,
        params: &str,
        method: Method,
    ) -> Result<reqwest::blocking::RequestBuilder, BinanceError> {
        let url = self.sign(host, path, params)?;

        match method {
            Method::DELETE => Ok(client.delete(url).header("X-MBX-APIKEY", self.api_key)),
            Method::GET => Ok(client.get(url).header("X-MBX-APIKEY", self.api_key)),
            Method::POST => Ok(client.post(url).header("X-MBX-APIKEY", self.api_key)),
            _ => Err(BinanceError::Unknown(String::from(
                "Invalid method to send the reuqest!",
            ))),
        }
    }

    fn build_request(
        &self,
        client: &reqwest::Client,
        host: &str,
        path: &str,
        params: &str,
        method: Method,
    ) -> Result<reqwest::RequestBuilder, BinanceError> {
        let url = self.sign(host, path, params)?;

        match method {
            Method::DELETE => Ok(client.delete(url).header("X-MBX-APIKEY", self.api_key)),
            Method::GET => Ok(client.get(url).header("X-MBX-APIKEY", self.api_key)),
            Method::POST => Ok(client.post(url).header("X-MBX-APIKEY", self.api_key)),
            _ => Err(BinanceError::Unknown(String::from(
                "Invalid method to send the reuqest!",
            ))),
        }
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
