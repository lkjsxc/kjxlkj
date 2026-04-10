//! Web layer

pub(crate) mod db;
pub mod handlers;
mod routes;
mod site;
mod templates;
mod view;

pub use routes::run_server;
