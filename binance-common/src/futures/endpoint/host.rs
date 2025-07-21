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

pub enum WsHost {
    SingleStream,
    CombinedStreams,
}

impl AsRef<str> for WsHost {
    fn as_ref(&self) -> &str {
        match self {
            WsHost::SingleStream => "wss://fstream.binance.com/ws/",
            WsHost::CombinedStreams => "wss://fstream.binance.com/stream?streams=",
        }
    }
}
