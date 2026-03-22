mod admin;
mod content;
mod content_index;
mod session;
mod settings;

use std::sync::Arc;

use crate::app_state::AppState;
use crate::web::state::{AdminStore, ContentStore, SessionStore, SettingsStore, WebState};

pub fn build_runtime_web_state(app_state: AppState) -> WebState {
    let admin_store: Arc<dyn AdminStore> = Arc::new(admin::RuntimeAdminStore {
        app_state: app_state.clone(),
    });
    let session_store: Arc<dyn SessionStore> = Arc::new(session::RuntimeSessionStore {
        app_state: app_state.clone(),
    });
    let content_store: Arc<dyn ContentStore> = Arc::new(content::RuntimeContentStore {
        app_state: app_state.clone(),
    });
    let settings_store: Arc<dyn SettingsStore> =
        Arc::new(settings::RuntimeSettingsStore { app_state });
    WebState {
        admin_store,
        session_store,
        content_store,
        settings_store,
    }
}
