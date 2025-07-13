pub enum Market {
    Ping,
    ServerTime,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggregateTrades,
    Klines,
    MarkPrice,
    FundingRateHistory,
    FundingInfo,
    Ticker24h,
    TickerPrice,
    TickerPriceV2,
    BookTicker,
    DeliveryPrice,
    OpenInterest,
    OpenInterestHistory,
    TopLongShortPositionRatio,
    TopLongShortAccountRatio,
    GlobalLongShortAccountRatio,
    TakerBuySellVolume,
    Basis,
}

impl AsRef<str> for Market {
    fn as_ref(&self) -> &'static str {
        match self {
            Market::Ping => "/fapi/v1/ping?",
            Market::ServerTime => "/fapi/v1/time?",
            Market::ExchangeInfo => "/fapi/v1/exchangeInfo?",
            Market::Depth => "/fapi/v1/depth?",
            Market::Trades => "/fapi/v1/trades?",
            Market::HistoricalTrades => "/fapi/v1/historicalTrades?",
            Market::AggregateTrades => "/fapi/v1/aggTrades?",
            Market::Klines => "/fapi/v1/klines?",
            Market::MarkPrice => "/fapi/v1/premiumIndex?",
            Market::FundingRateHistory => "/fapi/v1/fundingRate?",
            Market::FundingInfo => "/fapi/v1/fundingInfo?",
            Market::Ticker24h => "/fapi/v1/ticker/24hr?",
            Market::TickerPrice => "/fapi/v1/ticker/price?",
            Market::TickerPriceV2 => "/fapi/v2/ticker/price?",
            Market::BookTicker => "/fapi/v1/ticker/bookTicker?",
            Market::DeliveryPrice => "/futures/data/delivery-price?",
            Market::OpenInterest => "/fapi/v1/openInterest?",
            Market::OpenInterestHistory => "/futures/data/openInterestHist?",
            Market::TopLongShortPositionRatio => "/futures/data/topLongShortPositionRatio?",
            Market::TopLongShortAccountRatio => "/futures/data/topLongShortAccountRatio?",
            Market::GlobalLongShortAccountRatio => "/futures/data/globalLongShortAccountRatio?",
            Market::TakerBuySellVolume => "/futures/data/takerlongshortRatio?",
            Market::Basis => "/futures/data/basis?",
        }
    }
}

pub enum Trade {
    NewOrder,
    MultipleOrders,
    ModifyOrder,
    ModifyMultipleOrders,
    OrderModificationHistory,
    CancelOrder,
    CancelMultipleOrders,
    CancelAllOrders,
    AutoCancelAllOpenOrders,
    GetOrder,
    AllOrders,
    OpenOrders,
    GetOpenOrder,
    ForceOrders,
    UserTrades,
    SetMarginType,
    SetPositionSide,
    SetLeverage,
    SetMultiAssetsMargin,
    UpdatePositionMargin,
    PositionRisk2,
    PositionRisk3,
    GetPositionMarginHistory,
    TestNewOrder,
}

impl AsRef<str> for Trade {
    fn as_ref(&self) -> &'static str {
        match self {
            Trade::NewOrder => "/fapi/v1/order?",
            Trade::MultipleOrders => "/fapi/v1/batchOrders?",
            Trade::ModifyOrder => "/fapi/v1/amendOrder?",
            Trade::ModifyMultipleOrders => "/fapi/v1/amendBatchOrders?",
            Trade::OrderModificationHistory => "/fapi/v1/orderAmendmentHistory?",
            Trade::CancelOrder => "/fapi/v1/cancelOrder?",
            Trade::CancelMultipleOrders => "/fapi/v1/cancelBatchOrders?",
            Trade::CancelAllOrders => "/fapi/v1/allOpenOrders?",
            Trade::AutoCancelAllOpenOrders => "/fapi/v1/autoCancelAllOpenOrders?",
            Trade::GetOrder => "/fapi/v1/order?",
            Trade::AllOrders => "/fapi/v1/allOrders?",
            Trade::OpenOrders => "/fapi/v1/openOrders?",
            Trade::GetOpenOrder => "/fapi/v1/openOrder?",
            Trade::ForceOrders => "/fapi/v1/forceOrders?",
            Trade::UserTrades => "/fapi/v1/userTrades?",
            Trade::SetMarginType => "/fapi/v1/marginType?symbol=",
            Trade::SetPositionSide => "/fapi/v1/positionSide/dual?",
            Trade::SetLeverage => "/fapi/v1/leverage?",
            Trade::SetMultiAssetsMargin => "/vapi/multiAssetsMargin?symbol=",
            Trade::UpdatePositionMargin => "/vapi/updatePositionMargin?symbol=",
            Trade::PositionRisk2 => "/fapi/v2/positionRisk",
            Trade::PositionRisk3 => "/fapi/v3/positionRisk",
            Trade::GetPositionMarginHistory => "/vapi/positionMarginHistory?symbol=",
            Trade::TestNewOrder => "/fapi/v1/order/test?",
        }
    }
}

pub enum Account {
    NewTransfer,
    FuturesBalance,
    FuturesBalanceV3,
    FuturesAccount,
    TransactionHistory,
    CommissionRate,
    Configuration,
    SymbolConfiguration,
    OrderRateLimit,
    LeverageBracket,
    MultiAssetsMargin,
    PositionSide,
    IncomeHistory,
}

impl AsRef<str> for Account {
    fn as_ref(&self) -> &'static str {
        match self {
            Account::NewTransfer => "/fapi/v1/transfer?",
            Account::FuturesBalance => "/fapi/v2/balance?",
            Account::FuturesBalanceV3 => "/fapi/v3/balance?",
            Account::FuturesAccount => "/fapi/v3/account?",
            Account::TransactionHistory => "/fapi/v1/transactionHistory?",
            Account::CommissionRate => "/fapi/v1/commissionRate?",
            Account::Configuration => "/fapi/v1/configuration?",
            Account::SymbolConfiguration => "/fapi/v1/symbolConfiguration?symbol=",
            Account::OrderRateLimit => "/fapi/v1/orderRateLimit?symbol=",
            Account::LeverageBracket => "/fapi/v1/leverageBracket?symbol=",
            Account::MultiAssetsMargin => "/vapi/multiAssetsMarginConfig?symbol=",
            Account::PositionSide => "/fapi/v1/positionSide/dual?",
            Account::IncomeHistory => "/vapi/incomeHistory?symbol=",
        }
    }
}
