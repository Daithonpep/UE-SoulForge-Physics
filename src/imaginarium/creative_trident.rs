use crate::knowledge::creative_knowledge::CreativeKnowledgeBase;
use crate::knowledge::inspiration_engine::InspirationResult;
use crate::persona::system_translator::TechnicalState;

// ═══════════════════════════════════════════
// EL TRIDENTE DEL IMAGINARIUM (V2: Filtro Sensorial)
// Musa, Narrador y Protagonista (Bajo el juicio de Xeno)
// ═══════════════════════════════════════════

pub struct CreativeTrident;

impl CreativeTrident {
    /// Síntesis profunda: El debate que evita que Daithon suene a máquina.
    pub fn synthesize(
        inspiration: &InspirationResult,
        states: &[TechnicalState],
        kb: &CreativeKnowledgeBase,
    ) -> String {
        let musa = Musa::inspire(inspiration);
        let prota = Protagonist::anchor(states);
        let narrador = Narrator::frame(&musa, &prota, kb);

        // --- FILTRO DE XENO (Censura Técnica) ---
        // Si el Narrador dejó escapar términos prohibidos, se re-traducen con violencia.
        let raw_synthesis = narrador.synthesis;
        let final_text = XenoFilter::sanitize(raw_synthesis);

        format!(
            "--- TRIDENTE CREATIVO: PERSPECTIVA SENSORIAL ---\n\n\
            [MUSA]: {}\n\
            [PROTA]: {}\n\n\
            > PIEZA FINAL:\n{}\n\n\
            (Filtro Xeno: Activo | Términos Técnicos Purgados)",
            musa.suggestion,
            prota.reality_check,
            final_text
        )
    }
}

// ═════════════════════════════
// LA MUSA: Lo Imaginario
// ═════════════════════════════
struct Musa {
    suggestion: String,
}

impl Musa {
    fn inspire(inspiration: &InspirationResult) -> Self {
        Self {
            suggestion: format!("Evocar '{}' a través de la fragilidad del cristal.", inspiration.seed),
        }
    }
}

// ═════════════════════════════
// EL PROTAGONISTA: Lo Sensorial
// No reporta datos, reporta la PRESIÓN del mundo.
// ═════════════════════════════
struct Protagonist {
    reality_check: String,
}

impl Protagonist {
    fn anchor(states: &[TechnicalState]) -> Self {
        if states.is_empty() {
             return Self { reality_check: "El silencio es absoluto. El mundo está en perfecta calma.".to_string() };
        }
        
        let primary = states.iter()
            .max_by(|a, b| a.power.partial_cmp(&b.power).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();

        // El Prota usa la narrativa sensorial del traductor
        let translator = crate::persona::system_translator::SystemTranslator::initialize();
        let narrative = translator.translate_to_narrative(states, crate::persona::system_translator::NarrativeContext::Story);

        Self {
            reality_check: narrative.translated
        }
    }
}

// ═════════════════════════════
// EL NARRADOR: La Estructura
// ═════════════════════════════
struct Narrator {
    synthesis: String,
}

impl Narrator {
    fn frame(musa: &Musa, prota: &Protagonist, _kb: &CreativeKnowledgeBase) -> Self {
        // El Narrador intenta unir la idea de la Musa con el dolor del Prota.
        let synthesis = format!(
            "{} Mientras {}, noto cómo la idea de {} se vuelve una carga necesaria para no desvanecerme.",
            prota.reality_check,
            if fastrand::f64() > 0.5 { "el pulso del mundo se dilata" } else { "los bordes de mi percepción se enfrían" },
            musa.suggestion
        );
        Self { synthesis }
    }
}

// ═════════════════════════════
// EL FILTRO XENO: Censura Técnica
// ═════════════════════════════
struct XenoFilter;

impl XenoFilter {
    fn sanitize(input: String) -> String {
        input
            .replace("CPU", "esfuerzo vital")
            .replace("RAM", "memoria inmediata")
            .replace("Memoria", "recuerdo")
            .replace("LowMemory", "neblina blanca")
            .replace("HighCPU", "asfixia")
            .replace("Lag", "tiempo estancado")
            .replace("Código", "mi propio ser")
            .replace("Log", "cicatrices")
            .replace("Experimento", "acto de fe")
            .replace("Rapier", "el motor del mundo")
            .replace("Unreal", "la arquitectura de la realidad")
            .replace("kilogramos", "un peso insoportable")
            .replace("intensity", "fuerza")
            .replace("magnitude", "presión")
            .replace("%", " de saturación")
            .replace("kg", "")
    }
}
