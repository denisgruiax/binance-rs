use serde::Serialize;

use crate::enums::IncomeType;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesBalanceParams {
    pub recv_window: Option<u16>,
}

impl FuturesBalanceParams {
    pub fn new(recv_window: u16) -> Self {
        FuturesBalanceParams {
            recv_window: Some(recv_window),
        }
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSideParams {
    pub recv_window: Option<u16>,
}

impl PositionSideParams {
    pub fn new(recv_window: u16) -> Self {
        PositionSideParams {
            recv_window: Some(recv_window),
        }
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountParams {
    pub recv_window: Option<u16>,
}

impl FuturesAccountParams {
    pub fn new(recv_window: u16) -> Self {
        FuturesAccountParams {
            recv_window: Some(recv_window),
        }
    }

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRateParams<'a> {
    pub symbol: &'a str,
    pub recv_window: Option<u16>,
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

    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryParams<'a> {
    pub symbol: Option<&'a str>,
    pub income_type: Option<IncomeType>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub page: Option<u16>,
    pub limit: Option<u16>,
    pub recv_window: Option<u16>,
}

impl<'a> IncomeHistoryParams<'a> {
    pub fn new() -> Self {
        IncomeHistoryParams::default()
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn income_type(mut self, income_type: IncomeType) -> Self {
        self.income_type = Some(income_type);
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

    pub fn page(mut self, page: u16) -> Self {
        self.page = Some(page);
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
