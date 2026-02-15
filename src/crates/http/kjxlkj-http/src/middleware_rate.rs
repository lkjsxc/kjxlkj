//! Rate limiting middleware for auth endpoints per IMP-BACKLOG-SEC-02.
//! Simple in-memory sliding-window rate limiter for auth routes.
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Ready};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// Rate limit configuration.
pub struct RateLimiter {
    window: Duration,
    max_requests: usize,
}

impl RateLimiter {
    pub fn new(window_secs: u64, max_requests: usize) -> Self {
        Self {
            window: Duration::from_secs(window_secs),
            max_requests,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimiterMiddleware {
            service,
            state: Arc::new(Mutex::new(HashMap::new())),
            window: self.window,
            max_requests: self.max_requests,
        })
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    state: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    window: Duration,
    max_requests: usize,
}

type RateFut<B> = Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>>>>;

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = RateFut<B>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_owned();
        if !path.starts_with("/auth/") && !path.starts_with("/setup/") {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

        let key = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_owned();
        let now = Instant::now();
        let window = self.window;
        let max_req = self.max_requests;

        let mut state = self.state.lock().unwrap();
        let entries = state.entry(key).or_default();
        entries.retain(|t| now.duration_since(*t) < window);

        if entries.len() >= max_req {
            let resp = HttpResponse::TooManyRequests()
                .json(serde_json::json!({
                    "code": "RATE_LIMITED",
                    "message": "too many requests",
                    "request_id": uuid::Uuid::now_v7().to_string()
                }));
            let sr = req.into_response(resp).map_into_boxed_body();
            // We need B = BoxBody alignment; use transmute-safe cast
            // via the actix-provided map_into_body utility.
            return Box::pin(async move {
                // For now, the rate limiter only applies to auth routes.
                // The boxed body is compatible since we control the response.
                Err(actix_web::error::InternalError::from_response(
                    actix_web::error::ErrorTooManyRequests("rate limited"),
                    sr.into_parts().1,
                ).into())
            });
        }

        entries.push(now);
        drop(state);
        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}
