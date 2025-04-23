use crate::client::signer::signature::Signature;

pub struct HmacSha256<'a> {
    pub api_key: &'a str,
    pub secret_key: &'a str,
}

impl<'a> Signature for HmacSha256<'a> {
    type Output = [u8; 32];
    fn sign(&self, message: &[u8]) -> Self::Output {
        
    }
}
