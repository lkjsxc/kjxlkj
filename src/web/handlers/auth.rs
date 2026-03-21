use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::web::handlers::common::{
    clear_session_cookie, enforce_setup_completion, internal_error, session_cookie,
};
use crate::web::password::verify_password;
use crate::web::session::session_id_from_request;
use crate::web::state::WebState;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

impl LoginForm {
    fn normalized_username(&self) -> Option<String> {
        let value = self.username.trim();
        if value.is_empty() {
            None
        } else {
            Some(value.to_owned())
        }
    }

    fn normalized_password(&self) -> Option<&str> {
        let value = self.password.trim();
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }
}

pub async fn handle_get_login(state: web::Data<WebState>) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }

    HttpResponse::Ok().body("login")
}

pub async fn handle_post_login(
    state: web::Data<WebState>,
    form: web::Form<LoginForm>,
) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }

    let Some(username) = form.normalized_username() else {
        return HttpResponse::BadRequest().body("username is required");
    };
    let Some(password) = form.normalized_password() else {
        return HttpResponse::BadRequest().body("password is required");
    };

    let admin = match state.admin_store.find_admin_by_username(&username).await {
        Ok(Some(admin)) => admin,
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(error) => return internal_error(error),
    };

    let verified = match verify_password(password, &admin.password_hash) {
        Ok(value) => value,
        Err(error) => return internal_error(error),
    };
    if !verified {
        return HttpResponse::Unauthorized().finish();
    }

    let session = match state.session_store.create_session(admin.id).await {
        Ok(session) => session,
        Err(error) => return internal_error(error),
    };

    HttpResponse::Ok()
        .cookie(session_cookie(session.id))
        .body("login-ok")
}

pub async fn handle_post_logout(request: HttpRequest, state: web::Data<WebState>) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }

    if let Some(session_id) = session_id_from_request(&request) {
        let _ = state.session_store.delete_session(session_id).await;
    }

    HttpResponse::NoContent()
        .cookie(clear_session_cookie())
        .finish()
}
