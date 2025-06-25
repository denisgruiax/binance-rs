use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct DepthParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

impl<'a> DepthParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TradesParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

impl<'a> TradesParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTradesParams<'a> {
    pub symbol: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
}

impl<'a> HistoricalTradesParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn from_if(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlinesParams<'a> {
    pub symbol: &'a str,
    pub interval: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

impl<'a> KlinesParams<'a> {
    pub fn new(symbol: &'a str, interval: &'a impl AsRef<str>) -> Self {
        Self {
            symbol,
            interval: interval.as_ref(),
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn interval(mut self, interval: &'a str) -> Self {
        self.interval = interval;
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn time_zone(mut self, time_zone: &'a str) -> Self {
        self.time_zone = Some(time_zone);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Serialize)]
pub struct AvgPriceParams<'a> {
    pub symbol: &'a str,
}

impl<'a> AvgPriceParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Self { symbol }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Ticker24hParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<&'a str>,
}

impl<'a> Ticker24hParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbols(mut self, symbols: &'a str) -> Self {
        self.symbols = Some(symbols);
        self
    }

    pub fn r#type(mut self, r#type: &'a str) -> Self {
        self.r#type = Some(r#type);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TickerDayParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<&'a str>,
}

impl<'a> TickerDayParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbols(mut self, symbols: &'a str) -> Self {
        self.symbols = Some(symbols);
        self
    }

    pub fn time_zone(mut self, time_zone: &'a str) -> Self {
        self.time_zone = Some(time_zone);
        self
    }

    pub fn r#type(mut self, r#type: &'a str) -> Self {
        self.r#type = Some(r#type);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PriceTickerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,
}

impl<'a> PriceTickerParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbols(mut self, symbols: &'a str) -> Self {
        self.symbols = Some(symbols);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct BookTickerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,
}

impl<'a> BookTickerParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbols(mut self, symbols: &'a str) -> Self {
        self.symbols = Some(symbols);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RollingTickerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<&'a str>,
}

impl<'a> RollingTickerParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbols(mut self, symbols: &'a str) -> Self {
        self.symbols = Some(symbols);
        self
    }

    pub fn window_size(mut self, window_size: &'a str) -> Self {
        self.window_size = Some(window_size);
        self
    }

    pub fn r#type(mut self, r#type: &'a str) -> Self {
        self.r#type = Some(r#type);
        self
    }
}
