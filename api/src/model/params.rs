use serde::Serialize;

pub mod account;
pub mod general;
pub mod binance;
pub mod market;
pub mod trading;
pub mod url;

#[derive(Serialize)]
pub struct EmptyParams;