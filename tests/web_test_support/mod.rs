#![allow(dead_code)]

mod admin_store;
mod content_store;
mod session_store;
mod settings_store;

pub use admin_store::MockAdminStore;
pub use content_store::MockContentStore;
pub use session_store::MockSessionStore;

use std::sync::Arc;

use kjxlkj::web::state::WebState;

pub fn make_web_state() -> (WebState, MockAdminStore, MockSessionStore, MockContentStore) {
    let admin = MockAdminStore::default();
    let sessions = MockSessionStore::default();
    let content = MockContentStore::default();
    let settings = settings_store::MockSettingsStore::default();
    let state = WebState::new_for_tests(
        Arc::new(admin.clone()),
        Arc::new(sessions.clone()),
        Arc::new(content.clone()),
        Arc::new(settings),
    );
    (state, admin, sessions, content)
}
