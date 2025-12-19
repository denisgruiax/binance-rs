use binance_common::{
    enums::{WebSocketCommand, WebSocketStatus, WebSocketType},
    error::BinanceError,
    futures::model::response::websocket::{CombinedStreamResponse, WebSocketResponse},
};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, Utf8Bytes},
};

pub struct WebSocketMarket {
    rx_controller: tokio::sync::mpsc::Receiver<WebSocketCommand>,
    tx_response: tokio::sync::mpsc::Sender<Result<(), BinanceError>>,
    tx_watch: tokio::sync::watch::Sender<Result<WebSocketResponse, BinanceError>>,
    socket: Option<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    status: WebSocketStatus,
    websocket_type: WebSocketType,
}

impl WebSocketMarket {
    pub fn new(
        rx_controller: tokio::sync::mpsc::Receiver<WebSocketCommand>,
        tx_response: tokio::sync::mpsc::Sender<Result<(), BinanceError>>,
        tx_watch: tokio::sync::watch::Sender<Result<WebSocketResponse, BinanceError>>,
        websocket_type: WebSocketType,
    ) -> WebSocketMarket {
        WebSocketMarket {
            rx_controller,
            tx_response,
            tx_watch,
            socket: None,
            status: WebSocketStatus::Idle,
            websocket_type,
        }
    }

    async fn connect(&mut self, stream: String) -> Result<(), BinanceError> {
        let (socket, _) = connect_async(stream).await.map_err(|_| {
            BinanceError::WebSocketInternal(
                "WebSocketMarket has failed to connect to the stream!".to_string(),
            )
        })?;

        self.socket = Some(socket);
        self.status = WebSocketStatus::Connected;

        Ok(())
    }

    async fn close(&mut self) -> Result<(), BinanceError> {
        if let Some(ref mut socket) = self.socket {
            socket.close(None).await.map_err(|_| {
                BinanceError::WebSocketInternal(
                    "WebSocketMarket has failed to disconnect from the stream!".to_string(),
                )
            })?;

            self.socket = None;
            self.status = WebSocketStatus::Closed;

            return Ok(());
        }

        Err(BinanceError::WebSocketInternal(
            "WebSocketMarket is already disconnected!".to_string(),
        ))
    }

    async fn disconnect(&mut self) -> Result<(), BinanceError> {
        if let Some(ref mut socket) = self.socket {
            socket.close(None).await.map_err(|_| {
                BinanceError::WebSocketInternal(
                    "WebSocketMarket has failed to disconnect from the stream!".to_string(),
                )
            })?;

            self.socket = None;
            self.status = WebSocketStatus::Idle;

            return Ok(());
        }

        Err(BinanceError::WebSocketInternal(
            "WebSocketMarket is already disconnected!".to_string(),
        ))
    }

    fn handle(&self, message: Utf8Bytes) -> Result<WebSocketResponse, BinanceError> {
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

    async fn select_action(&mut self) -> Result<(), BinanceError> {
        let socket_ref = self.socket.as_mut().ok_or(BinanceError::WebSocketInternal("Unable to get a mutable reference to socket".to_string()))?;

        tokio::select! {
        Some(result) = socket_ref.next() => {
                match result {
                    Ok(response) => {match response {
                Message::Close(_) => {
                        self.tx_watch.send(Err(BinanceError::WebSocketInternal("Close frame arrived into WebSocketMarket engine".to_string()))).map_err(|_|BinanceError::Channel("Failed to send the close error through watch channel.".to_string()))?;
                        let response = self.close().await;
                        self.tx_response.send(response).await.map_err(|_|BinanceError::Channel("Failed to send the close error through response channel.".to_string()))
                    },

                Message::Text(msg) => {self.tx_watch.send(self.handle(msg)).map_err(|_| BinanceError::Channel("Failed to send the close error through watch channel.".to_string()))},

                Message::Ping(payload) => {
                    self.socket.as_mut().ok_or(BinanceError::WebSocketInternal("Unable to get a mutable reference to socket".to_string()))?.send(Message::Pong(payload)).await.map_err(|_| BinanceError::Channel("Failed to send the Pong response back to server with socket channel".to_string()))
                },
                _ => Err(BinanceError::Unknown("Unknown error for socket reeference".to_string()))}},
                    Err(_) => {Err(BinanceError::Unknown("Socket reference problem!".to_string()))},
                }
            },

        Some(command) = self.rx_controller.recv() => {match  command {
            WebSocketCommand::Disconnect => {self.disconnect().await.map_err(|_| BinanceError::Channel("Failed to disconnect the socket channel".to_string()))},
            WebSocketCommand::Close => {self.close().await.map_err(|_| BinanceError::Channel("Failed to close the socket channel".to_string()))},
            _ => {
                Err(BinanceError::Unknown("Unknown error for controller channel".to_string()))
            }
        }},
        }
    }

    pub async fn run(&mut self) -> () {
        loop {
            match self.status {
                WebSocketStatus::Connected => {
                    self.select_action().await.unwrap();
                }

                WebSocketStatus::Closed => break,

                WebSocketStatus::Disconnected | WebSocketStatus::Idle => {
                    if let Some(command) = self.rx_controller.recv().await {
                        match command {
                            WebSocketCommand::Connect(stream) => {
                                let response = self.connect(stream).await;
                                self.tx_response.send(response).await.unwrap();
                            }
                            _ => {
                                self.tx_response
                                    .send(Err(BinanceError::WebSocketInternal(
                                        "Status is Idle, only connect or close is available"
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

    pub async fn get_response(&mut self) -> Result<(), BinanceError> {
        match self.rx_response.recv().await {
            Some(response) => response,
            None => Err(BinanceError::Channel(
                "The sender from WebSocket to Controller have been dropped.".to_string(),
            )),
        }
    }

    pub async fn send_command(&mut self, command: WebSocketCommand) -> Result<(), BinanceError> {
        self.tx_controller
            .send(command)
            .await
            .map_err(|_| BinanceError::Channel("Failed to send WebSocket command.".to_string()))
    }

    pub async fn watch(
        &self,
    ) -> tokio::sync::watch::Receiver<Result<WebSocketResponse, BinanceError>> {
        self.rx_watch.clone()
    }
}
