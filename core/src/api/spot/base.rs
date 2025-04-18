pub enum Base {
    Api,
    ApiGcp,
    Api1,
    Api2,
    Api3,
    Api4,
}

impl From<Base> for &'static str {
    fn from(value: Base) -> Self {
        match value {
            Base::Api => "https://api.binance.com",
            Base::ApiGcp => "https://api-gcp.binance.com",
            Base::Api1 => "https://api1.binance.com",
            Base::Api2 => "https://api2.binance.com",
            Base::Api3 => "https://api3.binance.com",
            Base::Api4 => "https://api4.binance.com",
        }
    }
}