use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default)]
pub enum ContractType {
    #[default]
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    PerpetualDelivering,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    PendingTrading,
    Trading,
    PreDelivering,
    Delivering,
    Delivered,
    PreSettle,
    Settling,
    Close,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
    ExpiredInMatch,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default)]
pub enum OrderType {
    #[default]
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderResponseType {
    Ack,
    Result,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[derive(Default)]
pub enum OrderSide {
    #[default]
    Buy,
    Sell,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[derive(Default)]
pub enum PositionSide {
    Both,
    #[default]
    Long,
    Short,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
    Gtx,
    Gtd,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseType {
    Ack,
    Result,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StpModes {
    ExpireTaker,
    ExpireBoth,
    ExpireMaker,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceMatch {
    None,
    Opponent,
    Opponent5,
    Opponent10,
    Opponent20,
    Queue,
    Queue5,
    Queue10,
    Queue20,
}

#[derive(Debug, Clone, Serialize)]
pub enum AdlLevel {
    LowestRisk,
    LowRisk,
    Medium,
    HighRisk,
    HighestRisk,
}

impl<'de> Deserialize<'de> for AdlLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: u8 = Deserialize::deserialize(deserializer)?;
        Ok(match value {
            0 => AdlLevel::LowestRisk,
            1 => AdlLevel::LowRisk,
            2 => AdlLevel::Medium,
            3 => AdlLevel::HighRisk,
            4 => AdlLevel::HighestRisk,
            _ => return Err(serde::de::Error::custom("Invalid ADL level")),
        })
    }
}
