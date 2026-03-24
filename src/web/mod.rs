//! Web layer

mod db;
pub mod handlers;
mod routes;
mod templates;

pub use routes::run_server;
