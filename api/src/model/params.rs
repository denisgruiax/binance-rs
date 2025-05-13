use serde::Serialize;

pub mod account;
pub mod general;
pub mod interval;
pub mod market;
pub mod url;

#[derive(Serialize)]
pub struct EmptyParams;