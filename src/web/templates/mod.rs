//! HTML templates

mod auth;
mod card_frame;
mod dashboard;
mod dashboard_favorites;
mod history;
#[cfg(test)]
mod history_tests;
pub(crate) mod home;
mod index;
#[cfg(test)]
mod index_tests;
mod layout;
mod list_sections;
mod model;
mod popular_sections;
#[cfg(test)]
mod popular_sections_tests;
mod resource;
mod resource_media;
mod resource_shell;
#[cfg(test)]
mod resource_shell_tests;
#[cfg(test)]
mod resource_tests;
mod resource_words;
mod search;
mod search_form;
#[cfg(test)]
mod search_tests;
mod sections;
mod settings_home;
mod settings_page;
mod settings_panel;
mod settings_security;
mod style_bundle;

pub use auth::{login_page, password_reset_page, setup_page};
pub use dashboard::admin_page;
pub use history::{history_page, snapshot_page, HistoryPage};
pub use home::home_page;
pub use layout::{not_found_page, render_time};
pub use model::{HistoryLink, IndexItem, IndexMetric, NavLink, ResourceAnalytics, ResourceChrome};
pub use popular_sections::{admin_popular_section, home_popular_section};
pub use resource::resource_page;
pub use search::{search_page, SearchView};
pub use settings_page::settings_page;
