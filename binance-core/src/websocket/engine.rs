use binance_common::{enums::WebSocketCommand, error::BinanceError};
use tokio_tungstenite::tungstenite::{Error, Message};

pub trait WebSocketEngine {
    type Command;
    type Error;
    type Response;

    fn connect(&mut self, stream: String) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn close(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn disconnect(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn handle(
        &self,
        message: tokio_tungstenite::tungstenite::Utf8Bytes,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;

    fn handle_frame(
        &mut self,
        frame: Result<Message, Error>,
    ) -> impl Future<Output = Result<(), BinanceError>>;

    fn handle_message(
        &mut self,
        message: Message,
    ) -> impl Future<Output = Result<(), BinanceError>> + Send;

    fn handle_command(
        &mut self,
        command: WebSocketCommand,
    ) -> impl Future<Output = Result<(), BinanceError>> + Send;

    fn select_action(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn run(&mut self) -> impl Future<Output = ()> + Send;
}
