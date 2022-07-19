use hyper::body::HttpBody;
use tap::Pipe;

use serde::de::DeserializeOwned;

use crate::BoxFuture;

pub trait ReqExt<'a>
{
	#[cfg(feature = "urlencoded")]
	fn body_urlencoded<T: DeserializeOwned>(&'a mut self) -> BoxFuture<'a, crate::Result<T>>;
	#[cfg(feature = "json")]
	fn body_json<T: DeserializeOwned>(&'a mut self) -> BoxFuture<'a, crate::Result<T>>;
	fn body_raw_bytes(&'a mut self) -> BoxFuture<'a, crate::Result<Vec<u8>>>;
	fn body_raw_bytes_with_max_size(
		&'a mut self,
		size: u64,
	) -> BoxFuture<'a, crate::Result<Vec<u8>>>;
}
impl<'a, B> ReqExt<'a> for hyper::Request<B>
where
	B: hyper::body::HttpBody + std::marker::Unpin,
	<B as hyper::body::HttpBody>::Error: std::error::Error + Send + Sync + 'static,
{
	#[cfg(feature = "urlencoded")]
	fn body_urlencoded<T: DeserializeOwned>(&'a mut self) -> BoxFuture<'a, crate::Result<T>>
//{{{
	{
		Box::pin(async move {
			serde_urlencoded::from_bytes(&self.body_raw_bytes().await?).map_err(Into::into)
		})
	} //}}}
	#[cfg(feature = "json")]
	fn body_json<T: DeserializeOwned>(&'a mut self) -> BoxFuture<'a, crate::Result<T>>
//{{{
	{
		Box::pin(async move {
			serde_json::from_slice(&self.body_raw_bytes().await?).map_err(Into::into)
		})
	} //}}}
	fn body_raw_bytes(&'a mut self) -> BoxFuture<'a, crate::Result<Vec<u8>>>
//{{{
	{
		self.body_raw_bytes_with_max_size(1024)
	} //}}}
	fn body_raw_bytes_with_max_size(
		&'a mut self,
		max_size: u64,
	) -> BoxFuture<'a, crate::Result<Vec<u8>>>
//{{{
	{
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
	} //}}}
}
