//! Sitemap and robots handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, SitemapRecord};
use crate::web::site::normalize_public_base_url;
use actix_web::{get, web, HttpResponse};

#[get("/robots.txt")]
pub async fn robots_txt(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let Some(public_base_url) = public_base_url(&pool).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(robots_body(&public_base_url)))
}

#[get("/sitemap.xml")]
pub async fn sitemap_xml(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let Some(public_base_url) = public_base_url(&pool).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };
    Ok(HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .body(sitemap_body(
            &public_base_url,
            &db::list_public_sitemap_records(&pool).await?,
        )))
}

async fn public_base_url(pool: &DbPool) -> Result<Option<String>, AppError> {
    Ok(normalize_public_base_url(
        &db::get_settings(pool).await?.public_base_url,
    ))
}

fn robots_body(public_base_url: &str) -> String {
    format!(
        "User-agent: *\nAllow: /\nDisallow: /search\nDisallow: /setup\nDisallow: /login\nDisallow: /admin\nDisallow: /resources\nDisallow: /_/\nDisallow: /healthz\nDisallow: /*/history\nSitemap: {public_base_url}/sitemap.xml\n"
    )
}

fn sitemap_body(public_base_url: &str, records: &[SitemapRecord]) -> String {
    let mut urls = vec![format!("<url><loc>{public_base_url}/</loc></url>")];
    urls.extend(records.iter().map(|record| {
        let path = format!("/{}", record.alias.as_deref().unwrap_or(&record.id));
        format!(
            "<url><loc>{public_base_url}{path}</loc><lastmod>{}</lastmod></url>",
            record.updated_at.to_rfc3339()
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
    fn sitemap_body_lists_home_and_current_note_urls() {
        let body = sitemap_body(
            "https://example.com",
            &[SitemapRecord {
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
