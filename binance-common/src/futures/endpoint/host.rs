pub enum Host {
    Api,
    Test,
}

impl AsRef<str> for Host {
    fn as_ref(&self) -> &str {
        match self {
            Host::Api => "https://fapi.binance.com",
            Host::Test => "https://testnet.binancefuture.com",
        }
    }
}
