//! HTML templates

mod auth;
mod dashboard;
mod dashboard_favorites;
mod history;
#[cfg(test)]
mod history_tests;
mod home;
mod index;
mod layout;
mod list_sections;
mod model;
mod note;
mod note_shell;
#[cfg(test)]
mod note_tests;
mod search;
#[cfg(test)]
mod search_tests;
mod sections;
mod settings_page;

pub use auth::{login_page, setup_page};
pub use dashboard::admin_page;
pub use history::{history_page, revision_page};
pub use home::home_page;
pub use layout::{not_found_page, render_time};
pub use model::{HistoryLink, IndexItem, IndexMetric, NavLink, NoteAnalytics, NoteChrome};
pub use note::note_page;
pub use search::search_page;
pub use settings_page::settings_page;
