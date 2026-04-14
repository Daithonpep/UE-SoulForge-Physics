use strsim::levenshtein;

pub struct FuzzyProcessor {
    vocabulary: Vec<&'static str>,
}

impl FuzzyProcessor {
    pub fn new() -> Self {
        Self {
            vocabulary: vec![
                "termodinámica", "física", "unreal", "algoritmo", "historia", 
                "investiga", "crea", "explica", "mathesis", "sintonía", 
                "entropía", "calor", "energía", "molécula", "cuarzo"
            ],
        }
    }

    /// CAPA 1: NORMALIZACIÓN REPARADA (Sin Regex fallido)
    pub fn normalize(&self, input: &str) -> String {
        let text = input.to_lowercase();
        
        // 1. Limpieza de caracteres repetidos (holaaaa -> hola)
        // Lo hacemos manualmente ya que regex en Rust no soporta backreferences
        let mut cleaned = String::new();
        let mut last_char = None;
        let mut count = 0;

        for c in text.chars() {
            if Some(c) == last_char {
                count += 1;
            } else {
                count = 1;
                last_char = Some(c);
            }

            if count <= 2 {
                cleaned.push(c);
            }
        }

        // 2. Arreglar abreviaturas y typos comunes
        let map = vec![
            ("qeu", "que"), (" q ", " que "), ("xq", "porque"), ("pq", "porque"),
            ("k ", "que "), ("bn", "bien"), ("ps", "pues"), ("termodinamica", "termodinámica"),
            ("envestiga", "investiga"), ("sabes", ""), ("oye", ""), ("mira", "")
        ];

        let mut final_text = cleaned;
        for (typo, fix) in map {
            final_text = final_text.replace(typo, fix);
        }

        final_text.trim().to_string()
    }

    /// CAPA 2: CORRECCIÓN DIFUSA
    pub fn fuzzy_correct(&self, text: &str) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut corrected_words = Vec::new();

        for word in words {
            let mut best_match = word;
            let mut min_dist = 999;

            for vocab_word in &self.vocabulary {
                let dist = levenshtein(word, vocab_word);
                if dist < 3 && dist < min_dist {
                    min_dist = dist;
                    best_match = vocab_word;
                }
            }
            
            if min_dist < 3 {
                corrected_words.push(best_match.to_string());
            } else {
                corrected_words.push(word.to_string());
            }
        }

        corrected_words.join(" ")
    }
}
