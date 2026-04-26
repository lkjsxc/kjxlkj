//! HTML templates

#[path = "shared/auth.rs"]
mod auth;
#[path = "shared/card_frame.rs"]
mod card_frame;
#[path = "lists/dashboard.rs"]
mod dashboard;
#[path = "lists/dashboard_favorites.rs"]
mod dashboard_favorites;
#[path = "history/history.rs"]
mod history;
#[cfg(test)]
#[path = "history/history_tests.rs"]
mod history_tests;
#[path = "lists/home.rs"]
pub(crate) mod home;
#[path = "lists/index.rs"]
mod index;
#[cfg(test)]
#[path = "lists/index_tests.rs"]
mod index_tests;
#[path = "shared/layout.rs"]
mod layout;
#[path = "lists/list_sections.rs"]
mod list_sections;
#[path = "live/live.rs"]
mod live;
#[cfg(test)]
#[path = "live/live_tests.rs"]
mod live_tests;
#[path = "shared/model.rs"]
mod model;
#[path = "lists/popular_sections.rs"]
mod popular_sections;
#[cfg(test)]
#[path = "lists/popular_sections_tests.rs"]
mod popular_sections_tests;
#[path = "resources/resource.rs"]
mod resource;
#[path = "resources/resource_editor.rs"]
mod resource_editor;
#[path = "resources/resource_focus.rs"]
mod resource_focus;
#[path = "resources/resource_media.rs"]
mod resource_media;
#[path = "resources/resource_shell.rs"]
mod resource_shell;
#[cfg(test)]
#[path = "resources/resource_shell_tests.rs"]
mod resource_shell_tests;
#[cfg(test)]
#[path = "resources/resource_tests.rs"]
mod resource_tests;
#[path = "resources/resource_words.rs"]
mod resource_words;
#[path = "lists/search.rs"]
mod search;
#[path = "lists/search_form.rs"]
mod search_form;
#[cfg(test)]
#[path = "lists/search_tests.rs"]
mod search_tests;
#[path = "shared/sections.rs"]
mod sections;
#[path = "settings/settings_core.rs"]
mod settings_core;
#[path = "settings/settings_home.rs"]
mod settings_home;
#[path = "settings/settings_icon.rs"]
mod settings_icon;
#[path = "settings/settings_live.rs"]
mod settings_live;
#[path = "settings/settings_page.rs"]
mod settings_page;
#[path = "settings/settings_panel.rs"]
mod settings_panel;
#[path = "settings/settings_security.rs"]
mod settings_security;
#[path = "shared/style_bundle.rs"]
mod style_bundle;

pub use auth::{login_page, password_reset_page, setup_page};
pub use dashboard::admin_page;
pub use history::{history_page, snapshot_page, HistoryPage};
pub use home::home_page;
pub use layout::{not_found_page, render_time};
pub use live::live_page;
pub use model::{HistoryLink, IndexItem, IndexMetric, NavLink, ResourceAnalytics, ResourceChrome};
pub use popular_sections::{admin_popular_section, home_popular_section};
pub use resource::resource_page;
pub use search::{search_page, SearchView};
pub use settings_page::settings_page;
