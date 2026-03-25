//! HTML templates

mod auth;
mod layout;
mod notes;

pub use auth::{login_page, setup_page};
pub use layout::not_found_page;
pub use notes::{admin_page, home_page, note_page};
