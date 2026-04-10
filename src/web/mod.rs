//! Web layer

pub(crate) mod db;
pub mod handlers;
mod history_summary;
mod routes;
mod site;
mod templates;
mod view;
mod view_media;

pub use routes::run_server;
