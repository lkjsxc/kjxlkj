//! Public site identity and metadata helpers

use super::db::AppSettings;
use crate::config::Config;

const INDEX_FOLLOW: &str = "index,follow";
const NOINDEX_NOFOLLOW: &str = "noindex,nofollow";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SiteContext {
    pub site_name: String,
    pub site_description: String,
    pub public_base_url: Option<String>,
}

impl SiteContext {
    pub fn from_settings(config: &Config, settings: &AppSettings) -> Self {
        Self {
            site_name: settings.site_name.clone(),
            site_description: settings.site_description.clone(),
            public_base_url: config.public_base_url.clone(),
        }
    }

    pub fn absolute_url(&self, path: &str) -> Option<String> {
        self.public_base_url
            .as_ref()
            .map(|base| format!("{base}{path}"))
    }

    pub fn page_meta(
        &self,
        page_title: &str,
        description: impl Into<String>,
        indexable: bool,
        canonical_path: Option<&str>,
    ) -> PageMeta {
        let description = description.into().trim().to_string();
        let robots_content = if indexable && self.public_base_url.is_some() {
            INDEX_FOLLOW
        } else {
            NOINDEX_NOFOLLOW
        };
        PageMeta {
            page_title: page_title.to_string(),
            site_name: self.site_name.clone(),
            description: if description.is_empty() {
                self.site_description.clone()
            } else {
                description
            },
            robots_content,
            canonical_url: if robots_content == INDEX_FOLLOW {
                canonical_path.and_then(|path| self.absolute_url(path))
            } else {
                None
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageMeta {
    page_title: String,
    site_name: String,
    description: String,
    robots_content: &'static str,
    canonical_url: Option<String>,
}

impl PageMeta {
    pub fn full_title(&self) -> String {
        format!("{} | {}", self.page_title, self.site_name)
    }

    pub fn head_tags(&self) -> String {
        let canonical = self.canonical_url.as_ref().map_or_else(String::new, |url| {
            format!(r#"<link rel="canonical" href="{}">"#, escape_html_attr(url))
        });
        format!(
            r#"<meta name="description" content="{}">
<meta name="robots" content="{}">{}"#,
            escape_html_attr(&self.description),
            self.robots_content,
            canonical,
        )
    }
}

fn escape_html_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> AppSettings {
        AppSettings {
            site_name: "Launchpad".to_string(),
            site_description: "Search-friendly notes.".to_string(),
            ..AppSettings::default()
        }
    }

    fn config(public_base_url: Option<&str>) -> Config {
        Config {
            bind_host: "0.0.0.0".to_string(),
            bind_port: 8080,
            database_url: "postgres://example".to_string(),
            public_base_url: public_base_url.map(str::to_string),
        }
    }

    #[test]
    fn page_meta_uses_safe_noindex_without_public_origin() {
        let meta = SiteContext::from_settings(&config(None), &settings()).page_meta(
            "Home",
            "",
            true,
            Some("/"),
        );
        assert!(meta.head_tags().contains("noindex,nofollow"));
        assert!(!meta.head_tags().contains("rel=\"canonical\""));
    }

    #[test]
    fn page_meta_uses_page_then_site_titles() {
        let meta = SiteContext::from_settings(&config(Some("https://example.com")), &settings())
            .page_meta("Home", "", true, Some("/"));
        assert_eq!(meta.full_title(), "Home | Launchpad");
        assert!(meta.head_tags().contains("https://example.com/"));
    }
}
