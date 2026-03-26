pub mod futures;
pub mod spot;

pub trait WebSocket{
    type Command;
    type Error;
    type Response;
}