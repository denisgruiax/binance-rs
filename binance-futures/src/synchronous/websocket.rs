use binance_common::{
    enums::WebSocketType, error::BinanceError,
    futures::model::response::websocket::WebSocketResponse,
};
use binance_core::websocket::futures::market::{WebSocketMarket, WebSocketMarketController};

pub struct WebSocketConnection;

impl WebSocketConnection {
    pub fn new_market(
        websocket_type: WebSocketType,
    ) -> (WebSocketMarketController, WebSocketMarket) {
        let (tx_controller, rx_controller) = tokio::sync::mpsc::channel(1);
        let (tx_response, rx_response) = tokio::sync::mpsc::channel(1);
        let (tx_watch, rx_watch) = tokio::sync::watch::channel::<
            Result<WebSocketResponse, BinanceError>,
        >(Ok(WebSocketResponse::Init));

        let controller = WebSocketMarketController::new(tx_controller, rx_response, rx_watch);
        let web_socket = WebSocketMarket::new(rx_controller, tx_response, tx_watch, websocket_type);

        (controller, web_socket)
    }
}
