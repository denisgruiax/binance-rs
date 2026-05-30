use binance_common::{
    enums::{
        WebSocketCommand::{self, Disconnect},
        WebSocketState, WebSocketType,
    },
    error::BinanceError,
    futures::model::response::websocket::WebSocketResponse,
};
use tokio::task::JoinHandle;

use super::controller::WebSocketMarketController;
use crate::websocket::{
    controller::WebSocketController, engine::WebSocketEngine,
    futures::market::engine::WebSocketMarketEngine, handler::WebSocketHandler,
};

pub struct WebSocketMarketHandler<C, E>
where
    C: WebSocketController,
    E: WebSocketEngine + Send,
{
    controller: Option<C>,
    engine: Option<E>,
    handler: Option<JoinHandle<E>>,
}

impl WebSocketHandler for WebSocketMarketHandler<WebSocketMarketController, WebSocketMarketEngine> {
    type Error = BinanceError;
    type Response = WebSocketResponse;
    type State = WebSocketState;

    fn new(websocket_type: WebSocketType) -> Self {
        let (tx_controller, rx_controller) = tokio::sync::mpsc::channel(1);
        let (tx_response, rx_response) = tokio::sync::mpsc::channel(1);
        let (tx_watch, rx_watch) = tokio::sync::watch::channel::<
            Result<WebSocketResponse, BinanceError>,
        >(Ok(WebSocketResponse::Init));

        let controller = Some(WebSocketMarketController::new(
            tx_controller,
            rx_response,
            rx_watch,
        ));

        let engine = Some(WebSocketMarketEngine::new(
            rx_controller,
            tx_response,
            tx_watch,
            websocket_type,
        ));

        Self {
            controller,
            engine,
            handler: None,
        }
    }

    async fn start(&mut self, route: String) -> Result<(), Self::Error> {
        if self.engine.is_some() && self.handler.is_none() {
            let mut engine = self.engine.take().unwrap();

            self.handler = Some(tokio::spawn(async move {
                engine.run().await;
                engine
            }));

            self.controller
                .as_mut()
                .unwrap()
                .send_command(WebSocketCommand::Connect(route))
                .await?;

            Ok(())
        } else {
            Err(BinanceError::Unknown(
                "Engine or handler bad state".to_string(),
            ))
        }
    }
    async fn stop(&mut self) -> Result<(), Self::Error> {
        if self.engine.is_none() && self.handler.is_some() {
            self.controller
                .as_mut()
                .unwrap()
                .send_command(WebSocketCommand::Close)
                .await?;

            self.engine = Some(tokio::join!(self.handler.take().unwrap()).0.unwrap());

            Ok(())
        } else {
            Err(BinanceError::Unknown(
                "Engine or handler bad state".to_string(),
            ))
        }
    }

    async fn restart(&mut self, route: String) -> Result<(), Self::Error> {
        if self.engine.is_none() && self.handler.is_some() {
            self.controller
                .as_mut()
                .unwrap()
                .send_command(Disconnect)
                .await?;
            self.controller
                .as_mut()
                .unwrap()
                .send_command(WebSocketCommand::Connect(route))
                .await?;

            Ok(())
        } else {
            Err(BinanceError::Unknown(
                "Engine or handler bad state".to_string(),
            ))
        }
    }

    async fn get_state(&mut self) -> Result<Self::State, Self::Error> {
        if self.engine.is_none() && self.handler.is_some() {
            return self.controller.as_mut().unwrap().get_state().await;
        } else {
            Err(BinanceError::Unknown(
                "Can not get state from engine".to_string(),
            ))
        }
    }

    async fn watch(
        &mut self,
    ) -> Result<tokio::sync::watch::Receiver<Result<Self::Response, Self::Error>>, Self::Error>
    {
        match self.engine.is_none() && self.handler.is_some() {
            true => Ok(self.controller.as_mut().unwrap().watch().await),
            _ => Err(BinanceError::Unknown(
                ("Can not watch data from engine").to_string(),
            )),
        }
    }
}
