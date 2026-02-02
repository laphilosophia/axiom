use tracing::{debug, info, warn};

use crate::core::errors::Result;

/// Placeholder ONNX Engine
///
/// The full ONNX implementation is planned for future release.
/// For now, this provides a simplified similarity calculation
/// based on keyword matching and basic heuristics.
pub struct OnnxEngine;

impl OnnxEngine {
    pub fn new() -> Result<Self> {
        info!("Initializing ONNX Engine (placeholder mode)");
        warn!("Full ONNX support not yet implemented - using keyword fallback");

        Ok(Self)
    }

    /// Generate a simple embedding based on keyword frequency
    /// This is a placeholder for actual ONNX model inference
    pub fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        debug!("Generating embedding for text of length: {}", text.len());

        // Simple keyword-based embedding (384 dimensions as placeholder)
        let mut embedding = vec![0.0f32; 384];
        let text_lower = text.to_lowercase();

        // Create a simple hash-based embedding from keywords
        let words: Vec<&str> = text_lower.split_whitespace().collect();

        for word in words.iter() {
            let hash = Self::simple_hash(word);
            let idx = (hash as usize) % 384;
            embedding[idx] += 1.0;
        }

        // Normalize
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            embedding.iter_mut().for_each(|x| *x /= norm);
        }

        Ok(embedding)
    }

    fn simple_hash(s: &str) -> u64 {
        let mut hash: u64 = 5381;
        for c in s.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u64);
        }
        hash
    }

    /// Calculate cosine similarity between two embeddings
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }

    /// Find similar documents based on embedding similarity
    pub fn find_similar(
        query_embedding: &[f32],
        candidates: &[(String, Vec<f32>)],
        threshold: f32,
        limit: usize,
    ) -> Vec<(String, f32)> {
        let mut similarities: Vec<(String, f32)> = candidates
            .iter()
            .map(|(id, emb)| {
                let sim = Self::cosine_similarity(query_embedding, emb);
                (id.clone(), sim)
            })
            .filter(|(_, sim)| *sim >= threshold)
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Take top N
        similarities.into_iter().take(limit).collect()
    }

    /// Calculate similarity between two texts using keyword overlap
    /// Fallback method when embeddings are not available
    pub fn text_similarity(a: &str, b: &str) -> f32 {
        let a_words: std::collections::HashSet<String> = a
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let b_words: std::collections::HashSet<String> = b
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if a_words.is_empty() || b_words.is_empty() {
            return 0.0;
        }

        let intersection: std::collections::HashSet<_> = a_words.intersection(&b_words).collect();

        let union: std::collections::HashSet<_> = a_words.union(&b_words).collect();

        intersection.len() as f32 / union.len() as f32
    }
}

impl Default for OnnxEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create OnnxEngine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let c = vec![1.0, 0.0, 0.0];

        assert!((OnnxEngine::cosine_similarity(&a, &b) - 0.0).abs() < 0.001);
        assert!((OnnxEngine::cosine_similarity(&a, &c) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_text_similarity() {
        let a = "hello world";
        let b = "hello universe";
        let c = "completely different";

        let sim_ab = OnnxEngine::text_similarity(a, b);
        let sim_ac = OnnxEngine::text_similarity(a, c);

        assert!(sim_ab > 0.0);
        assert!(sim_ac < sim_ab);
    }
}
