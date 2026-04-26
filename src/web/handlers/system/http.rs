use axum::http::{header, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub fn html(body: String) -> Response {
    html_status(StatusCode::OK, body)
}

pub fn html_status(status: StatusCode, body: String) -> Response {
    text_with_type(status, "text/html; charset=utf-8", body)
}

pub fn text_with_type(status: StatusCode, content_type: &str, body: String) -> Response {
    let mut response = (status, body).into_response();
    set_header(&mut response, header::CONTENT_TYPE, content_type);
    response
}

pub fn bytes_with_type(status: StatusCode, content_type: &str, body: Vec<u8>) -> Response {
    let mut response = (status, body).into_response();
    set_header(&mut response, header::CONTENT_TYPE, content_type);
    response
}

pub fn json_status<T: Serialize>(status: StatusCode, value: T) -> Response {
    (status, Json(value)).into_response()
}

pub fn empty(status: StatusCode) -> Response {
    status.into_response()
}

pub fn redirect(location: &str) -> Response {
    redirect_status(StatusCode::FOUND, location)
}

pub fn see_other(location: &str) -> Response {
    redirect_status(StatusCode::SEE_OTHER, location)
}

pub fn redirect_status(status: StatusCode, location: &str) -> Response {
    let mut response = status.into_response();
    set_header(&mut response, header::LOCATION, location);
    response
}

pub fn set_header(response: &mut Response, name: HeaderName, value: &str) {
    if let Ok(value) = HeaderValue::from_str(value) {
        response.headers_mut().insert(name, value);
    }
}

pub fn set_cookie(response: &mut Response, value: &str) {
    set_header(response, header::SET_COOKIE, value);
}

pub fn session_cookie(session_id: &str) -> String {
    format!("session_id={session_id}; Path=/; HttpOnly; SameSite=Strict")
}

pub fn clear_session_cookie() -> &'static str {
    "session_id=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict"
}
