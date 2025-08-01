use serde::{Deserialize, Deserializer, Serialize};
pub enum Interval {
    Seconds1,
    Minutes1,
    Minutes3,
    Minutes5,
    Minutes15,
    Minutes30,
    Hours1,
    Hours2,
    Hours4,
    Hours6,
    Hours8,
    Hours12,
    Days1,
    Days3,
    Weeks1,
    Months1,
}

impl AsRef<str> for Interval {
    fn as_ref(&self) -> &'static str {
        match self {
            Interval::Seconds1 => "1s",
            Interval::Minutes1 => "1m",
            Interval::Minutes3 => "3m",
            Interval::Minutes5 => "5m",
            Interval::Minutes15 => "15m",
            Interval::Minutes30 => "30m",
            Interval::Hours1 => "1h",
            Interval::Hours2 => "2h",
            Interval::Hours4 => "4h",
            Interval::Hours6 => "6h",
            Interval::Hours8 => "8h",
            Interval::Hours12 => "12h",
            Interval::Days1 => "1d",
            Interval::Days3 => "3d",
            Interval::Weeks1 => "1w",
            Interval::Months1 => "1M",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyType {
    Oco,
    Oto,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AllocationType {
    Sor,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    LimitMaker,
    Market,
    Stop,
    StopLoss,
    StopLossLimit,
    StopMarket,
    TakeProfit,
    TakeProfitLimit,
    TakeProfitMarket,
    TrailingStopMarket,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderResponseType {
    Ack,
    Result,
    Full,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WorkingFloor {
    Exchange,
    Sor,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Default for OrderSide {
    fn default() -> Self {
        OrderSide::Buy
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StpModes {
    None,
    ExpireMaker,
    ExpireTaker,
    ExpireBoth,
    Decrement,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelRestrictions {
    OnlyNew,
    OnlyPartiallyFilled,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    CurrentQuarter,
    NextQuarter,
    Perpetual,
}

impl Default for ContractType {
    fn default() -> Self {
        ContractType::Perpetual
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

impl Default for PositionSide {
    fn default() -> Self {
        PositionSide::Both
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    ContractPrice,
    MarkPrice,
}

impl Default for WorkingType {
    fn default() -> Self {
        WorkingType::ContractPrice
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceMatch {
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
        let val: u8 = Deserialize::deserialize(deserializer)?;
        Ok(match val {
            0 => AdlLevel::LowestRisk,
            1 => AdlLevel::LowRisk,
            2 => AdlLevel::Medium,
            3 => AdlLevel::HighRisk,
            4 => AdlLevel::HighestRisk,
            _ => return Err(serde::de::Error::custom("Invalid ADL level")),
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IncomeType {
    Transfer,
    WelcomeBonus,
    RealizedPnl,
    FundingFee,
    Commission,
    InsuranceClear,
    ReferralKickback,
    CommissionRebate,
    ApiRebate,
    ContestReward,
    CrossCollateralTransfer,
    OptionsPremiumFee,
    OptionsSettleProfit,
    InternalTransfer,
    AutoExchange,
    DeliveredSettelment,
    CoinSwapDeposit,
    CoinSwapWithdraw,
    PositionLimitIncreaseFee,
    StrategyUmfuturesTransfer,
    FeeReturn,
    BfusdReward,
}

#[derive(Debug)]
pub enum WebSocketCommand {
    Connect(String),
    Disconnect,
    Close,
    Reconnect,
}

#[derive(Debug)]
pub enum WebSocketStatus {
    Connected,
    Disconnected,
    Closed,
    Idle,
}

pub enum WebSocketStreamRate {
    Milliseconds100,
    Milliseconds250,
    Milliseconds500,
    Seconds1,
    Seconds3,
}

impl std::fmt::Display for WebSocketStreamRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rate: &'static str = match self {
            WebSocketStreamRate::Milliseconds100 => "100ms",
            WebSocketStreamRate::Milliseconds250 => "250ms",
            WebSocketStreamRate::Milliseconds500 => "500ms",
            WebSocketStreamRate::Seconds1 => "1s",
            WebSocketStreamRate::Seconds3 => "3s",
        };

        write!(f, "{}", rate)
    }
}

#[derive(Debug)]
pub enum WebSocketType {
    SingleStream,
    MultiStream,
}
