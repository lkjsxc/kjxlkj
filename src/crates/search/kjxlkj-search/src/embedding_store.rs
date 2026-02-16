/// In-memory embedding vector store and cosine similarity for semantic search.
///
/// Spec: /docs/spec/domain/search.md
/// - Deleted notes MUST be removed from vector candidate sets
/// - Embeddings MUST be regenerated whenever title or body changes
use super::embedding::EmbeddingVec;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// In-memory embedding vector store for semantic search.
/// Maps note_id → embedding vector for nearest-neighbor lookup.
pub struct EmbeddingStore {
    vectors: RwLock<HashMap<Uuid, EmbeddingVec>>,
}

impl EmbeddingStore {
    pub fn new() -> Self {
        Self {
            vectors: RwLock::new(HashMap::new()),
        }
    }

    /// Store or update embedding for a note.
    pub fn upsert(&self, note_id: Uuid, vec: EmbeddingVec) {
        self.vectors.write().unwrap().insert(note_id, vec);
    }

    /// Remove embedding for a deleted note.
    pub fn remove(&self, note_id: Uuid) {
        self.vectors.write().unwrap().remove(&note_id);
    }

    /// Find top-k nearest neighbors by cosine similarity.
    pub fn nearest(&self, query_vec: &[f64], k: usize) -> Vec<(Uuid, f64)> {
        let vecs = self.vectors.read().unwrap();
        let mut scored: Vec<(Uuid, f64)> = vecs
            .iter()
            .map(|(id, v)| (*id, cosine_similarity(query_vec, v)))
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);
        scored
    }

    /// Number of stored embeddings.
    pub fn len(&self) -> usize {
        self.vectors.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for EmbeddingStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Cosine similarity between two vectors.
/// Returns 0.0 for zero-length or mismatched vectors.
pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}
