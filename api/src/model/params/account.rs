use serde::Serialize;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoParams {
    pub omit_zero_balances: Option<bool>,
    pub recv_window: Option<u16>,
}

impl InfoParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn omit_zero_balances(mut self, omit_zero_balances: bool) -> Self {
        self.omit_zero_balances = Some(omit_zero_balances);
        self
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTradesParams<'a> {
    pub symbol: &'a str,
    pub order_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>,
    pub limit: Option<u16>,
    pub recv_window: Option<u16>,
}

impl<'a> MyTradesParams<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnfilledOrderCountParams {
    pub recv_window: Option<u16>,
}

impl UnfilledOrderCountParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}
