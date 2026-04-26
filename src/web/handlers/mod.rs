//! Web handlers

#[path = "system/admin.rs"]
pub mod admin;
#[path = "system/assets.rs"]
pub mod assets;
#[path = "system/discoverability.rs"]
pub mod discoverability;
#[path = "resources/favorites.rs"]
pub mod favorites;
#[path = "system/health.rs"]
pub mod health;
#[path = "system/history.rs"]
pub mod history;
#[path = "resources/resource_history.rs"]
pub mod resource_history;
#[path = "system/home.rs"]
pub mod home;
#[path = "system/http.rs"]
mod http;
#[path = "live/live.rs"]
pub mod live;
#[path = "auth/login.rs"]
pub mod login;
#[path = "auth/logout.rs"]
pub mod logout;
#[path = "media/media.rs"]
pub mod media;
#[path = "media/media_attachments.rs"]
pub mod media_attachments;
#[path = "media/media_derivatives.rs"]
mod media_derivatives;
#[path = "media/media_input.rs"]
mod media_input;
#[path = "media/media_insert.rs"]
mod media_insert;
#[path = "media/media_support.rs"]
mod media_support;
#[path = "media/note_media_input.rs"]
mod note_media_input;
#[path = "auth/password_reset.rs"]
pub mod password_reset;
#[path = "system/popular_sections.rs"]
pub mod popular_sections;
#[path = "resources/preview.rs"]
pub mod preview;
#[path = "resources/resource.rs"]
pub mod resource;
#[path = "resources/resource_api.rs"]
pub mod resource_api;
#[path = "resources/resource_file.rs"]
pub mod resource_file;
#[path = "resources/resource_file_support.rs"]
mod resource_file_support;
#[path = "resources/resource_payload.rs"]
mod resource_payload;
#[path = "resources/resources.rs"]
pub mod resources;
#[path = "system/search.rs"]
pub mod search;
#[path = "auth/session.rs"]
pub mod session;
#[path = "settings/settings.rs"]
pub mod settings;
#[path = "settings/settings_input.rs"]
mod settings_input;
#[cfg(test)]
#[path = "settings/settings_input_tests.rs"]
mod settings_input_tests;
#[path = "auth/setup.rs"]
pub mod setup;
#[path = "settings/site_icon.rs"]
pub mod site_icon;
