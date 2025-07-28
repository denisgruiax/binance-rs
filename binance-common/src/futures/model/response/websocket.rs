use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug)]
pub enum WebSocketResponse {
    Kline(AggTradeResponse),
    MarkPrice(MarkPriceResponse),
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct AggTradeResponse {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub aggregate_trade_id: u64,

    #[serde(rename = "p")]
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,

    #[serde(rename = "q")]
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "l")]
    pub last_trade_id: u64,

    #[serde(rename = "T")]
    pub trade_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct MarkPriceResponse {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    #[serde_as(as = "DisplayFromStr")]
    pub mark_price: f64,

    #[serde(rename = "i")]
    #[serde_as(as = "DisplayFromStr")]
    pub index_price: f64,

    #[serde(rename = "P")]
    #[serde_as(as = "DisplayFromStr")]
    pub estimated_settle_price: f64,

    #[serde(rename = "r")]
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,

    #[serde(rename = "T")]
    pub next_funding_time: u64,
}
