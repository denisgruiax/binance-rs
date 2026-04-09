pub mod futures;
pub mod spot;

pub trait WebSocket {
    type Command;
    type Error;
    type Response;

    fn connect(&mut self, stream: String) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn close(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn disconnect(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn handle(
        &mut self,
        message: tokio_tungstenite::tungstenite::Utf8Bytes,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;

    fn select_action(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn run(&mut self);
}
