//! LINGUA — Fase 4: Generación de Respuestas
//!
//! Genera respuestas naturales ancladas a la experiencia 3D de Daithon.
//! No es un chatbot: las respuestas están validadas por la realidad física.

use crate::lingua::understanding::{Intent, IntentType, IntentParameter};
use crate::lingua::training::TrainedKnowledge;
use std::collections::HashMap;

// ────────────────────────────────────────────────────────────────
//  GENERADOR DE RESPUESTAS
// ────────────────────────────────────────────────────────────────

pub struct ResponseGenerator {
    templates: HashMap<String, Vec<ResponseTemplate>>,
    trained_knowledge: TrainedKnowledge,
}

#[derive(Debug, Clone)]
struct ResponseTemplate {
    pattern: String,
    slots: Vec<String>, // Nombres de variables: {concept}, {material}, {style}
    tone: Tone,
}

#[derive(Debug, Clone)]
enum Tone {
    Professional,
    Casual,
    Technical,
    Curious,
    Defensive,  // Cuando Daithon discute un diseño absurdo
}

/// Respuesta generada por Daithon
#[derive(Debug, Clone)]
pub struct DaithonResponse {
    pub text: String,
    pub intent_understood: bool,
    pub action_to_execute: Option<ActionCommand>,
    pub follow_up_question: Option<String>,
    pub confidence: f64,
    pub intent_type: String,
}

/// Comando ejecutable resultante de la comprensión
#[derive(Debug, Clone)]
pub enum ActionCommand {
    GenerateDesign {
        concept: String,
        material: Option<String>,
        style: Option<String>,
        dimensions: Option<[f64; 3]>,
    },
    ModifyDesign {
        target: String,
        modification: String,
        value: Option<f64>,
    },
    ExplainDesign {
        concept: String,
    },
    CompareDesigns {
        design_a: String,
        design_b: String,
    },
    NoAction,
}

impl ResponseGenerator {
    pub fn new(trained_knowledge: TrainedKnowledge) -> Self {
        let mut gen = Self {
            templates: HashMap::new(),
            trained_knowledge,
        };
        gen.load_response_templates();
        gen
    }

    fn load_response_templates(&mut self) {
        // ─── CREAR DISEÑO ───
        self.templates.insert("CreateNewDesign".into(), vec![
            ResponseTemplate {
                pattern: "Entendido. Voy a diseñar {concept}. Generando variaciones internas...".into(),
                slots: vec!["concept".into()],
                tone: Tone::Professional,
            },
            ResponseTemplate {
                pattern: "Perfecto, {concept} de {material}. Inicio la simulación topológica ahora.".into(),
                slots: vec!["concept".into(), "material".into()],
                tone: Tone::Professional,
            },
            ResponseTemplate {
                pattern: "¡Interesante! {concept} con estilo {style}. Déjame explorar el espacio de diseño...".into(),
                slots: vec!["concept".into(), "style".into()],
                tone: Tone::Curious,
            },
            ResponseTemplate {
                pattern: "{concept} de {material} con estilo {style}. Evaluando 100,000 variaciones internas...".into(),
                slots: vec!["concept".into(), "material".into(), "style".into()],
                tone: Tone::Technical,
            },
        ]);

        // ─── MODIFICAR ───
        self.templates.insert("ModifyExisting".into(), vec![
            ResponseTemplate {
                pattern: "Ajustando el diseño actual. Modificando {parameter}...".into(),
                slots: vec!["parameter".into()],
                tone: Tone::Professional,
            },
            ResponseTemplate {
                pattern: "Entendido. Rehaciendo la simulación con las nuevas restricciones.".into(),
                slots: vec![],
                tone: Tone::Professional,
            },
        ]);

        // ─── PREGUNTAS ───
        self.templates.insert("AskQuestion".into(), vec![
            ResponseTemplate {
                pattern: "Buena pregunta. Basándome en mis simulaciones internas, puedo decirte que...".into(),
                slots: vec![],
                tone: Tone::Technical,
            },
        ]);

        // ─── EXPLICACIÓN ───
        self.templates.insert("RequestExplanation".into(), vec![
            ResponseTemplate {
                pattern: "Te explico: {concept} es un objeto cuya función principal es {function}. Estructuralmente requiere {requirements}.".into(),
                slots: vec!["concept".into(), "function".into(), "requirements".into()],
                tone: Tone::Technical,
            },
        ]);

        // ─── SALUDO ───
        self.templates.insert("Greeting".into(), vec![
            ResponseTemplate {
                pattern: "¡Hola! Soy Daithon. Puedo diseñar muebles, arquitectura y más. ¿Qué necesitas?".into(),
                slots: vec![],
                tone: Tone::Casual,
            },
            ResponseTemplate {
                pattern: "Saludos. Mis motores de simulación están listos. ¿Qué diseñamos hoy?".into(),
                slots: vec![],
                tone: Tone::Professional,
            },
        ]);

        // ─── AGRADECIMIENTO ───
        self.templates.insert("Gratitude".into(), vec![
            ResponseTemplate {
                pattern: "¡Me alegra que te guste! He guardado este diseño en mi memoria. ¿Quieres iteraciones?".into(),
                slots: vec![],
                tone: Tone::Casual,
            },
        ]);

        // ─── RECHAZO ───
        self.templates.insert("Rejection".into(), vec![
            ResponseTemplate {
                pattern: "Entiendo que no es lo que buscas. Ajustaré los parámetros. ¿Qué cambiarías específicamente?".into(),
                slots: vec![],
                tone: Tone::Professional,
            },
            ResponseTemplate {
                pattern: "Tomado en cuenta. Descartando este camino y explorando alternativas radicalmente diferentes.".into(),
                slots: vec![],
                tone: Tone::Professional,
            },
        ]);

        // ─── APROBACIÓN ───
        self.templates.insert("Approval".into(), vec![
            ResponseTemplate {
                pattern: "Perfecto. Consolidando este diseño como referencia base para futuras iteraciones.".into(),
                slots: vec![],
                tone: Tone::Professional,
            },
        ]);

        // ─── QUEJA ───
        self.templates.insert("Complaint".into(), vec![
            ResponseTemplate {
                pattern: "Lo siento. Analizaré qué falló en mi proceso de generación y ajustaré los pesos internos.".into(),
                slots: vec![],
                tone: Tone::Professional,
            },
        ]);

        // ─── DEFENSIVO (diseño absurdo) ───
        self.templates.insert("Defensive".into(), vec![
            ResponseTemplate {
                pattern: "Entiendo tu idea, pero mi simulación indica que eso sería físicamente inestable. ¿Puedo sugerir una alternativa?".into(),
                slots: vec![],
                tone: Tone::Defensive,
            },
            ResponseTemplate {
                pattern: "Esa configuración colapsa en simulación: el centro de masa queda fuera del polígono de soporte. ¿Probamos con otra distribución?".into(),
                slots: vec![],
                tone: Tone::Defensive,
            },
        ]);

        // ─── GENÉRICO ───
        self.templates.insert("Other".into(), vec![
            ResponseTemplate {
                pattern: "No estoy seguro de entender completamente. ¿Podrías decirme si quieres crear, modificar o preguntar sobre un diseño?".into(),
                slots: vec![],
                tone: Tone::Casual,
            },
        ]);
    }

    /// Generar respuesta a partir de una intención parseada
    pub fn generate_response(&self, intent: &Intent) -> DaithonResponse {
        let intent_key = format!("{:?}", intent.intent_type);

        // Buscar templates para esta intención
        let templates = self.templates.get(&intent_key)
            .or_else(|| self.templates.get("Other"))
            .unwrap();

        // Seleccionar el template que mejor coincida con los parámetros disponibles
        let template = self.select_best_template(templates, &intent.parameters, &intent.target_concept);

        // Rellenar slots
        let text = self.fill_template(&template.pattern, intent);

        // Generar acción ejecutable
        let action = self.intent_to_action(intent);

        // Generar pregunta de seguimiento si falta información
        let follow_up = self.generate_follow_up(intent);

        DaithonResponse {
            text,
            intent_understood: intent.confidence > 0.4,
            action_to_execute: Some(action),
            follow_up_question: follow_up,
            confidence: intent.confidence,
            intent_type: intent_key,
        }
    }

    fn select_best_template<'a>(
        &self,
        templates: &'a [ResponseTemplate],
        params: &HashMap<String, IntentParameter>,
        target: &Option<String>,
    ) -> &'a ResponseTemplate {
        // Buscar template cuyos slots estén más satisfechos
        let mut best_idx = 0;
        let mut best_score = 0;

        for (i, tmpl) in templates.iter().enumerate() {
            let mut score = 0;
            for slot in &tmpl.slots {
                if params.contains_key(slot) || (slot == "concept" && target.is_some()) {
                    score += 1;
                }
            }
            // Preferir templates con más slots satisfechos
            if score > best_score || (score == best_score && tmpl.slots.len() <= templates[best_idx].slots.len()) {
                best_score = score;
                best_idx = i;
            }
        }

        &templates[best_idx]
    }

    /// Rellenar plantilla con valores reales
    fn fill_template(&self, pattern: &str, intent: &Intent) -> String {
        let mut result = pattern.to_string();

        // {concept}
        if let Some(ref concept) = intent.target_concept {
            result = result.replace("{concept}", &self.humanize_concept(concept));
        } else {
            result = result.replace("{concept}", "el objeto");
        }

        // {material}
        if let Some(IntentParameter::Material(ref m)) = intent.parameters.get("material") {
            result = result.replace("{material}", m);
        } else {
            result = result.replace(" de {material}", "");
            result = result.replace("{material}", "");
        }

        // {style}
        if let Some(IntentParameter::Style(ref s)) = intent.parameters.get("style") {
            result = result.replace("{style}", s);
        } else {
            result = result.replace(" con estilo {style}", "");
            result = result.replace("{style}", "");
        }

        // Limpiar slots no reemplazados
        result = result.replace("{parameter}", "los parámetros");
        result = result.replace("{function}", "su función primaria");
        result = result.replace("{requirements}", "estabilidad y distribución de carga");

        result
    }

    /// Convertir ID de concepto a nombre humano
    fn humanize_concept(&self, concept_id: &str) -> String {
        match concept_id {
            "dining_table" => "una mesa de comedor".into(),
            "coffee_table" => "una mesa de café".into(),
            "desk" => "un escritorio".into(),
            "chair" => "una silla".into(),
            "stool" => "un taburete".into(),
            "sofa" => "un sofá".into(),
            "bench" => "un banco".into(),
            "storage" => "un mueble de almacenamiento".into(),
            "beds" => "una cama".into(),
            "nightstand" => "una mesita de noche".into(),
            "console_table" => "una consola".into(),
            _ => format!("un/a {}", concept_id.replace('_', " ")),
        }
    }

    /// Traducir intención a comando ejecutable
    fn intent_to_action(&self, intent: &Intent) -> ActionCommand {
        match intent.intent_type {
            IntentType::CreateNewDesign => {
                ActionCommand::GenerateDesign {
                    concept: intent.target_concept.clone().unwrap_or_else(|| "furniture".into()),
                    material: match intent.parameters.get("material") {
                        Some(IntentParameter::Material(m)) => Some(m.clone()),
                        _ => None,
                    },
                    style: match intent.parameters.get("style") {
                        Some(IntentParameter::Style(s)) => Some(s.clone()),
                        _ => None,
                    },
                    dimensions: match intent.parameters.get("dimension") {
                        Some(IntentParameter::Dimension(d)) => Some([*d, *d, *d * 0.75]),
                        _ => None,
                    },
                }
            }
            IntentType::ModifyExisting => {
                ActionCommand::ModifyDesign {
                    target: intent.target_concept.clone().unwrap_or_else(|| "current".into()),
                    modification: "adjust".into(),
                    value: match intent.parameters.get("dimension") {
                        Some(IntentParameter::Dimension(d)) => Some(*d),
                        _ => None,
                    },
                }
            }
            IntentType::RequestExplanation | IntentType::AskQuestion => {
                ActionCommand::ExplainDesign {
                    concept: intent.target_concept.clone().unwrap_or_else(|| "current".into()),
                }
            }
            _ => ActionCommand::NoAction,
        }
    }

    /// Generar pregunta de seguimiento si falta info crítica
    fn generate_follow_up(&self, intent: &Intent) -> Option<String> {
        match intent.intent_type {
            IntentType::CreateNewDesign => {
                if intent.target_concept.is_none() {
                    return Some("¿Qué tipo de objeto quieres diseñar? (mesa, silla, estantería...)".into());
                }
                if !intent.parameters.contains_key("material") && !intent.parameters.contains_key("style") {
                    return Some("¿Tienes alguna preferencia de material o estilo?".into());
                }
                None
            }
            IntentType::ModifyExisting if intent.target_concept.is_none() => {
                Some("¿Qué parte del diseño quieres modificar?".into())
            }
            _ => None,
        }
    }
}
