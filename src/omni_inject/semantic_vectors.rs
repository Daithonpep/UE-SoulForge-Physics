use std::collections::HashMap;

/// Sistema de embeddings para comprensión semántica
pub struct SemanticEmbeddingEngine {
    /// Vectores de palabras (word → vector de 300 dimensiones)
    pub word_vectors: HashMap<String, Vec<f32>>,
    /// Dimensionalidad de vectores
    pub dimension: usize,
}

impl SemanticEmbeddingEngine {
    pub fn new() -> Self {
        Self {
            word_vectors: HashMap::new(),
            dimension: 300,
        }
    }

    /// Cargar FastText embeddings pre-entrenados
    pub fn load_fasttext(&mut self, model_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("[EMBEDDINGS] Cargando FastText desde: {}", model_path);

        let content = std::fs::read_to_string(model_path)?;
        let mut loaded = 0;

        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < self.dimension + 1 { continue; }

            let word = parts[0].to_string();
            let vector: Vec<f32> = parts[1..]
                .iter()
                .filter_map(|s| s.parse::<f32>().ok())
                .collect();

            if vector.len() == self.dimension {
                self.word_vectors.insert(word, vector);
                loaded += 1;

                if loaded % 10000 == 0 {
                    print!("\r  Cargadas: {} palabras", loaded);
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                }
            }
        }

        println!("\n✓ Embeddings cargados: {} palabras", loaded);
        Ok(())
    }

    /// Obtener vector de una palabra
    pub fn get_vector(&self, word: &str) -> Option<&Vec<f32>> {
        self.word_vectors.get(&word.to_lowercase())
    }

    /// Calcular similitud coseno entre dos palabras
    pub fn similarity(&self, word_a: &str, word_b: &str) -> f32 {
        let vec_a = match self.get_vector(word_a) { Some(v) => v, None => return 0.0 };
        let vec_b = match self.get_vector(word_b) { Some(v) => v, None => return 0.0 };
        Self::cosine_similarity(vec_a, vec_b)
    }

    /// Encontrar palabras más similares a una dada
    pub fn find_similar(&self, word: &str, top_k: usize) -> Vec<(String, f32)> {
        let target_vec = match self.get_vector(word) { Some(v) => v, None => return vec![] };

        let mut similarities: Vec<(String, f32)> = self.word_vectors.iter()
            .filter(|(w, _)| *w != word)
            .map(|(w, v)| {
                let sim = Self::cosine_similarity(target_vec, v);
                (w.clone(), sim)
            })
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.into_iter().take(top_k).collect()
    }

    /// Inferir significado de palabra desconocida
    pub fn infer_unknown_word(&self, unknown: &str) -> Option<WordInference> {
        if self.word_vectors.contains_key(&unknown.to_lowercase()) {
            return None;
        }

        let similar = self.find_similar_by_morphology(unknown);
        if let Some((similar_word, similarity)) = similar.first() {
            if *similarity > 0.7 {
                return Some(WordInference {
                    unknown_word: unknown.to_string(),
                    inferred_meaning: format!("Probablemente un tipo de '{}' o relacionado", similar_word),
                    confidence: *similarity,
                    similar_words: vec![similar_word.clone()],
                });
            }
        }
        None
    }

    fn find_similar_by_morphology(&self, word: &str) -> Vec<(String, f32)> {
        let word_lower = word.to_lowercase();
        let mut candidates = Vec::new();

        if word_lower.len() >= 4 {
            let prefix = &word_lower[..4];
            for (vocab_word, _) in &self.word_vectors {
                if vocab_word.starts_with(prefix) && vocab_word != &word_lower {
                    candidates.push((vocab_word.clone(), 0.8));
                }
            }
        }

        if word_lower.len() >= 3 {
            let suffix = &word_lower[word_lower.len()-3..];
            for (vocab_word, _) in &self.word_vectors {
                if vocab_word.ends_with(suffix) && vocab_word != &word_lower {
                    if !candidates.iter().any(|(w, _)| w == vocab_word) {
                        candidates.push((vocab_word.clone(), 0.6));
                    }
                }
            }
        }

        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates.into_iter().take(5).collect()
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() { return 0.0; }
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude_a == 0.0 || magnitude_b == 0.0 { return 0.0; }
        dot_product / (magnitude_a * magnitude_b)
    }

    /// Generar embedding sintético para palabra nueva
    pub fn synthesize_embedding(&mut self, new_word: &str, definition: &str) -> Vec<f32> {
        let words: Vec<&str> = definition.split_whitespace().collect();
        let mut sum_vector = vec![0.0f32; self.dimension];
        let mut count = 0;

        for word in words {
            if let Some(vec) = self.get_vector(word) {
                for (i, &val) in vec.iter().enumerate() {
                    sum_vector[i] += val;
                }
                count += 1;
            }
        }

        if count > 0 {
            for val in &mut sum_vector { *val /= count as f32; }
        }

        let magnitude: f32 = sum_vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for val in &mut sum_vector { *val /= magnitude; }
        }

        self.word_vectors.insert(new_word.to_lowercase(), sum_vector.clone());
        sum_vector
    }
}

#[derive(Debug, Clone)]
pub struct WordInference {
    pub unknown_word: String,
    pub inferred_meaning: String,
    pub confidence: f32,
    pub similar_words: Vec<String>,
}
