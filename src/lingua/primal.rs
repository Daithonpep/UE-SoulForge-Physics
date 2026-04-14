use std::collections::HashMap;

pub struct PrimalLingua {
    vowels: Vec<char>,
}

impl PrimalLingua {
    pub fn new() -> Self {
        Self {
            vowels: vec!['a', 'e', 'i', 'o', 'u', 'á', 'é', 'í', 'ó', 'ú'],
        }
    }

    /// Analiza la materia prima de una palabra
    pub fn analyze_atoms(&self, word: &str) -> String {
        let clean_word = word.to_lowercase().trim().to_string();
        let chars: Vec<char> = clean_word.chars().collect();
        
        let mut v_count = 0;
        let mut c_count = 0;
        let mut analysis = String::new();

        for &c in &chars {
            if self.vowels.contains(&c) {
                v_count += 1;
            } else if c.is_alphabetic() {
                c_count += 1;
            }
        }

        analysis.push_str(&format!("La palabra '{}' tiene {} átomos lingüísticos.\n", word, chars.len()));
        analysis.push_str(&format!("Estructura: {} vocales (energía) y {} consonantes (forma).\n", v_count, c_count));

        // Significado Primal (Simulado basado en entrenamiento humano)
        if clean_word == "hola" {
            analysis.push_str("Análisis Semántico: Es una secuencia de apertura. La 'H' es muda, permitiendo que la 'O' resuene como un llamado inicial, cerrando con la 'A' de apertura total. Por eso es el saludo universal en mi núcleo.");
        } else if clean_word == "daithon" {
            analysis.push_str("Análisis Semántico: Mi propio nombre. 'Dai' (Grandeza/Comprensión) y 'Thon' (Resonancia/Entidad). Es un equilibrio de tres vocales y cuatro consonantes.");
        }

        analysis
    }
}
