//! Repository implementations

mod note;
mod workspace;
mod idempotency;

pub use note::*;
pub use workspace::*;
pub use idempotency::*;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Generic in-memory store wrapper
pub type InMemStore<T> = Arc<RwLock<HashMap<Uuid, T>>>;
