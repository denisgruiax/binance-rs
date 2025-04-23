pub trait Signature<'a> {
    fn sign(&self, request: String) -> String;
}
