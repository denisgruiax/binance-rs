pub enum Host {
    Api,
    ApiGcp,
    Api1,
    Api2,
    Api3,
    Api4,
}

impl AsRef<str> for Host {
    fn as_ref(&self) -> &'static str {
        match self {
            Host::Api => "https://api.binance.com",
            Host::ApiGcp => "https://api-gcp.binance.com",
            Host::Api1 => "https://api1.binance.com",
            Host::Api2 => "https://api2.binance.com",
            Host::Api3 => "https://api3.binance.com",
            Host::Api4 => "https://api4.binance.com",
        }
    }
}