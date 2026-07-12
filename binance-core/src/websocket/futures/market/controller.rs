use binance_common::{
    enums::{WebSocketCommand, WebSocketState},
    error::BinanceError,
    futures::model::response::websocket::WebSocketResponse,
};

use crate::websocket::controller::WebSocketController;

pub struct WebSocketMarketController {
    tx_command: tokio::sync::mpsc::Sender<WebSocketCommand>,
    rx_response: tokio::sync::mpsc::Receiver<Result<(), BinanceError>>,
    rx_watch: tokio::sync::watch::Receiver<Result<WebSocketResponse, BinanceError>>,
}

impl WebSocketMarketController {
    pub fn new(
        tx_command: tokio::sync::mpsc::Sender<WebSocketCommand>,
        rx_response: tokio::sync::mpsc::Receiver<Result<(), BinanceError>>,
        rx_watch: tokio::sync::watch::Receiver<Result<WebSocketResponse, BinanceError>>,
    ) -> Self {
        WebSocketMarketController {
            tx_command,
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

    async fn send_command<T: Send>(
        &mut self,
        command: impl FnOnce(tokio::sync::oneshot::Sender<Result<T, BinanceError>>) -> WebSocketCommand
        + Send,
    ) -> Result<T, BinanceError> {
        let (tx, rx) = tokio::sync::oneshot::channel();

        self.tx_command
            .send(command(tx))
            .await
            .map_err(|_| BinanceError::Channel("Failed to send WebSocket command.".to_string()))?;

        rx.await
            .map_err(|_| BinanceError::Channel("Response dropped!".to_string()))?
    }

    async fn get_state(&mut self) -> Result<Self::State, Self::Error> {
        self.send_command(
            |tx: tokio::sync::oneshot::Sender<Result<WebSocketState, BinanceError>>| {
                WebSocketCommand::GetState(tx)
            },
        )
        .await
    }

    async fn connect(&mut self, route: String) -> Result<(), Self::Error> {
        self.send_command(
            |tx: tokio::sync::oneshot::Sender<Result<(), BinanceError>>| {
                WebSocketCommand::Connect(route, tx)
            },
        )
        .await
    }

    async fn disconnect(&mut self) -> Result<(), BinanceError> {
        self.send_command(
            |tx: tokio::sync::oneshot::Sender<Result<(), BinanceError>>| {
                WebSocketCommand::Disconnect(tx)
            },
        )
        .await
    }

    async fn close(&mut self) -> Result<(), BinanceError> {
        self.send_command(
            |tx: tokio::sync::oneshot::Sender<Result<(), BinanceError>>| {
                WebSocketCommand::Close(tx)
            },
        )
        .await
    }

    async fn reconnect(&mut self) -> Result<(), BinanceError> {
        self.send_command(
            |tx: tokio::sync::oneshot::Sender<Result<(), BinanceError>>| {
                WebSocketCommand::Reconnect(tx)
            },
        )
        .await
    }

    async fn watch(&self) -> tokio::sync::watch::Receiver<Result<WebSocketResponse, BinanceError>> {
        self.rx_watch.clone()
    }
}
