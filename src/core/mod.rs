//! Core domain models and validation

pub mod live_settings;
mod markdown;
mod markdown_embed_blocks;
mod markdown_embeds;
mod markdown_links;
mod markdown_options;
#[cfg(test)]
mod markdown_tests;
pub mod nostr;
#[cfg(test)]
mod nostr_tests;
mod validation;
#[cfg(test)]
mod validation_tests;

pub use markdown::{
    external_embed_urls, render_markdown, render_markdown_with_options, render_markdown_with_origin,
};
pub use markdown_options::{EmbedMetadata, MarkdownOptions};
pub use validation::{
    derive_summary, derive_title, derive_title_with_fallback, extract_title, generate_id,
    looks_like_id, normalize_alias, validate_id, AliasError, IdError,
};
