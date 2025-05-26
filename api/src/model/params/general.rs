use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ExchangeInformationParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<&'a str>,
}

impl<'a> ExchangeInformationParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbols(mut self, symbols: &'a str) -> Self {
        self.symbols = Some(symbols);
        self
    }

    pub fn permissions(mut self, permissions: &'a str) -> Self {
        self.permissions = Some(permissions);
        self
    }
}
