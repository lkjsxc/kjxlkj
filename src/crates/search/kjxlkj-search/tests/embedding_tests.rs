/// Tests for embedding provider trait and implementations.
///
/// Spec: /docs/spec/domain/search.md
#[cfg(test)]
mod tests {
    use kjxlkj_search::embedding::*;
    use kjxlkj_search::embedding_store::*;

    #[test]
    fn stub_embed_returns_correct_dimensions() {
        let provider = StubEmbeddingProvider::new(768);
        let vec = provider.embed("hello world").unwrap();
        assert_eq!(vec.len(), 768);
        assert_eq!(provider.dimensions(), 768);
        assert!(provider.is_available());
        assert_eq!(provider.provider_name(), "stub");
    }

    #[test]
    fn stub_embed_is_deterministic() {
        let provider = StubEmbeddingProvider::new(128);
        let a = provider.embed("test text").unwrap();
        let b = provider.embed("test text").unwrap();
        assert_eq!(a, b, "same input must produce same embedding");
    }

    #[test]
    fn stub_embed_different_text_different_vector() {
        let provider = StubEmbeddingProvider::new(128);
        let a = provider.embed("alpha").unwrap();
        let b = provider.embed("beta gamma delta").unwrap();
        assert_ne!(a, b, "different text should produce different embedding");
    }

    #[test]
    fn stub_batch_embed() {
        let provider = StubEmbeddingProvider::new(64);
        let texts = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let vecs = provider.embed_batch(&texts).unwrap();
        assert_eq!(vecs.len(), 3);
        for v in &vecs {
            assert_eq!(v.len(), 64);
        }
    }

    #[test]
    fn null_provider_always_fails() {
        let provider = NullEmbeddingProvider;
        assert!(provider.embed("anything").is_err());
        assert!(provider.embed_batch(&["test".to_string()]).is_err());
        assert!(!provider.is_available());
        assert_eq!(provider.provider_name(), "null");
        assert_eq!(provider.dimensions(), 0);
    }

    #[test]
    fn http_provider_stub_returns_error() {
        let provider = HttpEmbeddingProvider::new(
            "http://localhost:1234/v1".into(),
            "test-model".into(),
            768,
        );
        assert!(provider.embed("text").is_err());
        assert!(!provider.is_available());
        assert_eq!(provider.provider_name(), "http");
        assert_eq!(provider.dimensions(), 768);
    }

    #[test]
    fn cosine_similarity_identical_vectors() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-10);
    }

    #[test]
    fn cosine_similarity_orthogonal_vectors() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-10);
    }

    #[test]
    fn cosine_similarity_opposite_vectors() {
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim + 1.0).abs() < 1e-10);
    }

    #[test]
    fn cosine_similarity_mismatched_length() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn cosine_similarity_zero_vector() {
        let a = vec![0.0, 0.0];
        let b = vec![1.0, 2.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn embedding_store_upsert_and_nearest() {
        let store = EmbeddingStore::new();
        let id1 = uuid::Uuid::new_v4();
        let id2 = uuid::Uuid::new_v4();
        let id3 = uuid::Uuid::new_v4();
        store.upsert(id1, vec![1.0, 0.0, 0.0]);
        store.upsert(id2, vec![0.9, 0.1, 0.0]);
        store.upsert(id3, vec![0.0, 0.0, 1.0]);
        assert_eq!(store.len(), 3);

        let query = vec![1.0, 0.0, 0.0];
        let nearest = store.nearest(&query, 2);
        assert_eq!(nearest.len(), 2);
        // id1 should be closest (identical), id2 second
        assert_eq!(nearest[0].0, id1);
        assert!((nearest[0].1 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn embedding_store_remove() {
        let store = EmbeddingStore::new();
        let id = uuid::Uuid::new_v4();
        store.upsert(id, vec![1.0, 0.0]);
        assert_eq!(store.len(), 1);
        store.remove(id);
        assert!(store.is_empty());
    }

    #[test]
    fn factory_creates_stub_when_test() {
        let provider = create_embedding_provider(true, "stub", "", "", 64);
        assert!(provider.is_available());
        assert_eq!(provider.provider_name(), "stub");
    }

    #[test]
    fn factory_creates_null_when_disabled() {
        let provider = create_embedding_provider(false, "lmstudio", "", "", 768);
        assert!(!provider.is_available());
        assert_eq!(provider.provider_name(), "null");
    }

    #[test]
    fn factory_creates_http_for_lmstudio() {
        let p = create_embedding_provider(true, "lmstudio", "http://localhost:1234/v1", "nomic", 768);
        assert_eq!(p.provider_name(), "http");
        assert_eq!(p.dimensions(), 768);
    }

    #[test]
    fn stub_embed_produces_unit_vectors() {
        let provider = StubEmbeddingProvider::new(32);
        let vec = provider.embed("test data for normalization").unwrap();
        let norm: f64 = vec.iter().map(|v| v * v).sum::<f64>().sqrt();
        assert!(
            (norm - 1.0).abs() < 1e-10,
            "stub embeddings should be unit vectors, got norm={norm}"
        );
    }

    #[test]
    fn semantic_search_integration_with_store() {
        // End-to-end: embed notes, query, get ranked results
        let provider = StubEmbeddingProvider::new(64);
        let store = EmbeddingStore::new();

        let id_meeting = uuid::Uuid::new_v4();
        let id_recipe = uuid::Uuid::new_v4();
        let id_meeting2 = uuid::Uuid::new_v4();

        // Index notes
        let e1 = provider.embed("weekly meeting standup").unwrap();
        store.upsert(id_meeting, e1);
        let e2 = provider.embed("chocolate cake recipe baking").unwrap();
        store.upsert(id_recipe, e2);
        let e3 = provider.embed("meeting agenda project review").unwrap();
        store.upsert(id_meeting2, e3);

        // Query for "meeting"
        let query_vec = provider.embed("meeting").unwrap();
        let results = store.nearest(&query_vec, 3);
        assert_eq!(results.len(), 3);
        // Meeting-related notes should rank higher than recipe
        let meeting_ids: Vec<uuid::Uuid> = vec![id_meeting, id_meeting2];
        let top_id = results[0].0;
        assert!(
            meeting_ids.contains(&top_id),
            "top result should be a meeting note"
        );
    }
}
