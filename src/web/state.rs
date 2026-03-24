use crate::storage::FsStore;

use super::auth_store::AuthStore;

#[derive(Clone)]
pub struct AppState {
    pub admin_token: String,
    pub store: FsStore,
    pub auth_store: AuthStore,
    pub session_timeout_minutes: i32,
}
