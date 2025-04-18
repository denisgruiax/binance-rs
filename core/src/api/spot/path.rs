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
    QueryOrderUserData,
    CancelOrderTrade,
    CancelAllOpenOrdersOnASymbol,
    CancelAnExistingOrderAndSendANewOrderTrade,
    OrderAmendKeepPriorityTrade,
    CurrentOpenOrdersUserData,
    AllOrdersUserData,
    NewOrderListOcoTrade,
    NewOrderListOtoTrade,
    NewOrderListOtocoTrade,
    QueryOrderListUserData,
    QueryAllOrderListsUserData,
    QueryOpenOrderListsUserData,
    NewOrderUsingSORTrade,
    TestNewOrderUsingSorTrade,
}

impl From<Trading> for &'static str {
    fn from(value: Trading) -> Self {
        match value {
            Trading::NewOrder => "/api/v3/order",
            Trading::TestNewOrder => "/api/v3/order/test",
            Trading::QueryOrderUserData => "/api/v3/order",
            Trading::CancelOrderTrade => "/api/v3/order",
            Trading::CancelAllOpenOrdersOnASymbol => "/api/v3/openOrders",
            Trading::CancelAnExistingOrderAndSendANewOrderTrade => "/api/v3/order/cancelReplace",
            Trading::OrderAmendKeepPriorityTrade => "/api/v3/order/amend/keepPriority",
            Trading::CurrentOpenOrdersUserData => "/api/v3/openOrders",
            Trading::AllOrdersUserData => "/api/v3/allOrders",
            Trading::NewOrderListOcoTrade => "/api/v3/orderList/oco",
            Trading::NewOrderListOtoTrade => "/api/v3/orderList/oto",
            Trading::NewOrderListOtocoTrade => "/api/v3/orderList/otoco",
            Trading::QueryOrderListUserData => "/api/v3/orderList",
            Trading::QueryAllOrderListsUserData => "/api/v3/allOrderList",
            Trading::QueryOpenOrderListsUserData => "/api/v3/openOrderList",
            Trading::NewOrderUsingSORTrade => "/api/v3/sor/order",
            Trading::TestNewOrderUsingSorTrade => "/api/v3/sor/order/test",
        }
    }
}
