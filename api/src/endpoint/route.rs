pub enum General {
    Ping,
    ServerTime,
    ExchangeInfo,
}

impl From<General> for &'static str {
    fn from(value: General) -> Self {
        match value {
            General::Ping => "/api/v3/ping?",
            General::ServerTime => "/api/v3/time?",
            General::ExchangeInfo => "/api/v3/exchangeInfo?",
        }
    }
}

pub enum Market {
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    UIKlines,
    AvgPrice,
    Ticker24h,
    TickerDay,
    PriceTicker,
    BookTicker,
    RollingTicker,
}

impl From<Market> for &'static str {
    fn from(value: Market) -> Self {
        match value {
            Market::Depth => "/api/v3/depth?",
            Market::Trades => "/api/v3/trades?",
            Market::HistoricalTrades => "/api/v3/historicalTrades?",
            Market::AggTrades => "/api/v3/aggTrades?",
            Market::Klines => "/api/v3/klines?",
            Market::UIKlines => "/api/v3/uiKlines?",
            Market::AvgPrice => "/api/v3/avgPrice?",
            Market::Ticker24h => "/api/v3/ticker/24hr?",
            Market::TickerDay => "/api/v3/ticker/tradingDay?",
            Market::PriceTicker => "/api/v3/ticker/price?",
            Market::BookTicker => "/api/v3/ticker/bookTicker?",
            Market::RollingTicker => "/api/v3/ticker?",
        }
    }
}

pub enum Trading {
    NewOrder,
    TestOrder,
    GetOrder,
    CancelOrder,
    CancelAllOrders,
    CancelReplaceOrder,
    AmendOrderPriority,
    OpenOrders,
    AllOrders,
    OcoOrderList,
    OtoOrderList,
    OtocoOrderList,
    GetOrderList,
    AllOrderLists,
    OpenOrderLists,
    SorOrder,
    TestSorOrder,
}

impl From<Trading> for &'static str {
    fn from(value: Trading) -> Self {
        match value {
            Trading::NewOrder => "/api/v3/order?",
            Trading::TestOrder => "/api/v3/order/test?",
            Trading::GetOrder => "/api/v3/order?",
            Trading::CancelOrder => "/api/v3/order?",
            Trading::CancelAllOrders => "/api/v3/openOrders?",
            Trading::CancelReplaceOrder => "/api/v3/order/cancelReplace?",
            Trading::AmendOrderPriority => "/api/v3/order/amend/keepPriority?",
            Trading::OpenOrders => "/api/v3/openOrders?",
            Trading::AllOrders => "/api/v3/allOrders?",
            Trading::OcoOrderList => "/api/v3/orderList/oco?",
            Trading::OtoOrderList => "/api/v3/orderList/oto?",
            Trading::OtocoOrderList => "/api/v3/orderList/otoco?",
            Trading::GetOrderList => "/api/v3/orderList?",
            Trading::AllOrderLists => "/api/v3/allOrderList?",
            Trading::OpenOrderLists => "/api/v3/openOrderList?",
            Trading::SorOrder => "/api/v3/sor/order?",
            Trading::TestSorOrder => "/api/v3/sor/order/test?",
        }
    }
}

pub enum Account {
    Info,
    MyTrades,
    UnfilledOrderCount,
    PreventedMatches,
    Allocations,
    CommissionRates,
    Amendments,
}

impl From<Account> for &'static str {
    fn from(value: Account) -> Self {
        match value {
            Account::Info => "/api/v3/account?",
            Account::MyTrades => "/api/v3/myTrades?",
            Account::UnfilledOrderCount => "/api/v3/rateLimit/order?",
            Account::PreventedMatches => "/api/v3/myPreventedMatches?",
            Account::Allocations => "/api/v3/myAllocations?",
            Account::CommissionRates => "/api/v3/account/commission?",
            Account::Amendments => "/api/v3/order/amendments?",
        }
    }
}