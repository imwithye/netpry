use http::{uri::Uri, HeaderMap, Method, StatusCode};
use time::serde::rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

mod base64_serde {
    use base64::Engine;
    use serde::Serialize;
    use serde::Serializer;

    pub fn serialize<S: Serializer>(v: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error> {
        if let Some(v) = v {
            let base64 = base64::prelude::BASE64_STANDARD.encode(v);
            String::serialize(&base64, s)
        } else {
            s.serialize_none()
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestDetails {
    uuid: Uuid,

    #[serde(with = "http_serde::method")]
    pub method: Method,

    #[serde(with = "http_serde::uri")]
    pub uri: Uri,

    #[serde(with = "http_serde::option::status_code")]
    pub status_code: Option<StatusCode>,

    #[serde(with = "http_serde::header_map")]
    pub request_headers: HeaderMap,

    #[serde(with = "http_serde::option::header_map")]
    pub response_headers: Option<HeaderMap>,

    #[serde(with = "base64_serde")]
    pub request_body: Option<Vec<u8>>,

    #[serde(with = "base64_serde")]
    pub response_body: Option<Vec<u8>>,

    #[serde(with = "rfc3339")]
    pub start_time: OffsetDateTime,

    #[serde(with = "rfc3339::option")]
    pub end_time: Option<OffsetDateTime>,
}

impl RequestDetails {
    pub fn new() -> RequestDetails {
        RequestDetails {
            uuid: Uuid::new_v4(),
            method: Method::default(),
            uri: Uri::default(),
            status_code: None,
            request_headers: HeaderMap::new(),
            response_headers: None,
            request_body: None,
            response_body: None,
            start_time: OffsetDateTime::now_utc(),
            end_time: None,
        }
    }
}
