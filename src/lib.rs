mod req;
use std::{future::Future, pin::Pin};

pub use req::*;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "routerify")]
pub async fn start_server<B, E>(addr: std::net::SocketAddr, router: routerify::Router<B, E>)
where
	E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
	B: hyper::body::HttpBody + Send + Sync + 'static,
	B::Error: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
	B::Data: Send + Sync + 'static,
{
	let service = routerify::RouterService::new(router).expect("failed to start server");
	hyper::Server::bind(&addr)
		.serve(service)
		.await
		.expect("server failed");
}
