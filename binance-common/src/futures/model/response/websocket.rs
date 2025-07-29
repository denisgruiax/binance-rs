use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Deserialize)]
pub enum WebSocketResponse {
    AggTrade(AggTradeResponse),
    Kline(KlineResponse),
    MarkPrice(MarkPriceResponse),
    MiniTicker(MiniTickerResponse),
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

#[derive(Debug, Clone, Deserialize)]
pub struct KlineResponse {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub kline: Kline,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: u64,

    #[serde(rename = "T")]
    pub close_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "L")]
    pub last_trade_id: u64,

    #[serde(rename = "o")]
    #[serde_as(as = "DisplayFromStr")]
    pub open_price: f64,

    #[serde(rename = "c")]
    #[serde_as(as = "DisplayFromStr")]
    pub close_price: f64,

    #[serde(rename = "h")]
    #[serde_as(as = "DisplayFromStr")]
    pub high_price: f64,

    #[serde(rename = "l")]
    #[serde_as(as = "DisplayFromStr")]
    pub low_price: f64,

    #[serde(rename = "v")]
    #[serde_as(as = "DisplayFromStr")]
    pub base_asset_volume: f64,

    #[serde(rename = "n")]
    pub number_of_trades: u64,

    #[serde(rename = "x")]
    pub is_closed: bool,

    #[serde(rename = "q")]
    #[serde_as(as = "DisplayFromStr")]
    pub quote_asset_volume: f64,

    #[serde(rename = "V")]
    #[serde_as(as = "DisplayFromStr")]
    pub taker_buy_base_asset_volume: f64,

    #[serde(rename = "Q")]
    #[serde_as(as = "DisplayFromStr")]
    pub taker_buy_quote_asset_volume: f64,

    #[serde(rename = "B")]
    pub ignore: String,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct MiniTickerResponse {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    #[serde_as(as = "DisplayFromStr")]
    pub close_price: f64,

    #[serde(rename = "o")]
    #[serde_as(as = "DisplayFromStr")]
    pub open_price: f64,

    #[serde(rename = "h")]
    #[serde_as(as = "DisplayFromStr")]
    pub high_price: f64,

    #[serde(rename = "l")]
    #[serde_as(as = "DisplayFromStr")]
    pub low_price: f64,

    #[serde(rename = "v")]
    #[serde_as(as = "DisplayFromStr")]
    pub base_volume: f64,

    #[serde(rename = "q")]
    #[serde_as(as = "DisplayFromStr")]
    pub quote_volume: f64,
}