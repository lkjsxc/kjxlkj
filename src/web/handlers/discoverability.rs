//! Sitemap and robots handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, SitemapResource};
use crate::web::handlers::http;
use crate::web::routes::AppState;
use crate::web::site::normalize_public_base_url;
use axum::extract::{Query, State};
use axum::http::{header, StatusCode};
use axum::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

pub async fn robots_txt(State(state): State<AppState>) -> Result<Response, AppError> {
    let Some(public_base_url) = public_base_url(&state.pool).await? else {
        return Ok(http::empty(StatusCode::NOT_FOUND));
    };
    Ok(http::text_with_type(
        StatusCode::OK,
        "text/plain; charset=utf-8",
        robots_body(&public_base_url),
    ))
}

pub async fn sitemap_xml(State(state): State<AppState>) -> Result<Response, AppError> {
    let Some(public_base_url) = public_base_url(&state.pool).await? else {
        return Ok(http::empty(StatusCode::NOT_FOUND));
    };
    Ok(http::text_with_type(
        StatusCode::OK,
        "application/xml; charset=utf-8",
        sitemap_body(
            &public_base_url,
            &db::list_public_sitemap_resources(&state.pool).await?,
        ),
    ))
}

#[derive(Debug, Deserialize)]
pub struct NostrQuery {
    name: Option<String>,
}

#[derive(Debug, Serialize)]
struct NostrResponse {
    names: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    relays: BTreeMap<String, Vec<String>>,
}

pub async fn nostr_json(
    State(state): State<AppState>,
    Query(query): Query<NostrQuery>,
) -> Result<Response, AppError> {
    let settings = db::get_settings(&state.pool).await?;
    let names = selected_nostr_names(&settings.nostr_names, query.name.as_deref());
    let relays = nostr_relays(&settings.nostr_relays, &names);
    let mut response = http::json_status(StatusCode::OK, NostrResponse { names, relays });
    http::set_header(&mut response, header::ACCESS_CONTROL_ALLOW_ORIGIN, "*");
    Ok(response)
}

async fn public_base_url(pool: &DbPool) -> Result<Option<String>, AppError> {
    Ok(normalize_public_base_url(
        &db::get_settings(pool).await?.public_base_url,
    ))
}

fn robots_body(public_base_url: &str) -> String {
    format!(
        "User-agent: *\nAllow: /\nDisallow: /search\nDisallow: /live\nDisallow: /setup\nDisallow: /login\nDisallow: /admin\nDisallow: /resources\nDisallow: /_/\nDisallow: /.well-known/\nDisallow: /healthz\nDisallow: /*/history\nSitemap: {public_base_url}/sitemap.xml\n"
    )
}

fn selected_nostr_names(value: &Value, name: Option<&str>) -> BTreeMap<String, String> {
    let Some(object) = value.as_object() else {
        return BTreeMap::new();
    };
    match name.and_then(|name| crate::core::nostr::normalize_name(name).ok()) {
        Some(name) => object
            .get(&name)
            .and_then(Value::as_str)
            .map(|key| BTreeMap::from([(name, key.to_string())]))
            .unwrap_or_default(),
        None => object_to_names(object),
    }
}

fn object_to_names(object: &Map<String, Value>) -> BTreeMap<String, String> {
    object
        .iter()
        .filter_map(|(name, key)| key.as_str().map(|key| (name.clone(), key.to_string())))
        .collect()
}

fn nostr_relays(value: &Value, names: &BTreeMap<String, String>) -> BTreeMap<String, Vec<String>> {
    let relay_list = value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if relay_list.is_empty() {
        return BTreeMap::new();
    }
    names
        .values()
        .map(|key| (key.clone(), relay_list.clone()))
        .collect()
}

fn sitemap_body(public_base_url: &str, resources: &[SitemapResource]) -> String {
    let mut urls = vec![format!("<url><loc>{public_base_url}/</loc></url>")];
    urls.extend(resources.iter().map(|resource| {
        let path = format!("/{}", resource.alias.as_deref().unwrap_or(&resource.id));
        format!(
            "<url><loc>{public_base_url}{path}</loc><lastmod>{}</lastmod></url>",
            resource.updated_at.to_rfc3339()
        )
    }));
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{}</urlset>"#,
        urls.join("")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn robots_body_advertises_sitemap_and_disallows_search() {
        let body = robots_body("https://example.com");
        assert!(body.contains("Disallow: /search"));
        assert!(body.contains("Disallow: /resources"));
        assert!(!body.contains("Disallow: /records"));
        assert!(body.contains("Sitemap: https://example.com/sitemap.xml"));
    }

    #[test]
    fn sitemap_body_lists_home_and_current_resource_urls() {
        let body = sitemap_body(
            "https://example.com",
            &[SitemapResource {
                id: "abcdefghijklmnopqrstuvwx26".to_string(),
                alias: Some("release-notes".to_string()),
                updated_at: Utc::now(),
            }],
        );
        assert!(body.contains("<loc>https://example.com/</loc>"));
        assert!(body.contains("<loc>https://example.com/release-notes</loc>"));
        assert!(body.contains("<lastmod>"));
    }
}
