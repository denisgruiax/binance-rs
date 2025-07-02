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
