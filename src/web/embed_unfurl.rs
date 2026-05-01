use crate::error::AppError;
use crate::web::db::{self, DbPool, ExternalEmbed};
use futures_util::StreamExt;
use reqwest::{redirect::Policy, Client};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::lookup_host;
use url::Url;

mod metadata;

const MAX_BYTES: usize = 262_144;
const USER_AGENT: &str = "kjxlkj-embed-unfurler/1.0";

pub async fn refresh_body_embeds(
    pool: &DbPool,
    body: &str,
    public_base_url: Option<&str>,
) -> Result<(), AppError> {
    let urls = crate::core::external_embed_urls(body, public_base_url);
    let stale = db::stale_external_embed_urls(pool, &urls).await?;
    for url in stale
        .into_iter()
        .filter(|url| metadata::should_unfurl(url))
        .take(8)
    {
        let provider = metadata::provider_label(&url);
        match fetch_embed(&url).await {
            Ok(embed) => db::upsert_external_embed(pool, &embed).await?,
            Err(error) => db::upsert_external_embed_error(pool, &url, &provider, &error).await?,
        }
    }
    Ok(())
}

async fn fetch_embed(url: &str) -> Result<ExternalEmbed, String> {
    let parsed = validate_url(url)?;
    let host = parsed.host_str().ok_or("missing host")?.to_string();
    let addrs = resolve_public_addrs(&parsed).await?;
    let client = Client::builder()
        .redirect(Policy::none())
        .timeout(Duration::from_secs(3))
        .user_agent(USER_AGENT)
        .resolve_to_addrs(&host, &addrs)
        .build()
        .map_err(|error| error.to_string())?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| error.to_string())?;
    if !response.status().is_success() {
        return Err(format!("status {}", response.status()));
    }
    let html = capped_body(response).await?;
    Ok(metadata::parse_html(
        url,
        &metadata::provider_label(url),
        &html,
    ))
}

fn validate_url(url: &str) -> Result<Url, String> {
    let parsed = Url::parse(url).map_err(|_| "invalid url")?;
    match (parsed.scheme(), parsed.port_or_known_default()) {
        ("http", Some(80)) | ("https", Some(443)) => Ok(parsed),
        ("http" | "https", _) => Err("nonstandard port".to_string()),
        _ => Err("unsupported scheme".to_string()),
    }
}

async fn resolve_public_addrs(url: &Url) -> Result<Vec<SocketAddr>, String> {
    let host = url.host_str().ok_or("missing host")?;
    let port = url.port_or_known_default().ok_or("missing port")?;
    let addrs = lookup_host((host, port))
        .await
        .map_err(|_| "dns lookup failed")?
        .filter(|addr| public_ip(addr.ip()))
        .collect::<Vec<_>>();
    (!addrs.is_empty())
        .then_some(addrs)
        .ok_or_else(|| "no public addresses".to_string())
}

async fn capped_body(response: reqwest::Response) -> Result<String, String> {
    let mut body = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| error.to_string())?;
        if body.len() + chunk.len() > MAX_BYTES {
            return Err("response too large".to_string());
        }
        body.extend_from_slice(&chunk);
    }
    String::from_utf8(body).map_err(|_| "response is not utf-8".to_string())
}

fn public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            !(ip.is_private()
                || ip.is_loopback()
                || ip.is_link_local()
                || ip.is_broadcast()
                || ip.is_unspecified()
                || ip.is_multicast())
        }
        IpAddr::V6(ip) => {
            !(ip.is_loopback() || ip.is_unspecified() || ip.is_unique_local() || ip.is_multicast())
        }
    }
}
