//! Web layer

pub(crate) mod db;
pub(crate) mod embed_unfurl;
pub mod handlers;
mod history_summary;
mod live;
#[cfg(test)]
mod live_tests;
mod markdown;
mod markdown_cards;
mod markdown_external;
mod routes;
mod site;
#[cfg(test)]
mod site_tests;
mod templates;
mod view;
mod view_media;

pub use routes::run_server;
