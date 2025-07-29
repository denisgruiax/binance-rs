use binance_common::{error::BinanceError, futures::model::response::websocket::{AggTradeResponse, WebSocketResponse}};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, Utf8Bytes},
};

#[derive(Debug)]
pub enum WebSocketCommand {
    Connect(String),
    Disconnect,
    Reconnect,
}

#[derive(Debug)]
pub enum WebSocketStatus {
    Connected,
    Disconnected,
    Idle,
}

pub struct WebSocket {
    rx_controller: tokio::sync::mpsc::Receiver<WebSocketCommand>,
    tx_response: tokio::sync::mpsc::Sender<Result<(), BinanceError>>,
    tx_watch: tokio::sync::watch::Sender<Result<WebSocketResponse, BinanceError>>,
    socket: Option<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    status: WebSocketStatus,
}

impl WebSocket {
    pub fn new(
        rx_controller: tokio::sync::mpsc::Receiver<WebSocketCommand>,
        tx_response: tokio::sync::mpsc::Sender<Result<(), BinanceError>>,
        tx_watch: tokio::sync::watch::Sender<Result<WebSocketResponse, BinanceError>>,
    ) -> WebSocket {
        WebSocket {
            rx_controller,
            tx_response,
            tx_watch,
            socket: None,
            status: WebSocketStatus::Idle,
        }
    }

    pub async fn connect(&mut self, stream: String) -> Result<(), BinanceError> {
        println!("Connect function reached!");
        let (socket, _) = connect_async(stream).await.map_err(|_| {
            BinanceError::Unknown(String::from(
                "Unable to asynchronously connect to the socket.",
            ))
        })?;

        self.socket = Some(socket);
        self.status = WebSocketStatus::Connected;

        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), BinanceError> {
        if let Some(ref mut socket) = self.socket {
            socket.close(None).await.map_err(|_| {
                BinanceError::Unknown(String::from(
                    "Failed to disconnect teh socket from the server!",
                ))
            })?;

            self.socket = None;

            return Ok(());
        }

        Err(BinanceError::Unknown(String::from(
            "Socket is not intialized",
        )))
    }

    pub fn handle(&self, message: Utf8Bytes) -> Result<WebSocketResponse, BinanceError> {
        let response = serde_json::from_slice(message.as_bytes())
            .map_err(|error| BinanceError::Deserialize(error));

        match response {
        Ok(agg_trade:AggTradeResponse) => WebSocketResponse::AggTrade(agg_trade)
        }
    }

    pub async fn run(&mut self) -> () {
        loop {
            match self.status {
                WebSocketStatus::Connected => {
                    let socket_ref = self.socket.as_mut().unwrap();

                    tokio::select! {
                    Some(result) = socket_ref.next() => {
                        match result.unwrap() {
                            Message::Text(msg) => {self.tx_watch.send(Ok(Message::Text(msg))).unwrap();},
                            Message::Ping(payload) => {
                                println!("PING from Binance Server! Pong sent back!");
                                self.socket.as_mut().unwrap().send(Message::Pong(payload)).await.unwrap();
                            },
                            _ => {println!("Error, no Text message from Kline")}}
                        },
                    Some(command) = self.rx_controller.recv() => {match  command {
                        WebSocketCommand::Disconnect => {self.disconnect().await.unwrap()},
                        _ => {}
                    }},
                    }
                }
                WebSocketStatus::Disconnected => {
                    println!("Disconnecting")
                }
                WebSocketStatus::Idle => {
                    if let Some(command) = self.rx_controller.recv().await {
                        match command {
                            WebSocketCommand::Connect(stream) => {
                                self.tx_response.send(Ok(())).await.unwrap();
                                self.connect(stream).await.unwrap();
                            }
                            _ => {
                                self.tx_response
                                    .send(Err(BinanceError::WebSocketInternal(String::from(
                                        "value",
                                    ))))
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

pub struct WebSocketController {
    tx_controller: tokio::sync::mpsc::Sender<WebSocketCommand>,
    pub rx_response: tokio::sync::mpsc::Receiver<Result<(), BinanceError>>,
    rx_watch: tokio::sync::watch::Receiver<Result<Message, BinanceError>>,
}

impl WebSocketController {
    pub fn new(
        tx_controller: tokio::sync::mpsc::Sender<WebSocketCommand>,
        rx_response: tokio::sync::mpsc::Receiver<Result<(), BinanceError>>,
        rx_watch: tokio::sync::watch::Receiver<Result<Message, BinanceError>>,
    ) -> Self {
        WebSocketController {
            tx_controller,
            rx_response,
            rx_watch,
        }
    }

    pub async fn send_command(&mut self, command: WebSocketCommand) -> Result<(), BinanceError> {
        self.tx_controller.send(command).await.map_err(|_| {
            BinanceError::Channel(String::from("Failed to send WebSocket command."))
        })?;

        Ok(())
    }

    pub async fn watch(&self) -> tokio::sync::watch::Receiver<Result<Message, BinanceError>> {
        self.rx_watch.clone()
    }
}

pub struct WebSocketSession;

impl WebSocketSession {
    pub fn sessions() -> (WebSocketController, WebSocket) {
        let (tx_controller, rx_controller) = tokio::sync::mpsc::channel(1);
        let (tx_response, rx_response) = tokio::sync::mpsc::channel(1);
        let (tx_watch, rx_watch) = tokio::sync::watch::channel::<Result<Message, BinanceError>>(
            Ok(Message::Text(Utf8Bytes::from(""))),
        );

        let controller = WebSocketController::new(tx_controller, rx_response, rx_watch);
        let web_socket = WebSocket::new(rx_controller, tx_response, tx_watch);

        (controller, web_socket)
    }
}
z