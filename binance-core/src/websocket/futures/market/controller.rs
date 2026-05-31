use binance_common::{
    enums::{WebSocketCommand, WebSocketState},
    error::BinanceError,
    futures::model::response::websocket::WebSocketResponse,
};

use crate::websocket::controller::WebSocketController;

pub struct WebSocketMarketController {
    tx_controller: tokio::sync::mpsc::Sender<WebSocketCommand>,
    rx_response: tokio::sync::mpsc::Receiver<Result<(), BinanceError>>,
    rx_watch: tokio::sync::watch::Receiver<Result<WebSocketResponse, BinanceError>>,
}

impl WebSocketMarketController {
    pub fn new(
        tx_controller: tokio::sync::mpsc::Sender<WebSocketCommand>,
        rx_response: tokio::sync::mpsc::Receiver<Result<(), BinanceError>>,
        rx_watch: tokio::sync::watch::Receiver<Result<WebSocketResponse, BinanceError>>,
    ) -> Self {
        WebSocketMarketController {
            tx_controller,
            rx_response,
            rx_watch,
        }
    }
}

impl WebSocketController for WebSocketMarketController {
    type Command = WebSocketCommand;
    type Error = BinanceError;
    type Response = WebSocketResponse;
    type State = WebSocketState;

    async fn get_response(&mut self) -> Result<(), BinanceError> {
        match self.rx_response.recv().await {
            Some(response) => response,
            None => Err(BinanceError::Channel(
                "The sender from WebSocket to Controller has been dropped.".to_string(),
            )),
        }
    }

    async fn get_state(&mut self) -> Result<Self::State, Self::Error> {
        let (tx, rx) = tokio::sync::oneshot::channel();

        self.send_command(WebSocketCommand::GetState(tx)).await?;

        rx.await.map_err(|_| {
            BinanceError::Channel(
                "The sender from WebSocket to Controller has been dropped.".to_string(),
            )
        })
    }

    async fn send_command(&mut self, command: WebSocketCommand) -> Result<(), BinanceError> {
        self.tx_controller
            .send(command)
            .await
            .map_err(|_| BinanceError::Channel("Failed to send WebSocket command.".to_string()))
    }

    async fn watch(&self) -> tokio::sync::watch::Receiver<Result<WebSocketResponse, BinanceError>> {
        self.rx_watch.clone()
    }
}
