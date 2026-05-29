pub trait WebSocketController {
    type Command;
    type Error;
    type Response;
    type State;

    fn get_response(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn get_state(&mut self) -> impl Future<Output = Result<Self::State, Self::Error>> + Send;

    fn send_command(
        &mut self,
        command: Self::Command,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn watch(
        &self,
    ) -> impl Future<Output = tokio::sync::watch::Receiver<Result<Self::Response, Self::Error>>> + Send;
}
