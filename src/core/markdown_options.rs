#[derive(Clone, Copy, Debug, Default)]
pub struct MarkdownOptions<'a> {
    pub public_base_url: Option<&'a str>,
    pub google_maps_embed_api_key: Option<&'a str>,
}
