use hyper::{header::HeaderValue, Body, Response};
pub use routerify;
use tap::Tap;
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
pub async fn cors_allow_all_with_request_info<E>(
    mut resp: Response<Body>,
    req_info: routerify::RequestInfo,
) -> Result<Response<Body>, E> {
    let origin = req_info
        .headers()
        .get("origin")
        .or_else(|| req_info.headers().get("Origin"))
        .cloned()
        .unwrap_or_else(|| HeaderValue::from_static("*"));
    resp.headers_mut().tap_mut(|it| {
        it.insert("Access-Control-Allow-Origin", origin.clone());
        it.insert(
            "Access-Control-Allow-Credentials",
            HeaderValue::from_static("true"),
        );
        it.insert(
            "Access-Control-Allow-Headers",
            HeaderValue::from_static("*"),
        );
        it.insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("*"),
        );
    });
    Ok(resp)
}
pub async fn cors_allow_all<E>(mut resp: Response<Body>) -> Result<Response<Body>, E> {
    resp.headers_mut().tap_mut(|it| {
        it.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        it.insert(
            "Access-Control-Allow-Headers",
            HeaderValue::from_static("*"),
        );
        it.insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("*"),
        );
    });
    Ok(resp)
}
