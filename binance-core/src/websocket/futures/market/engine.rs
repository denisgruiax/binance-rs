use binance_common::{
    enums::{WebSocketCommand, WebSocketState, WebSocketType},
    error::BinanceError,
    futures::model::response::websocket::{CombinedStreamResponse, WebSocketResponse},
};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Error, Message, Utf8Bytes},
};

use crate::websocket::engine::WebSocketEngine;

pub struct WebSocketMarketEngine {
    rx_controller: tokio::sync::mpsc::Receiver<WebSocketCommand>,
    tx_response: tokio::sync::mpsc::Sender<Result<(), BinanceError>>,
    tx_watch: tokio::sync::watch::Sender<Result<WebSocketResponse, BinanceError>>,
    socket: Option<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    state: WebSocketState,
    websocket_type: WebSocketType,
}

impl WebSocketMarketEngine {
    pub fn new(
        rx_controller: tokio::sync::mpsc::Receiver<WebSocketCommand>,
        tx_response: tokio::sync::mpsc::Sender<Result<(), BinanceError>>,
        tx_watch: tokio::sync::watch::Sender<Result<WebSocketResponse, BinanceError>>,
        websocket_type: WebSocketType,
    ) -> Self {
        WebSocketMarketEngine {
            rx_controller,
            tx_response,
            tx_watch,
            socket: None,
            state: WebSocketState::Idle,
            websocket_type,
        }
    }
}

impl WebSocketEngine for WebSocketMarketEngine {
    type Command = WebSocketCommand;
    type Error = BinanceError;
    type Response = WebSocketResponse;

    async fn connect(&mut self, stream: String) -> Result<(), BinanceError> {
        let (socket, _) = connect_async(stream).await.map_err(|_| {
            BinanceError::WebSocketInternal(
                "WebSocketMarketEngine has failed to connect to the stream!".to_string(),
            )
        })?;

        self.socket = Some(socket);
        self.state = WebSocketState::Connected;

        Ok(())
    }

    async fn close(&mut self) -> Result<(), BinanceError> {
        if let Some(ref mut socket) = self.socket {
            socket.close(None).await.map_err(|_| {
                BinanceError::WebSocketInternal(
                    "WebSocketMarketEngine has failed to disconnect from the stream!".to_string(),
                )
            })?;

            self.socket = None;
            self.state = WebSocketState::Closed;

            return Ok(());
        }

        Err(BinanceError::WebSocketInternal(
            "WebSocketMarketEngine is already disconnected!".to_string(),
        ))
    }

    async fn disconnect(&mut self) -> Result<(), BinanceError> {
        if let Some(ref mut socket) = self.socket {
            socket.close(None).await.map_err(|_| {
                BinanceError::WebSocketInternal(
                    "WebSocketMarketEngine has failed to disconnect from the stream!".to_string(),
                )
            })?;

            self.socket = None;
            self.state = WebSocketState::Idle;

            return Ok(());
        }

        Err(BinanceError::WebSocketInternal(
            "WebSocketMarketEngine is already disconnected!".to_string(),
        ))
    }

    async fn handle(&self, message: Utf8Bytes) -> Result<WebSocketResponse, BinanceError> {
        match self.websocket_type {
            WebSocketType::SingleStream => {
                serde_json::from_slice::<WebSocketResponse>(message.as_bytes())
                    .map_err(BinanceError::Deserialize)
            }
            WebSocketType::MultiStream => {
                let response = serde_json::from_slice::<CombinedStreamResponse>(message.as_bytes())
                    .map_err(BinanceError::Deserialize)?;
                Ok(response.data)
            }
        }
    }

    async fn handle_frame(&mut self, frame: Result<Message, Error>) -> Result<(), BinanceError> {
        match frame {
            Ok(message) => self.handle_message(message).await,
            Err(_) => Err(BinanceError::Unknown(
                "Socket reference problem!".to_string(),
            )),
        }
    }

    async fn handle_message(&mut self, message: Message) -> Result<(), BinanceError> {
        match message {
            Message::Close(_) => {
                self.tx_watch
                    .send(Err(BinanceError::WebSocketInternal(
                        "Close frame arrived into WebSocketMarketEngine engine".to_string(),
                    )))
                    .map_err(|_| {
                        BinanceError::Channel(
                            "Failed to send the close error through watch channel.".to_string(),
                        )
                    })?;
                let response = self.close().await;
                self.tx_response.send(response).await.map_err(|_| {
                    BinanceError::Channel(
                        "Failed to send the close error through response channel.".to_string(),
                    )
                })
            }

            Message::Text(msg) => self.tx_watch.send(self.handle(msg).await).map_err(|_| {
                BinanceError::Channel(
                    "Failed to send the close error through watch channel.".to_string(),
                )
            }),

            Message::Ping(payload) => self
                .socket
                .as_mut()
                .ok_or(BinanceError::WebSocketInternal(
                    "Unable to get a mutable reference to socket".to_string(),
                ))?
                .send(Message::Pong(payload))
                .await
                .map_err(|_| {
                    BinanceError::Channel(
                        "Failed to send the Pong response back to server with socket channel"
                            .to_string(),
                    )
                }),
            _ => Err(BinanceError::Unknown(
                "Unknown error for socket reeference".to_string(),
            )),
        }
    }

    async fn handle_command(&mut self, command: WebSocketCommand) -> Result<(), BinanceError> {
        match command {
            WebSocketCommand::Connect(_, tx) => {
                let error = Err(BinanceError::Unknown(
                    "Websocket already Connected!".to_string(),
                ));

                tx.send(error)
                    .map_err(|error: Result<(), BinanceError>| error.unwrap_err())
            }
            WebSocketCommand::Disconnect(tx) => {
                let error = self.disconnect().await.map_err(|_| {
                    BinanceError::Channel("Failed to disconnect the socket channel".to_string())
                });

                tx.send(error)
                    .map_err(|error: Result<(), BinanceError>| error.unwrap_err())
            }
            WebSocketCommand::Close(tx) => {
                let error: Result<(), BinanceError> = self.close().await.map_err(|_| {
                    BinanceError::Channel("Failed to close the socket channel".to_string())
                });

                tx.send(error)
                    .map_err(|error: Result<(), BinanceError>| error.unwrap_err())
            }
            WebSocketCommand::GetState(tx) => tx.send(Ok(self.state)).map_err(|_| {
                BinanceError::Channel("Failed to send state to controller.".to_string())
            }),
            _ => Err(BinanceError::Unknown(
                "Unknown error for controller channel".to_string(),
            )),
        }
    }

    async fn select_action(&mut self) -> Result<(), BinanceError> {
        let socket_ref: &mut WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>> =
            self.socket.as_mut().ok_or(BinanceError::WebSocketInternal(
                "Unable to get a mutable reference to socket".to_string(),
            ))?;

        tokio::select! {
        Some(frame) = socket_ref.next() => {
                self.handle_frame(frame).await
            },

        Some(command) = self.rx_controller.recv() => {self.handle_command(command).await},
        }
    }

    async fn run(&mut self) {
        loop {
            match self.state {
                WebSocketState::Connected => {
                    self.select_action().await.unwrap();
                }

                WebSocketState::Closed => break,

                WebSocketState::Disconnected | WebSocketState::Idle => {
                    if let Some(command) = self.rx_controller.recv().await {
                        match command {
                            WebSocketCommand::Connect(stream, tx) => {
                                let response = self.connect(stream).await;
                                tx.send(response).unwrap();
                            }
                            _ => {
                                self.tx_response
                                    .send(Err(BinanceError::WebSocketInternal(
                                        "state is Idle, only connect or close is available"
                                            .to_string(),
                                    )))
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
