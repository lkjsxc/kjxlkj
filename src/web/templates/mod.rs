//! HTML templates

mod auth;
mod dashboard;
mod history;
mod home;
mod index;
mod layout;
mod model;
mod note;
mod note_shell;

pub use auth::{login_page, setup_page};
pub use dashboard::admin_page;
pub use history::{history_page, revision_page};
pub use home::home_page;
pub use layout::{not_found_page, render_time};
pub use model::{HistoryLink, IndexItem, NavLink, NoteChrome};
pub use note::note_page;
