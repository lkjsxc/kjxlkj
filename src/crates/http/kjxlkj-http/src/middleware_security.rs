//! Security response headers middleware per /docs/spec/security/transport.md.
//! Adds hardening headers to all responses.
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Middleware factory that adds security headers.
pub struct SecurityHeaders;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddleware { service })
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut resp = fut.await?;
            let headers = resp.headers_mut();
            headers.insert(
                actix_web::http::header::X_CONTENT_TYPE_OPTIONS,
                "nosniff".parse().unwrap(),
            );
            headers.insert(
                actix_web::http::header::X_FRAME_OPTIONS,
                "DENY".parse().unwrap(),
            );
            headers.insert(
                actix_web::http::header::CACHE_CONTROL,
                "no-store".parse().unwrap(),
            );
            // X-XSS-Protection (legacy but low cost)
            headers.insert(
                "x-xss-protection".parse().unwrap(),
                "1; mode=block".parse().unwrap(),
            );
            // Referrer-Policy
            headers.insert(
                "referrer-policy".parse().unwrap(),
                "strict-origin-when-cross-origin".parse().unwrap(),
            );
            Ok(resp)
        })
    }
}
