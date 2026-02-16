/// Structured tracing middleware per /docs/spec/technical/operations.md (IMP-OPS-01)
///
/// - Injects a request_id into every request
/// - Creates a tracing span with request_id, method, path
/// - Logs request completion with status and latency
/// - Includes user_id/workspace_id when available from headers
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use uuid::Uuid;

/// Header name for propagating request ID.
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Tracing middleware that wraps each request in a span with request_id.
/// Per /docs/spec/technical/operations.md: structured logs MUST include
/// request_id, and error logs MUST include stable error code.
pub async fn tracing_middleware(req: Request, next: Next) -> Response {
    let request_id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    // Extract optional workspace_id from query or header
    let workspace_id = req
        .headers()
        .get("x-workspace-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-")
        .to_string();

    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %method,
        path = %path,
        workspace_id = %workspace_id,
    );

    let _guard = span.enter();

    tracing::debug!("request started");

    // Drop the guard before await (cannot hold across await)
    drop(_guard);

    let response = next.run(req).await;

    let latency_ms = start.elapsed().as_millis();
    let status = response.status().as_u16();

    // Re-enter span for completion log
    let _guard2 = span.enter();

    if status >= 500 {
        tracing::error!(status, latency_ms, "request completed with server error");
    } else if status >= 400 {
        tracing::warn!(status, latency_ms, "request completed with client error");
    } else {
        tracing::info!(status, latency_ms, "request completed");
    }

    // Attach request_id to response headers for debugging
    let mut response = response;
    response.headers_mut().insert(
        REQUEST_ID_HEADER,
        request_id.parse().unwrap_or_else(|_| "unknown".parse().unwrap()),
    );
    response
}

/// Extracts the request ID from the response headers (for tests).
pub fn get_request_id<B>(response: &Response<B>) -> Option<String> {
    response
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_id_header_constant() {
        assert_eq!(REQUEST_ID_HEADER, "x-request-id");
    }

    #[test]
    fn get_request_id_from_response() {
        let mut resp = Response::new(());
        resp.headers_mut().insert(
            REQUEST_ID_HEADER,
            "test-123".parse().unwrap(),
        );
        assert_eq!(get_request_id(&resp), Some("test-123".to_string()));
    }

    #[test]
    fn get_request_id_missing() {
        let resp = Response::new(());
        assert_eq!(get_request_id(&resp), None);
    }
}
