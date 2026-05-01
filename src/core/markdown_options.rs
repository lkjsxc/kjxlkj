use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct EmbedMetadata {
    pub provider: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub site_name: Option<String>,
    pub author_name: Option<String>,
    pub thumbnail_url: Option<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MarkdownOptions<'a> {
    pub public_base_url: Option<&'a str>,
    pub google_maps_embed_api_key: Option<&'a str>,
    pub external_embed_cache: Option<&'a HashMap<String, EmbedMetadata>>,
}
