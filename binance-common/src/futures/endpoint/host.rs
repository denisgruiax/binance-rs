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

pub enum WebSocketHost {
    SingleStream,
    CombinedStreams,
}

impl AsRef<str> for WebSocketHost {
    fn as_ref(&self) -> &str {
        match self {
            WebSocketHost::SingleStream => "wss://fstream.binance.com/ws/",
            WebSocketHost::CombinedStreams => "wss://fstream.binance.com/stream?streams=",
        }
    }
}
