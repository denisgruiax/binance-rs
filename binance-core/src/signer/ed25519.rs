use base64::Engine;
use binance_common::error::BinanceError;
use ed25519_dalek::{Signer, SigningKey, pkcs8::DecodePrivateKey};
use reqwest::Method;

pub struct Ed25519Dalek {
    api_key: String,
    signing_key: SigningKey,
}

impl Ed25519Dalek {
    pub fn new(api_key: String, signing_key: &str) -> Result<Ed25519Dalek, BinanceError> {
        Ok(Ed25519Dalek {
            api_key,
            signing_key: SigningKey::from_pkcs8_pem(signing_key).map_err(|_| {
                BinanceError::Unknown("Invalid private key pem for ed25519!".to_string())
            })?,
        })
    }
}

impl<'a> crate::signer::signature::Signature<'a> for Ed25519Dalek {
    fn build_blocking_request(
        &self,
        client: &reqwest::blocking::Client,
        host: &str,
        path: &str,
        params: &str,
        method: reqwest::Method,
    ) -> Result<reqwest::blocking::RequestBuilder, binance_common::error::BinanceError> {
        let url = self.sign(host, path, params)?;

        match method {
            Method::DELETE => Ok(client
                .delete(url)
                .header("X-MBX-APIKEY", self.api_key.as_str())),
            Method::GET => Ok(client
                .get(url)
                .header("X-MBX-APIKEY", self.api_key.as_str())),
            Method::POST => Ok(client
                .post(url)
                .header("X-MBX-APIKEY", self.api_key.as_str())),
            _ => Err(binance_common::error::BinanceError::Unknown(
                "Invalid method to send the request!".to_string(),
            )),
        }
    }

    fn build_request(
        &self,
        client: &reqwest::Client,
        host: &str,
        path: &str,
        params: &str,
        method: reqwest::Method,
    ) -> Result<reqwest::RequestBuilder, binance_common::error::BinanceError> {
        let url = self.sign(host, path, params)?;

        match method {
            Method::DELETE => Ok(client
                .delete(url)
                .header("X-MBX-APIKEY", self.api_key.as_str())),
            Method::GET => Ok(client
                .get(url)
                .header("X-MBX-APIKEY", self.api_key.as_str())),
            Method::POST => Ok(client
                .post(url)
                .header("X-MBX-APIKEY", self.api_key.as_str())),
            _ => Err(binance_common::error::BinanceError::Unknown(
                "Invalid method to send the request!".to_string(),
            )),
        }
    }

    fn sign(
        &self,
        host: &str,
        path: &str,
        params: &str,
    ) -> Result<String, binance_common::error::BinanceError> {
        let params = self.add_timestamp(params);
        let signature = self.signing_key.sign(params.as_bytes());
        let endpoint = format!(
            "{}&signature={}",
            params,
            base64::engine::general_purpose::STANDARD.encode(signature.to_bytes())
        );

        Ok(format!("{}{}{}", host, path, endpoint))
    }
}
