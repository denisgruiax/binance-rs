pub mod futures;
pub mod spot;

use serde::{Deserialize, Serialize};
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

//PARTIAL IMPLEMENTATION
impl<'de> Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;

        Ok(match value.as_str() {
            "1s" => Interval::Seconds1,
            _ => return Err(serde::de::Error::custom("Invalid interval")),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RateLimits {
    Second,
    Minute,
    Day,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelRestrictions {
    OnlyNew,
    OnlyPartiallyFilled,
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
