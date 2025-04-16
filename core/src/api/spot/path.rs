pub enum General{
    TestConnectivity
}

impl From<General> for &'static str {
    fn from(value: General) -> Self {
        match value {
            General::TestConnectivity => "/api/v3/ping"
        }
    }
}