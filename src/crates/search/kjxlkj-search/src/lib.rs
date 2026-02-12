use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub note_id: Uuid,
    pub rank: f32,
}
