use binance_common::error::BinanceError;

pub trait WebSocket {
    type Command;
    type Host;
    type Response;
    type Status;

    fn connect() -> Result<(), BinanceError>;
    fn disconnect() -> Result<(), BinanceError>;
    fn handle() -> Result<Self::Response, BinanceError>;
    fn run() -> ();
    fn status() -> Result<Self::Status, BinanceError>;
}
