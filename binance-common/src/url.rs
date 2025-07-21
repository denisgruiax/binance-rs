use serde::Serialize;

pub trait UrlEncoded: Serialize {
    fn to_url_encoded(&self) -> String {
        serde_urlencoded::to_string(self).unwrap_or_default()
    }
}

impl<T: Serialize> UrlEncoded for T {}
