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
