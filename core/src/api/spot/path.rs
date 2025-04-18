pub enum General{
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