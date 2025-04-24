pub trait Signature<'a> {
    fn sign(&self, params: &str) -> String;
}
