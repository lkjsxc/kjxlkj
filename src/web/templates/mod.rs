//! HTML templates

mod auth;
mod card_frame;
mod dashboard;
mod dashboard_favorites;
mod history;
#[cfg(test)]
mod history_tests;
mod home;
mod index;
#[cfg(test)]
mod index_tests;
mod layout;
mod list_sections;
mod media_new;
mod model;
mod note;
mod note_shell;
#[cfg(test)]
mod note_shell_tests;
#[cfg(test)]
mod note_tests;
mod popular_sections;
#[cfg(test)]
mod popular_sections_tests;
mod resource_media;
mod resource_words;
mod search;
mod search_form;
#[cfg(test)]
mod search_tests;
mod sections;
mod settings_page;

pub use auth::{login_page, setup_page};
pub use dashboard::admin_page;
pub use history::{history_page, snapshot_page, HistoryPage};
pub use home::home_page;
pub use layout::{not_found_page, render_time};
pub use media_new::media_new_page;
pub use model::{HistoryLink, IndexItem, IndexMetric, NavLink, NoteAnalytics, NoteChrome};
pub use note::note_page;
pub use popular_sections::{admin_popular_section, home_popular_section};
pub use search::{search_page, SearchView};
pub use settings_page::settings_page;
