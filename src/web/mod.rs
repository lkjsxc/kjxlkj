//! Web layer

mod db;
pub mod handlers;
mod routes;
mod templates;
mod view;

pub use routes::run_server;
