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
    SingleStreamPublic,
    SingleStreamMarket,
    SingleStreamPrivate,
    CombinedStreamPublic,
    CombinedStreamMarket,
    CombinedStreamPrivate,
}

impl AsRef<str> for WebSocketHost {
    fn as_ref(&self) -> &str {
        match self {
            WebSocketHost::SingleStreamPublic => "wss://fstream.binance.com/public/",
            WebSocketHost::SingleStreamMarket => "wss://fstream.binance.com/market/",
            WebSocketHost::SingleStreamPrivate => "wss://fstream.binance.com/private/",
            WebSocketHost::CombinedStreamPublic => {
                "wss://fstream.binance.com/public/stream?streams="
            }
            WebSocketHost::CombinedStreamMarket => {
                "wss://fstream.binance.com/market/stream?streams="
            }
            WebSocketHost::CombinedStreamPrivate => {
                "wss://fstream.binance.com/private/stream?streams="
            }
        }
    }
}
