#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestDetails {
    method: String,
    url: String,
}

impl RequestDetails {
    pub fn new(method: String, url: String) -> RequestDetails {
        RequestDetails { method, url }
    }
}
