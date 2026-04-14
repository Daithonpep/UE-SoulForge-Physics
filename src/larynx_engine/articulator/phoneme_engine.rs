pub struct TimedPhoneme {
    pub phoneme: String,
    pub duration_ms: u32,
    pub is_voiced: bool,
    pub stress: f64,
}

pub struct PhonemeSequence {
    pub phonemes: Vec<TimedPhoneme>,
}

pub struct SpanishArticulator;

impl SpanishArticulator {
    pub fn new() -> Self {
        Self
    }

    pub fn text_to_phonemes(&self, text: &str) -> PhonemeSequence {
        let mut phonemes = Vec::new();
        for c in text.to_lowercase().chars() {
            let p = match c {
                'a' | 'á' => Some(("a", true, 1.0)),
                'e' | 'é' => Some(("e", true, 0.8)),
                'i' | 'í' => Some(("i", true, 0.7)),
                'o' | 'ó' => Some(("o", true, 0.9)),
                'u' | 'ú' => Some(("u", true, 0.6)),
                'm' => Some(("m", true, 0.5)),
                'n' => Some(("n", true, 0.5)),
                'l' => Some(("l", true, 0.5)),
                'r' => Some(("r", true, 0.6)),
                's' => Some(("s", false, 0.4)),
                ' ' => Some(("_", false, 0.2)),
                _ => None,
            };

            if let Some((ph, voiced, stress)) = p {
                phonemes.push(TimedPhoneme {
                    phoneme: ph.to_string(),
                    duration_ms: 100, // Duración base
                    is_voiced: voiced,
                    stress,
                });
            }
        }
        PhonemeSequence { phonemes }
    }
}
