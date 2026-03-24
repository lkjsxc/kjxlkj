use actix_web::{http::header, web, HttpResponse};

use super::state::AppState;

pub async fn enforce_setup_completion(state: &web::Data<AppState>) -> Result<(), HttpResponse> {
    match state.auth_store.has_admin_user().await {
        Ok(true) => Ok(()),
        Ok(false) => Err(redirect_to_setup()),
        Err(_) => Err(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn enforce_setup_pending(state: &web::Data<AppState>) -> Result<(), HttpResponse> {
    match state.auth_store.has_admin_user().await {
        Ok(false) => Ok(()),
        Ok(true) => Err(redirect_to_login()),
        Err(_) => Err(HttpResponse::InternalServerError().finish()),
    }
}

pub fn redirect_to_setup() -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, "/setup"))
        .finish()
}

pub fn redirect_to_login() -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, "/login"))
        .finish()
}
