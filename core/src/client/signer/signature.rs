pub trait Signature {
    fn sign(&self, request: String) -> String;
}
