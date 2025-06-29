use serde::Serialize;

pub mod account;
pub mod general;
pub mod market;
pub mod trade;

#[derive(Serialize)]
pub struct EmptyParams;
