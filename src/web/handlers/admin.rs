use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use askama::Template;
use serde::Deserialize;

use crate::web::handlers::common::{html, is_admin};
use crate::web::state::AppState;
use crate::web::templates::AdminTemplate;

#[derive(Deserialize)]
pub struct SaveReq {
    slug: String,
    title: String,
    body: String,
    private: bool,
}

#[derive(Deserialize)]
pub struct RenameReq {
    from_slug: String,
    to_slug: String,
}

#[get("")]
pub async fn admin_home(state: web::Data<AppState>, session: Session) -> impl Responder {
    let admin = match is_admin(&session, state.get_ref()).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !admin {
        return HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish();
    }
    match state.content.list(true).await {
        Ok(articles) => html(AdminTemplate { articles }.render()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/open/{slug:.*}")]
pub async fn admin_open(
    path: web::Path<String>,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let admin = match is_admin(&session, state.get_ref()).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !admin {
        return HttpResponse::Unauthorized().finish();
    }
    match state.content.get(&path.into_inner()).await {
        Ok(Some(article)) => HttpResponse::Ok().json(article),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/save")]
pub async fn admin_save(
    req: web::Json<SaveReq>,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let admin = match is_admin(&session, state.get_ref()).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !admin {
        return HttpResponse::Unauthorized().finish();
    }
    match state
        .content
        .save(&req.slug, &req.title, &req.body, req.private)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "ok": true })),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/create")]
pub async fn admin_create(
    req: web::Json<SaveReq>,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let admin = match is_admin(&session, state.get_ref()).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !admin {
        return HttpResponse::Unauthorized().finish();
    }
    match state
        .content
        .save(&req.slug, &req.title, &req.body, req.private)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "ok": true })),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/toggle-private/{slug:.*}")]
pub async fn admin_toggle_private(
    path: web::Path<String>,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let admin = match is_admin(&session, state.get_ref()).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !admin {
        return HttpResponse::Unauthorized().finish();
    }
    let slug = path.into_inner();
    match state.content.get(&slug).await {
        Ok(Some(article)) => {
            match state
                .content
                .save(&slug, &article.title, &article.body, !article.private)
                .await
            {
                Ok(_) => HttpResponse::Ok().json(
                    serde_json::json!({ "ok": true, "slug": slug, "private": !article.private }),
                ),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/rename")]
pub async fn admin_rename(
    req: web::Json<RenameReq>,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let admin = match is_admin(&session, state.get_ref()).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !admin {
        return HttpResponse::Unauthorized().finish();
    }
    match state.content.rename(&req.from_slug, &req.to_slug).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "ok": true })),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/delete/{slug:.*}")]
pub async fn admin_delete(
    path: web::Path<String>,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let admin = match is_admin(&session, state.get_ref()).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !admin {
        return HttpResponse::Unauthorized().finish();
    }
    match state.content.delete(&path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "ok": true })),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
