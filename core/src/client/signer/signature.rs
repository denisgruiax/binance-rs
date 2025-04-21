pub trait Signature {
    type Output;
    fn sign(&self, message: &[u8]) -> Self::Output;
}
