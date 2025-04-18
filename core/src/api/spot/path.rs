pub enum General {
    TestConnectivity,
    CheckServerTime,
    ExchangeInformation,
}

impl From<General> for &'static str {
    fn from(value: General) -> Self {
        match value {
            General::TestConnectivity => "/api/v3/ping",
            General::CheckServerTime => "/api/v3/time",
            General::ExchangeInformation => "/api/v3/exchangeInfo",
        }
    }
}

pub enum Market {
    OrderBook,
    RecentTradeList,
    OldTradeLookup,
    CompressedAggregateTradesList,
    KlineCandlestickData,
    UIKlines,
    CurrentAveragePrice,
    TickerPriceChangeStatistics24Hr,
    TradingDayTicker,
    SymbolPriceTicker,
    SymbolOrderBookTicker,
    RollingWindowPriceChangeStatistics,
}

impl From<Market> for &'static str {
    fn from(value: Market) -> Self {
        match value {
            Market::OrderBook => "/api/v3/depth",
            Market::RecentTradeList => "/api/v3/trades",
            Market::OldTradeLookup => "/api/v3/historicalTrades",
            Market::CompressedAggregateTradesList => "/api/v3/aggTrades",
            Market::KlineCandlestickData => "/api/v3/klines",
            Market::UIKlines => "/api/v3/uiKlines",
            Market::CurrentAveragePrice => "/api/v3/avgPrice",
            Market::TickerPriceChangeStatistics24Hr => "/api/v3/ticker/24hr",
            Market::TradingDayTicker => "/api/v3/ticker/tradingDay",
            Market::SymbolPriceTicker => "/api/v3/ticker/price",
            Market::SymbolOrderBookTicker => "/api/v3/ticker/bookTicker",
            Market::RollingWindowPriceChangeStatistics => "/api/v3/ticker",
        }
    }
}

pub enum Trading {
    NewOrder,
    TestNewOrder,
    QueryOrder,
    CancelOrder,
    CancelAllOpenOrdersOnASymbol,
    CancelAnExistingOrderAndSendANewOrder,
    OrderAmendKeepPriority,
    CurrentOpenOrders,
    AllOrders,
    NewOrderListOco,
    NewOrderListOto,
    NewOrderListOtoco,
    QueryOrderList,
    QueryAllOrderLists,
    QueryOpenOrderLists,
    NewOrderUsingSOR,
    TestNewOrderUsingSor,
}

impl From<Trading> for &'static str {
    fn from(value: Trading) -> Self {
        match value {
            Trading::NewOrder => "/api/v3/order",
            Trading::TestNewOrder => "/api/v3/order/test",
            Trading::QueryOrder => "/api/v3/order",
            Trading::CancelOrder => "/api/v3/order",
            Trading::CancelAllOpenOrdersOnASymbol => "/api/v3/openOrders",
            Trading::CancelAnExistingOrderAndSendANewOrder => "/api/v3/order/cancelReplace",
            Trading::OrderAmendKeepPriority => "/api/v3/order/amend/keepPriority",
            Trading::CurrentOpenOrders => "/api/v3/openOrders",
            Trading::AllOrders => "/api/v3/allOrders",
            Trading::NewOrderListOco => "/api/v3/orderList/oco",
            Trading::NewOrderListOto => "/api/v3/orderList/oto",
            Trading::NewOrderListOtoco => "/api/v3/orderList/otoco",
            Trading::QueryOrderList => "/api/v3/orderList",
            Trading::QueryAllOrderLists => "/api/v3/allOrderList",
            Trading::QueryOpenOrderLists => "/api/v3/openOrderList",
            Trading::NewOrderUsingSOR => "/api/v3/sor/order",
            Trading::TestNewOrderUsingSor => "/api/v3/sor/order/test",
        }
    }
}

pub enum Account {
    AccountInformation,
    AccountTradeList,
    QueryUnfilledOrderCount,
    QueryPreventedMatches,
    QueryAllocations,
    QueryCommissionRates,
    QueryOrderAmendments,
}

impl From<Account> for &'static str {
    fn from(value: Account) -> Self {
        match value {
            Account::AccountInformation => "/api/v3/account",
            Account::AccountTradeList => "/api/v3/myTrades",
            Account::QueryUnfilledOrderCount => "/api/v3/rateLimit/order",
            Account::QueryPreventedMatches => "/api/v3/myPreventedMatches",
            Account::QueryAllocations => "/api/v3/myAllocations",
            Account::QueryCommissionRates => "/api/v3/account/commission",
            Account::QueryOrderAmendments => "/api/v3/order/amendments",
        }
    }
}
