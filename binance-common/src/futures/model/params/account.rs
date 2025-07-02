use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesBalanceParams {
    pub recv_window: Option<u32>,
}

impl FuturesBalanceParams {
    pub fn new(recv_window: u32) -> Self {
        FuturesBalanceParams {
            recv_window: Some(recv_window),
        }
    }

    pub fn recv_window(mut self, recv_window: u32) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSideParams {
    pub recv_window: Option<u32>,
}

impl PositionSideParams {
    pub fn new(recv_window: u32) -> Self {
        PositionSideParams {
            recv_window: Some(recv_window),
        }
    }

    pub fn recv_window(mut self, recv_window: u32) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountParams {
    pub recv_window: Option<u32>,
}

impl FuturesAccountParams {
    pub fn new(recv_window: u32) -> Self {
        FuturesAccountParams {
            recv_window: Some(recv_window),
        }
    }

    pub fn recv_window(mut self, recv_window: u32) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRateParams<'a> {
    pub symbol: &'a str,
    pub recv_window: Option<u32>,
}

impl<'a> CommissionRateParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        CommissionRateParams {
            symbol,
            ..Default::default()
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn recv_window(mut self, recv_window: u32) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}
