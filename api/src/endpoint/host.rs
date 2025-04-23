pub enum Host {
    Api,
    ApiGcp,
    Api1,
    Api2,
    Api3,
    Api4,
}

impl From<Host> for &'static str {
    fn from(value: Host) -> Self {
        match value {
            Host::Api => "https://api.binance.com",
            Host::ApiGcp => "https://api-gcp.binance.com",
            Host::Api1 => "https://api1.binance.com",
            Host::Api2 => "https://api2.binance.com",
            Host::Api3 => "https://api3.binance.com",
            Host::Api4 => "https://api4.binance.com",
        }
    }
}