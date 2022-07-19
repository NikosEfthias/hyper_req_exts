mod req;
use std::{error::Error, future::Future, pin::Pin};

pub use req::*;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
