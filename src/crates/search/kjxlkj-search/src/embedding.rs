use sqlx::{PgPool, Row};
use uuid::Uuid;
use tracing::debug;

/// Query embeddings for semantic search. Returns (note_id, title, score).
/// Uses an external embedding API (e.g., LMStudio or OpenRouter).
pub async fn query_embeddings(
    pool: &PgPool,
    query: &str,
    workspace_id: Uuid,
    limit: i64,
    base_url: &str,
    model: &str,
) -> Result<Vec<(Uuid, String, f64)>, String> {
    // Get query embedding
    let query_vec = get_embedding(base_url, model, query).await?;

    // Retrieve embeddings from workspace and rank locally.
    // In production, use pgvector or similar for ANN.
    let rows = sqlx::query(
        r#"SELECT ne.note_id, ns.title, ne.embedding, ne.dimensions
           FROM note_embeddings ne
           JOIN note_streams ns ON ne.note_id = ns.id
           WHERE ns.workspace_id = $1 AND NOT ns.is_deleted
           LIMIT 1000"#
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("db: {e}"))?;

    let mut scored: Vec<(Uuid, String, f64)> = Vec::new();
    for row in rows {
        let note_id: Uuid = row.get("note_id");
        let title: String = row.get("title");
        let embedding_bytes: Vec<u8> = row.get("embedding");
        let stored_vec = bytes_to_f32_vec(&embedding_bytes);
        let score = cosine_similarity(&query_vec, &stored_vec);
        scored.push((note_id, title, score));
    }

    scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(limit as usize);

    debug!("semantic search returned {} candidates", scored.len());
    Ok(scored)
}

/// Get embedding vector from external API.
pub async fn get_embedding(
    base_url: &str,
    model: &str,
    text: &str,
) -> Result<Vec<f32>, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/embeddings", base_url);

    let body = serde_json::json!({
        "model": model,
        "input": text,
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("embedding request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("embedding API {status}: {text}"));
    }

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("embedding response parse: {e}"))?;

    let embedding = json["data"][0]["embedding"]
        .as_array()
        .ok_or("missing embedding array")?
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect();

    Ok(embedding)
}

/// Store embedding vector for a note.
pub async fn store_embedding(
    pool: &PgPool,
    note_id: Uuid,
    embedding: &[f32],
    model: &str,
    dimensions: i32,
) -> Result<(), sqlx::Error> {
    let bytes = f32_vec_to_bytes(embedding);
    sqlx::query(
        "INSERT INTO note_embeddings (note_id, embedding, model, dimensions)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (note_id) DO UPDATE
         SET embedding = $2, model = $3, dimensions = $4, updated_at = now()"
    )
    .bind(note_id)
    .bind(&bytes)
    .bind(model)
    .bind(dimensions)
    .execute(pool)
    .await?;
    Ok(())
}

fn f32_vec_to_bytes(vec: &[f32]) -> Vec<u8> {
    vec.iter().flat_map(|f| f.to_le_bytes()).collect()
}

fn bytes_to_f32_vec(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f64 = a.iter().zip(b).map(|(x, y)| (*x as f64) * (*y as f64)).sum();
    let mag_a: f64 = a.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }
    dot / (mag_a * mag_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_identical() {
        let v = vec![1.0, 0.0, 1.0];
        let sim = cosine_similarity(&v, &v);
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-6);
    }

    #[test]
    fn test_f32_bytes_roundtrip() {
        let v = vec![1.0f32, -2.5, 3.14, 0.0];
        let bytes = f32_vec_to_bytes(&v);
        let back = bytes_to_f32_vec(&bytes);
        assert_eq!(v, back);
    }
}
