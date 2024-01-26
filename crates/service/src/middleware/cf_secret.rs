use axum::{http::Request, middleware::Next, response::Response};
#[cfg(debug_assertions)]
#[inline(always)]
pub async fn check_secret<B>(request: Request<B>, next: Next<B>) -> Response {
    next.run(request).await
}
#[cfg(not(debug_assertions))]
pub async fn check_secret<B>(request: Request<B>, next: Next<B>) -> Response {
    use axum::{http::StatusCode, response::IntoResponse};
    use std::env;
    let cf_header = env::var("CF_HEADER");
    let cf_secret = env::var("CF_SECRET");
    match (cf_header, cf_secret) {
        (Ok(header_name), Ok(secret)) => {
            let value = request
                .headers()
                .get(&header_name)
                .and_then(|v| v.to_str().ok());

            if value == Some(secret.as_str()) {
                return next.run(request).await;
            }
        }
        _ => {}
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}
