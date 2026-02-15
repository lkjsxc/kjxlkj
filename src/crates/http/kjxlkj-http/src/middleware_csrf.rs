//! CSRF validation middleware per /docs/spec/security/csrf.md.
//! State-changing HTTP operations MUST enforce CSRF validation.
//! GET requests and WebSocket handshake are exempt.
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use std::task::{Context, Poll};

/// CSRF header name.
pub const CSRF_HEADER: &str = "x-csrf-token";

/// Middleware factory that enforces CSRF validation on mutating requests.
pub struct CsrfEnforcer;

impl<S, B> Transform<S, ServiceRequest> for CsrfEnforcer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CsrfEnforcerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CsrfEnforcerMiddleware { service })
    }
}

pub struct CsrfEnforcerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CsrfEnforcerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Either<
        futures::future::Map<S::Future, fn(Result<ServiceResponse<B>, Error>) -> Result<ServiceResponse<EitherBody<B>>, Error>>,
        Ready<Result<Self::Response, Self::Error>>,
    >;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        use futures::FutureExt;
        let method = req.method().clone();
        // GET, HEAD, OPTIONS are exempt per /docs/spec/security/csrf.md
        if method == actix_web::http::Method::GET
            || method == actix_web::http::Method::HEAD
            || method == actix_web::http::Method::OPTIONS
        {
            return Either::Left(self.service.call(req).map(wrap_left_body as _));
        }
        // Setup/login/register are exempt (no session yet)
        let path = req.path().to_owned();
        if path.starts_with("/setup/")
            || path.starts_with("/auth/")
            || path.starts_with("/api/healthz")
            || path.starts_with("/api/readyz")
        {
            return Either::Left(self.service.call(req).map(wrap_left_body as _));
        }
        // Validate CSRF token header is present for mutating requests
        let has_token = req.headers().get(CSRF_HEADER).is_some();
        if !has_token {
            let resp = HttpResponse::Forbidden()
                .json(serde_json::json!({
                    "code": "CSRF_INVALID",
                    "message": "missing CSRF token header",
                    "request_id": uuid::Uuid::now_v7().to_string()
                }));
            return Either::Right(ok(
                req.into_response(resp).map_into_right_body()
            ));
        }
        Either::Left(self.service.call(req).map(wrap_left_body as _))
    }
}

fn wrap_left_body<B>(
    res: Result<ServiceResponse<B>, Error>,
) -> Result<ServiceResponse<EitherBody<B>>, Error> {
    res.map(|resp| resp.map_into_left_body())
}
