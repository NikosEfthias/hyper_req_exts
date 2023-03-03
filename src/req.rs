use base64::{prelude::BASE64_STANDARD as b64, Engine};
use hyper::{body::HttpBody, Body};
use tap::Pipe;

use crate::BoxFuture;

pub trait ReqExt<'a> {
    #[cfg(feature = "urlencoded")]
    fn body_urlencoded<T: serde::de::DeserializeOwned>(
        &'a mut self,
    ) -> BoxFuture<'a, crate::Result<T>>;
    #[cfg(feature = "json")]
    fn body_json<T: serde::de::DeserializeOwned>(&'a mut self) -> BoxFuture<'a, crate::Result<T>>;
    fn body_raw_bytes(&'a mut self) -> BoxFuture<'a, crate::Result<Vec<u8>>>;
    fn body_raw_bytes_with_max_size(
        &'a mut self,
        size: u64,
    ) -> BoxFuture<'a, crate::Result<Vec<u8>>>;
    fn basic_auth(&'a self) -> Option<(String, String)>;
    fn bearer_auth(&'a self) -> Option<String>;
    fn body_text(&'a mut self) -> BoxFuture<'a, crate::Result<String>>;
}
impl<'a> ReqExt<'a> for hyper::Request<Body> {
    #[cfg(feature = "urlencoded")]
    fn body_urlencoded<T: serde::de::DeserializeOwned>(
        &'a mut self,
    ) -> BoxFuture<'a, crate::Result<T>> {
        Box::pin(async move {
            serde_urlencoded::from_bytes(&self.body_raw_bytes().await?).map_err(Into::into)
        })
    }
    #[cfg(feature = "json")]
    fn body_json<T: serde::de::DeserializeOwned>(&'a mut self) -> BoxFuture<'a, crate::Result<T>> {
        Box::pin(async move {
            serde_json::from_slice(&self.body_raw_bytes().await?).map_err(Into::into)
        })
    }
    fn body_raw_bytes(&'a mut self) -> BoxFuture<'a, crate::Result<Vec<u8>>> {
        self.body_raw_bytes_with_max_size(1024)
    }
    fn body_raw_bytes_with_max_size(
        &'a mut self,
        max_size: u64,
    ) -> BoxFuture<'a, crate::Result<Vec<u8>>> {
        Box::pin(async move {
            let size = self.size_hint();
            match size.upper() {
                Some(upper) if upper > max_size => {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "body too large",
                    ))?;
                }
                None => {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "body too large",
                    ))?;
                }
                _ => {}
            }
            hyper::body::to_bytes(self.body_mut())
                .await?
                .to_vec()
                .pipe(Ok)
        })
    }

    fn basic_auth(&'a self) -> Option<(String, String)> {
        self.headers().get("Authorization").and_then(|h| {
            let s = h.to_str().ok()?;
            b64.decode(s.strip_prefix("Basic ")?)
                .ok()?
                .as_slice()
                .pipe(String::from_utf8_lossy)
                .split(':')
                .pipe(|mut s| {
                    let uname = s.next()?;
                    let pwd = s.next()?;
                    Some((uname.to_string(), pwd.to_string()))
                })
        })
    }
    fn bearer_auth(&'a self) -> Option<String> {
        self.headers().get("Authorization").and_then(|h| {
            let s = h.to_str().ok()?;
            s.strip_prefix("Bearer ")?;
            Some(s.to_string())
        })
    }
    fn body_text(&'a mut self) -> BoxFuture<'a, crate::Result<String>> {
        Box::pin(async move {
            let body = self.body_raw_bytes().await?;
            Ok(String::from_utf8_lossy(&body).to_string())
        })
    }
}
