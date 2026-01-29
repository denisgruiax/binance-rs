use std::fmt;

use crate::{
    enums::{Interval, WebSocketStreamRate},
    futures::endpoint::host::WebSocketHost,
};

#[derive(Debug)]
pub enum WebSocketSymbol {
    AdaUsdc,
    AdaUsdt,
    AvaxUsdc,
    AvaxUsdt,
    BnbUsdc,
    BnbUsdt,
    BchUsdt,
    BtcUsdc,
    BtcUsdt,
    CfxUsdt,
    DogeUsdc,
    DogeUsdt,
    EgldUsdt,
    EnaUsdt,
    EthUsdc,
    EthUsdt,
    EthBusd,
    FilUsdc,
    IcpUsdt,
    LinkUsdc,
    LinkUsdt,
    LtcUsdc,
    LtcUsdt,
    MaticUsdc,
    MaticUsdt,
    SolUsdc,
    SolUsdt,
    DotBusd,
    DotUsdt,
    XrpUsdc,
    XrpUsdt,
    PenguUsdt,
}

impl fmt::Display for WebSocketSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol: &'static str = match self {
            WebSocketSymbol::AdaUsdc => "adausdc",
            WebSocketSymbol::AdaUsdt => "adausdt",
            WebSocketSymbol::AvaxUsdc => "avaxusdc",
            WebSocketSymbol::AvaxUsdt => "avaxusdt",
            WebSocketSymbol::BnbUsdc => "bnbusdc",
            WebSocketSymbol::BnbUsdt => "bnbusdt",
            WebSocketSymbol::BchUsdt => "bchusdt",
            WebSocketSymbol::BtcUsdc => "btcusdc",
            WebSocketSymbol::BtcUsdt => "btcusdt",
            WebSocketSymbol::CfxUsdt => "cfxusdt",
            WebSocketSymbol::DogeUsdc => "dogeusdc",
            WebSocketSymbol::DogeUsdt => "dogeusdt",
            WebSocketSymbol::EnaUsdt => "enausdt",
            WebSocketSymbol::EgldUsdt => "egldusdt",
            WebSocketSymbol::EthUsdc => "ethusdc",
            WebSocketSymbol::EthUsdt => "ethusdt",
            WebSocketSymbol::EthBusd => "ethbusd",
            WebSocketSymbol::FilUsdc => "filusdc",
            WebSocketSymbol::IcpUsdt => "icpusdt",
            WebSocketSymbol::LinkUsdc => "linkusdc",
            WebSocketSymbol::LinkUsdt => "linkusdt",
            WebSocketSymbol::LtcUsdc => "ltcusdc",
            WebSocketSymbol::LtcUsdt => "ltcusdt",
            WebSocketSymbol::MaticUsdc => "maticusdc",
            WebSocketSymbol::MaticUsdt => "maticusdt",
            WebSocketSymbol::SolUsdc => "solusdc",
            WebSocketSymbol::SolUsdt => "solusdt",
            WebSocketSymbol::DotBusd => "dotbusd",
            WebSocketSymbol::DotUsdt => "dotusdt",
            WebSocketSymbol::XrpUsdc => "xrpusdc",
            WebSocketSymbol::XrpUsdt => "xrpusdt",
            WebSocketSymbol::PenguUsdt => "penguusdt",
        };
        write!(f, "{}", symbol)
    }
}

pub struct WebSocketParams {
    pub route: String,
}

impl WebSocketParams {
    pub fn new(host: WebSocketHost) -> Self {
        WebSocketParams {
            route: host.as_ref().to_string(),
        }
    }

    pub fn agg_trade(mut self, symbol: WebSocketSymbol) -> Self {
        self.route.push_str(format!("{}@aggTrade", symbol).as_str());
        self
    }

    pub fn mark_price(mut self, symbol: WebSocketSymbol, rate: WebSocketStreamRate) -> Self {
        self.route
            .push_str(format!("{}@markPrice@{}", symbol, rate).as_str());
        self
    }

    pub fn mark_price_all(mut self, rate: WebSocketStreamRate) -> Self {
        self.route
            .push_str(format!("!@markPrice@{}", rate).as_str());
        self
    }

    pub fn kline_candlesticks(mut self, symbol: WebSocketSymbol, interval: Interval) -> Self {
        self.route
            .push_str(format!("{}@kline_{}", symbol, interval.as_ref()).as_str());
        self
    }

    pub fn symbol_ticker_mini(mut self, symbol: WebSocketSymbol) -> Self {
        self.route
            .push_str(format!("{}@miniTicker", symbol).as_str());
        self
    }

    pub fn symbol_ticker_all(mut self) -> Self {
        self.route.push_str("!ticker@arr");
        self
    }

    pub fn symbol_ticker(mut self, symbol: WebSocketSymbol) -> Self {
        self.route.push_str(format!("{}@ticker", symbol).as_str());
        self
    }

    pub fn symbol_ticker_all_mini(mut self) -> Self {
        self.route.push_str("!miniTicker@arr");
        self
    }

    pub fn all_book_tickers(mut self) -> Self {
        self.route.push_str("!bookTicker");
        self
    }

    pub fn liquidation_order(mut self, symbol: WebSocketSymbol) -> Self {
        self.route
            .push_str(format!("{}@forceOrder", symbol).as_str());
        self
    }

    pub fn all_market_liquidation_order(mut self) -> Self {
        self.route.push_str("!forceOrder@arr");
        self
    }

    pub fn partial_book_depth(
        mut self,
        symbol: WebSocketSymbol,
        levels: u8,
        milliseconds: WebSocketStreamRate,
    ) -> Self {
        self.route
            .push_str(format!("{}@depth{}@{}", symbol, levels, milliseconds).as_str());
        self
    }
}
