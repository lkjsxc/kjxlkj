use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use kjxlkj::core::auth::AdminUser;
use kjxlkj::error::AppError;
use kjxlkj::web::state::AdminStore;

#[derive(Clone, Default)]
pub struct MockAdminStore {
    inner: Arc<Mutex<Option<AdminUser>>>,
}

impl MockAdminStore {
    pub fn set_admin(&self, password_hash: &str) {
        let mut admin = self.inner.lock().expect("admin lock poisoned");
        *admin = Some(AdminUser {
            id: 1,
            password_hash: password_hash.to_owned(),
        });
    }

    pub fn admin(&self) -> Option<AdminUser> {
        self.inner.lock().expect("admin lock poisoned").clone()
    }
}

#[async_trait]
impl AdminStore for MockAdminStore {
    async fn has_admin_user(&self) -> Result<bool, AppError> {
        Ok(self.inner.lock().expect("admin lock poisoned").is_some())
    }

    async fn load_admin(&self) -> Result<Option<AdminUser>, AppError> {
        let admin = self.inner.lock().expect("admin lock poisoned");
        Ok(admin.clone())
    }

    async fn create_admin(&self, password_hash: &str) -> Result<AdminUser, AppError> {
        let mut admin = self.inner.lock().expect("admin lock poisoned");
        let next = AdminUser {
            id: 1,
            password_hash: password_hash.to_owned(),
        };
        *admin = Some(next.clone());
        Ok(next)
    }
}
