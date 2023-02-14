pub mod prelude;
mod req;
use std::{fmt::Display, future::Future, pin::Pin};
#[cfg(feature = "routerify")]
mod routerify_exts;
pub use hyper;
use hyper::{body::HttpBody, Body};
pub use req::*;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;
#[cfg(feature = "routerify")]
pub use routerify_exts::*;

pub trait IntoResponse {
    type Body: HttpBody + Send + Sync + 'static;
    fn into_response(self) -> hyper::Response<Self::Body>;
}

impl<T> IntoResponse for T
where
    T: Display,
{
    type Body = Body;
    fn into_response(self) -> hyper::Response<Self::Body> {
        hyper::Response::builder()
            .status(hyper::StatusCode::OK)
            .header(hyper::header::CONTENT_TYPE, "text/plain")
            .body(self.to_string().into())
            .unwrap()
    }
}
