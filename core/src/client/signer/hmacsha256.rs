use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::client::signer::signature::Signature;

pub struct HmacSha256<'a> {
    pub api_key: &'a str,
    pub secret_key: &'a str,
}

impl<'a> HmacSha256<'a> {
    fn new(api_key: &'a str, secret_key: &'a str) -> HmacSha256<'a> {
        HmacSha256 {
            api_key,
            secret_key,
        }
    }
}

impl<'a> Signature<'a> for HmacSha256<'a> {
    fn sign(&self, request: String) -> String {
        let mut hasher = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
        hasher.update(request.as_bytes());
        let result = hasher.finalize().into_bytes();

        hex::encode(result)
    }
}
