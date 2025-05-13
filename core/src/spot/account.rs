use crate::client::{signer::signature::Signature, synchronous::Client};

pub struct AccountApi<'a> {
    client: Client<'a, impl Signature<'a>>,
}

impl<'a> AccountApi<a> {
    pub fn new(client: Client<'a, impl Signature<'a>>) {
        AccountApi { client }
    }
}
