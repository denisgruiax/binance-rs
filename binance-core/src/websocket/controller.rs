use binance_common::error::BinanceError;

pub trait WebSocketController {
    type Command;
    type Error;
    type Response;
    type State;

    fn get_response(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn send_command<T: Send>(
        &mut self,
        command: impl FnOnce(tokio::sync::oneshot::Sender<Result<T, BinanceError>>) -> Self::Command
        + Send,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send;

    fn connect(&mut self, route: String) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn disconnect(&mut self) -> impl Future<Output = Result<(), BinanceError>> + Send;

    fn close(&mut self) -> impl Future<Output = Result<(), BinanceError>> + Send;

    fn reconnect(&mut self) -> impl Future<Output = Result<(), BinanceError>> + Send;

    fn get_state(&mut self) -> impl Future<Output = Result<Self::State, Self::Error>> + Send;

    fn watch(
        &self,
    ) -> impl Future<Output = tokio::sync::watch::Receiver<Result<Self::Response, Self::Error>>> + Send;
}
