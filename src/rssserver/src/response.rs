use axum::{
    body::{Bytes, Full},
    http::{HeaderValue, HeaderName},
    response::{IntoResponse, Response},
};

const XML: &str = "text/xml";
const CONTENT_TYPE: &str = "content-type";

#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct Xml<T>(pub T);

impl<T> IntoResponse for Xml<T>
where
    T: Into<Full<Bytes>>,
{
    fn into_response(self) -> Response {
        
        let b = ([(HeaderName::from_static(CONTENT_TYPE), HeaderValue::from_static(XML))], self.0.into());
        b.into_response()
    }
}

impl<T> From<T> for Xml<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}
