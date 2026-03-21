use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::web::handlers::common::{enforce_setup_pending, internal_error};
use crate::web::password::hash_password;
use crate::web::state::WebState;

#[derive(Debug, Deserialize)]
pub struct SetupForm {
    username: String,
    password: String,
}

impl SetupForm {
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

pub async fn handle_get_setup(state: web::Data<WebState>) -> HttpResponse {
    if let Err(response) = enforce_setup_pending(&state).await {
        return response;
    }
    HttpResponse::Ok().body("setup")
}

pub async fn handle_post_setup(
    state: web::Data<WebState>,
    form: web::Form<SetupForm>,
) -> HttpResponse {
    if let Err(response) = enforce_setup_pending(&state).await {
        return response;
    }

    let Some(username) = form.normalized_username() else {
        return HttpResponse::BadRequest().body("username is required");
    };
    let Some(password) = form.normalized_password() else {
        return HttpResponse::BadRequest().body("password is required");
    };

    let password_hash = match hash_password(password) {
        Ok(value) => value,
        Err(error) => return internal_error(error),
    };

    match state
        .admin_store
        .create_admin(&username, &password_hash)
        .await
    {
        Ok(admin) => HttpResponse::Created().body(format!("setup-complete:{}", admin.username)),
        Err(error) => internal_error(error),
    }
}
