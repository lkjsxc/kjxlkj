/// kjxlkj-search: Hybrid lexical + semantic search services.
///
/// Canonical spec: /docs/spec/domain/search.md
pub mod service;
pub mod backlinks;
pub mod embedding;
pub mod embedding_store;

pub use service::SearchService;
pub use embedding::{
    EmbeddingProvider, EmbeddingVec,
    StubEmbeddingProvider, NullEmbeddingProvider, HttpEmbeddingProvider,
    create_embedding_provider,
};
pub use embedding_store::{EmbeddingStore, cosine_similarity};
