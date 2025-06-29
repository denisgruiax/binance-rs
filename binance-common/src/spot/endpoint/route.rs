pub enum General {
    Ping,
    ServerTime,
    ExchangeInfo,
}

impl AsRef<str> for General {
    fn as_ref(&self) -> &'static str {
        match self {
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
    TickerPrice,
    BookTicker,
    RollingTicker,
}

impl AsRef<str> for Market {
    fn as_ref(&self) -> &'static str {
        match self {
            Market::Depth => "/api/v3/depth?",
            Market::Trades => "/api/v3/trades?",
            Market::HistoricalTrades => "/api/v3/historicalTrades?",
            Market::AggTrades => "/api/v3/aggTrades?",
            Market::Klines => "/api/v3/klines?",
            Market::UIKlines => "/api/v3/uiKlines?",
            Market::AvgPrice => "/api/v3/avgPrice?",
            Market::Ticker24h => "/api/v3/ticker/24hr?",
            Market::TickerDay => "/api/v3/ticker/tradingDay?",
            Market::TickerPrice => "/api/v3/ticker/price?",
            Market::BookTicker => "/api/v3/ticker/bookTicker?",
            Market::RollingTicker => "/api/v3/ticker?",
        }
    }
}

pub enum Trade {
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

impl AsRef<str> for Trade {
    fn as_ref(&self) -> &'static str {
        match self {
            Trade::NewOrder => "/api/v3/order?",
            Trade::TestOrder => "/api/v3/order/test?",
            Trade::GetOrder => "/api/v3/order?",
            Trade::CancelOrder => "/api/v3/order?",
            Trade::CancelAllOrders => "/api/v3/openOrders?",
            Trade::CancelReplaceOrder => "/api/v3/order/cancelReplace?",
            Trade::AmendOrderPriority => "/api/v3/order/amend/keepPriority?",
            Trade::OpenOrders => "/api/v3/openOrders?",
            Trade::AllOrders => "/api/v3/allOrders?",
            Trade::OcoOrderList => "/api/v3/orderList/oco?",
            Trade::OtoOrderList => "/api/v3/orderList/oto?",
            Trade::OtocoOrderList => "/api/v3/orderList/otoco?",
            Trade::GetOrderList => "/api/v3/orderList?",
            Trade::AllOrderLists => "/api/v3/allOrderList?",
            Trade::OpenOrderLists => "/api/v3/openOrderList?",
            Trade::SorOrder => "/api/v3/sor/order?",
            Trade::TestSorOrder => "/api/v3/sor/order/test?",
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

impl AsRef<str> for Account {
    fn as_ref(&self) -> &'static str {
        match self {
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
