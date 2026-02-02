use std::collections::HashSet;
use std::path::PathBuf;
use tracing::{debug, info, warn};

use crate::core::errors::Result;

/// ONNX-based embedding engine
/// Currently uses fallback keyword-based embeddings
/// Future: integrate tract-onnx for real model inference
pub struct OnnxEngine {
    model_path: Option<PathBuf>,
    is_ready: bool,
}

impl OnnxEngine {
    /// Create a new ONNX engine
    pub fn new() -> Result<Self> {
        info!("Initializing ONNX Engine (fallback mode)");

        Ok(Self {
            model_path: None,
            is_ready: true,
        })
    }

    /// Initialize with a specific model path
    pub fn with_model_path(model_path: PathBuf) -> Result<Self> {
        info!("Initializing ONNX Engine with model: {:?}", model_path);

        if !model_path.exists() {
            warn!("Model file not found at {:?}, using fallback", model_path);
        }

        Ok(Self {
            model_path: Some(model_path),
            is_ready: true,
        })
    }

    /// Check if the engine is ready
    pub fn is_ready(&self) -> bool {
        self.is_ready
    }

    /// Get model path if configured
    #[allow(dead_code)]
    pub fn model_path(&self) -> Option<&PathBuf> {
        self.model_path.as_ref()
    }

    /// Generate embedding for text using keyword frequency hashing
    /// This provides reasonable similarity detection for documents
    /// with overlapping vocabulary
    pub fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        debug!("Generating embedding for text of length: {}", text.len());

        let mut embedding = vec![0.0f32; 384];
        let text_lower = text.to_lowercase();

        // Word tokenization with basic cleanup
        let words: Vec<&str> = text_lower
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 2) // Skip short words
            .collect();

        // Generate hash-based embedding
        for word in words.iter() {
            let hash = Self::simple_hash(word);
            let idx = (hash as usize) % 384;
            embedding[idx] += 1.0;

            // Also add bigram influence for nearby dimensions
            let idx2 = ((hash >> 8) as usize) % 384;
            embedding[idx2] += 0.5;
        }

        // L2 normalize
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            embedding.iter_mut().for_each(|x| *x /= norm);
        }

        Ok(embedding)
    }

    fn simple_hash(s: &str) -> u64 {
        // FNV-1a hash
        let mut hash: u64 = 0xcbf29ce484222325;
        for c in s.bytes() {
            hash ^= c as u64;
            hash = hash.wrapping_mul(0x100000001b3);
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

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.into_iter().take(limit).collect()
    }

    /// Calculate Jaccard similarity between two texts
    pub fn text_similarity(a: &str, b: &str) -> f32 {
        let a_words: HashSet<String> = a
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 2)
            .map(|s| s.to_string())
            .collect();

        let b_words: HashSet<String> = b
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 2)
            .map(|s| s.to_string())
            .collect();

        if a_words.is_empty() || b_words.is_empty() {
            return 0.0;
        }

        let intersection = a_words.intersection(&b_words).count();
        let union = a_words.union(&b_words).count();

        intersection as f32 / union as f32
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
        let a = "hello world programming";
        let b = "hello universe programming";
        let c = "completely different topic";

        let sim_ab = OnnxEngine::text_similarity(a, b);
        let sim_ac = OnnxEngine::text_similarity(a, c);

        assert!(
            sim_ab > 0.0,
            "Similar texts should have positive similarity"
        );
        assert!(
            sim_ac < sim_ab,
            "Different texts should have lower similarity"
        );
    }

    #[test]
    fn test_embedding_generation() {
        let engine = OnnxEngine::new().unwrap();
        let embedding = engine
            .generate_embedding("hello world programming")
            .unwrap();

        assert_eq!(embedding.len(), 384);

        // Normalized embedding should have magnitude close to 1
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_find_similar() {
        let engine = OnnxEngine::new().unwrap();

        let query = engine
            .generate_embedding("rust programming language")
            .unwrap();
        let doc1 = engine
            .generate_embedding("rust systems programming")
            .unwrap();
        let doc2 = engine
            .generate_embedding("cooking recipes for dinner")
            .unwrap();

        let candidates = vec![("doc1".to_string(), doc1), ("doc2".to_string(), doc2)];

        let similar = OnnxEngine::find_similar(&query, &candidates, 0.0, 5);

        // doc1 should be more similar than doc2
        assert!(!similar.is_empty());
        if similar.len() >= 2 {
            assert!(
                similar[0].0 == "doc1",
                "Programming docs should be more similar"
            );
        }
    }
}
