use binance_common::enums::WebSocketType;

pub trait WebSocketSupervisor {
    type Error;
    type Response;
    type State;

    fn new(websocket_type: WebSocketType) -> Self;
    fn start(&mut self, route: String) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn stop(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn restart(&mut self, route: String) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn get_state(&mut self) -> impl Future<Output = Result<Self::State, Self::Error>> + Send;
    fn watch(
        &mut self,
    ) -> impl Future<
        Output = Result<
            tokio::sync::watch::Receiver<Result<Self::Response, Self::Error>>,
            Self::Error,
        >,
    > + Send;
}
