use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct BinanceError {
    pub code: i16,
    pub msg: String,
}

impl Display for BinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {},\nmsg: {},\n", self.code, self.msg)
    }
}
