pub mod futures;
pub mod spot;

pub trait WebSocket {
    type Command;
    type Error;
    type Response;

    fn connect(
        &mut self,
        stream: String,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'static;
}
