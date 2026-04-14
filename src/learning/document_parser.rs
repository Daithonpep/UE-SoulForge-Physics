// src/learning/document_parser.rs
// ============================================================
// PARSER DE DOCUMENTACIÓN: Convierte texto libre en conocimiento
// ============================================================
// Este módulo NO es específico de ajedrez. Funciona con CUALQUIER
// manual: ajedrez, Python, física, cocina, etc.
// ============================================================

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Conocimiento estructurado extraído de un documento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedKnowledge {
    pub domain: String,
    pub entities: Vec<Entity>,
    pub rules: Vec<Rule>,
    pub goals: Vec<Goal>,
    pub constraints: Vec<Constraint>,
    pub vocabulary: Vec<String>,       // Términos clave del dominio
    pub raw_sections: Vec<Section>,    // Secciones del manual parseado
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub category: String,              // "Pieza", "Tablero", "Variable", "Operador"
    pub properties: HashMap<String, String>,
    pub related_to: Vec<String>,       // Otras entidades relacionadas
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub subject: String,               // "Pawn", "for_loop"
    pub action: String,                // "moves", "iterates"
    pub condition: String,             // "forward 1 square if path is clear"
    pub exceptions: Vec<String>,       // "First move can be 2 squares"
    pub confidence: f32,               // Comienza en 1.0 (viene del manual)
    pub source: String,                // "manual", "discovered", "analogized"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub description: String,
    pub win_condition: String,
    pub sub_goals: Vec<String>,        // Objetivos intermedios
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub description: String,
    pub applies_to: String,            // Entidad o regla afectada
    pub severity: String,              // "hard" (violación = ilegal), "soft" (mala estrategia)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub content: String,
    pub key_terms: Vec<String>,
}

/// Motor de parseo: Extrae conocimiento de texto libre usando NLP ligero
pub struct DocumentParser;

impl DocumentParser {
    /// Parsea un manual completo y extrae conocimiento estructurado
    pub fn parse(domain_name: &str, manual_text: &str) -> ParsedKnowledge {
        println!("📖 [PARSER] Analizando documentación de '{}'...", domain_name);
        
        let sections = Self::split_into_sections(manual_text);
        let entities = Self::extract_entities(&sections);
        let rules = Self::extract_rules(&sections);
        let goals = Self::extract_goals(&sections);
        let constraints = Self::extract_constraints(&sections);
        let vocabulary = Self::extract_vocabulary(manual_text);
        
        println!("   ✅ Entidades: {}", entities.len());
        println!("   ✅ Reglas: {}", rules.len());
        println!("   ✅ Objetivos: {}", goals.len());
        println!("   ✅ Restricciones: {}", constraints.len());
        println!("   ✅ Vocabulario: {} términos", vocabulary.len());
        
        ParsedKnowledge {
            domain: domain_name.to_string(),
            entities,
            rules,
            goals,
            constraints,
            vocabulary,
            raw_sections: sections,
        }
    }

    /// Divide el texto en secciones por encabezados
    fn split_into_sections(text: &str) -> Vec<Section> {
        let mut sections = Vec::new();
        let mut current_title = "General".to_string();
        let mut current_content = String::new();
        
        for line in text.lines() {
            let trimmed = line.trim();
            
            // Detectar encabezados: "PIECES:", "RULES:", "## Syntax", etc.
            if (trimmed.ends_with(':') && trimmed.chars().filter(|c| c.is_uppercase()).count() > trimmed.len() / 2)
                || trimmed.starts_with('#')
                || trimmed.starts_with("===")
            {
                if !current_content.trim().is_empty() {
                    let key_terms = Self::extract_vocabulary(&current_content);
                    sections.push(Section {
                        title: current_title.clone(),
                        content: current_content.trim().to_string(),
                        key_terms,
                    });
                }
                current_title = trimmed.trim_matches(|c: char| c == ':' || c == '#' || c == '=' || c == ' ').to_string();
                current_content = String::new();
            } else {
                current_content.push_str(trimmed);
                current_content.push('\n');
            }
        }
        
        // Última sección
        if !current_content.trim().is_empty() {
            let key_terms = Self::extract_vocabulary(&current_content);
            sections.push(Section {
                title: current_title,
                content: current_content.trim().to_string(),
                key_terms,
            });
        }
        
        sections
    }

    /// Extrae entidades (sustantivos capitalizados, patrones "- Name: ...")
    fn extract_entities(sections: &[Section]) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for section in sections {
            for line in section.content.lines() {
                let trimmed = line.trim();
                
                // Patrón: "- Name: Description" o "- Name (attr): Description"
                if trimmed.starts_with('-') || trimmed.starts_with('•') || trimmed.starts_with('*') {
                    let content = trimmed.trim_start_matches(|c: char| c == '-' || c == '•' || c == '*' || c == ' ');
                    
                    if let Some(colon_pos) = content.find(':') {
                        let name = content[..colon_pos].trim().to_string();
                        let desc = content[colon_pos+1..].trim().to_string();
                        
                        if !name.is_empty() && !seen.contains(&name.to_lowercase()) {
                            seen.insert(name.to_lowercase());
                            
                            let mut properties = HashMap::new();
                            properties.insert("description".to_string(), desc.clone());
                            properties.insert("source_section".to_string(), section.title.clone());
                            
                            // Extraer propiedades inline
                            Self::extract_inline_properties(&desc, &mut properties);
                            
                            entities.push(Entity {
                                name: name.clone(),
                                category: section.title.clone(),
                                properties,
                                related_to: Self::find_references(&desc, &seen),
                            });
                        }
                    }
                }
            }
        }
        
        entities
    }

    /// Extrae reglas del texto (patrones de acción-condición)
    fn extract_rules(sections: &[Section]) -> Vec<Rule> {
        let mut rules = Vec::new();
        let mut rule_id = 0;
        
        let action_verbs = [
            "moves", "mueve", "captures", "captura", "attacks", "ataca",
            "returns", "retorna", "iterates", "itera", "executes", "ejecuta",
            "combines", "combina", "jumps", "salta", "can", "puede",
            "allows", "permite", "creates", "crea", "defines", "define",
        ];
        
        for section in sections {
            for line in section.content.lines() {
                let trimmed = line.trim().to_lowercase();
                
                // Buscar verbos de acción
                for verb in &action_verbs {
                    if trimmed.contains(verb) {
                        rule_id += 1;
                        
                        // Extraer sujeto (antes del verbo)
                        let subject = Self::extract_subject_before_verb(&trimmed, verb);
                        
                        // Extraer condición (después del verbo)
                        let (action, condition) = Self::extract_action_and_condition(&trimmed, verb);
                        
                        // Buscar excepciones en la misma línea o siguiente
                        let exceptions = Self::extract_exceptions(line);
                        
                        if !subject.is_empty() {
                            rules.push(Rule {
                                id: format!("rule_{}", rule_id),
                                subject,
                                action,
                                condition,
                                exceptions,
                                confidence: 1.0, // Viene del manual = máxima confianza
                                source: "manual".to_string(),
                            });
                        }
                        break; // Solo un verbo por línea
                    }
                }
            }
        }
        
        rules
    }

    /// Extrae objetivos/metas del documento
    fn extract_goals(sections: &[Section]) -> Vec<Goal> {
        let mut goals = Vec::new();
        
        let goal_keywords = [
            "objective", "objetivo", "goal", "meta", "win", "ganar",
            "checkmate", "jaque mate", "purpose", "propósito",
            "target", "solve", "resolver",
        ];
        
        for section in sections {
            let content_lower = section.content.to_lowercase();
            
            for keyword in &goal_keywords {
                if content_lower.contains(keyword) {
                    // Extraer la línea que contiene el objetivo
                    for line in section.content.lines() {
                        if line.to_lowercase().contains(keyword) {
                            goals.push(Goal {
                                description: line.trim().to_string(),
                                win_condition: line.trim().to_string(),
                                sub_goals: Vec::new(),
                            });
                        }
                    }
                }
            }
        }
        
        // Deduplicar
        goals.dedup_by(|a, b| a.description == b.description);
        goals
    }

    /// Extrae restricciones (lo que NO se puede hacer)
    fn extract_constraints(sections: &[Section]) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        
        let constraint_patterns = [
            "cannot", "no puede", "not allowed", "no permitido",
            "must not", "no debe", "forbidden", "prohibido",
            "illegal", "ilegal", "invalid", "inválido",
            "restriction", "restricción",
        ];
        
        for section in sections {
            for line in section.content.lines() {
                let lower = line.to_lowercase();
                
                for pattern in &constraint_patterns {
                    if lower.contains(pattern) {
                        constraints.push(Constraint {
                            description: line.trim().to_string(),
                            applies_to: section.title.clone(),
                            severity: if lower.contains("cannot") || lower.contains("must not") || lower.contains("illegal") {
                                "hard".to_string()
                            } else {
                                "soft".to_string()
                            },
                        });
                        break;
                    }
                }
            }
        }
        
        constraints
    }

    /// Extrae vocabulario clave del texto
    fn extract_vocabulary(text: &str) -> Vec<String> {
        let stop_words: std::collections::HashSet<&str> = [
            "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "shall",
            "should", "may", "might", "must", "can", "could", "to", "of", "in",
            "for", "on", "with", "at", "by", "from", "or", "and", "not", "but",
            "if", "then", "else", "when", "up", "down", "its", "it", "this",
            "that", "el", "la", "los", "las", "de", "en", "un", "una", "y",
            "del", "al", "se", "es", "que", "por", "con", "no", "su", "para",
        ].into_iter().collect();
        
        let mut word_freq: HashMap<String, usize> = HashMap::new();
        
        for word in text.split(|c: char| !c.is_alphanumeric()) {
            let lower = word.to_lowercase();
            if lower.len() > 2 && !stop_words.contains(lower.as_str()) {
                *word_freq.entry(lower).or_insert(0) += 1;
            }
        }
        
        let mut vocab: Vec<(String, usize)> = word_freq.into_iter().collect();
        vocab.sort_by(|a, b| b.1.cmp(&a.1));
        vocab.into_iter().take(50).map(|(w, _)| w).collect()
    }

    // === HELPERS ===

    fn extract_inline_properties(desc: &str, props: &mut HashMap<String, String>) {
        // Buscar patrones como "X squares", "any direction", etc.
        if desc.contains("square") || desc.contains("casilla") {
            props.insert("movement_type".to_string(), "grid_based".to_string());
        }
        if desc.contains("any direction") || desc.contains("cualquier dirección") {
            props.insert("direction".to_string(), "omnidirectional".to_string());
        }
        if desc.contains("L-shape") || desc.contains("forma de L") {
            props.insert("movement_pattern".to_string(), "L_shape".to_string());
        }
        if desc.contains("diagonal") {
            props.insert("direction".to_string(), "diagonal".to_string());
        }
        if desc.contains("horizontal") || desc.contains("vertical") {
            props.insert("direction".to_string(), "orthogonal".to_string());
        }
    }

    fn find_references(text: &str, known_entities: &std::collections::HashSet<String>) -> Vec<String> {
        known_entities.iter()
            .filter(|e| text.to_lowercase().contains(e.as_str()))
            .cloned()
            .collect()
    }

    fn extract_subject_before_verb(line: &str, verb: &str) -> String {
        if let Some(pos) = line.find(verb) {
            let before = line[..pos].trim();
            // Tomar las últimas 1-3 palabras antes del verbo
            let words: Vec<&str> = before.split_whitespace().collect();
            if words.len() > 0 {
                words[words.len().saturating_sub(2)..].join(" ")
                    .trim_matches(|c: char| c == '-' || c == '•' || c == '*' || c == ' ')
                    .to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    fn extract_action_and_condition(line: &str, verb: &str) -> (String, String) {
        if let Some(pos) = line.find(verb) {
            let after = line[pos..].trim().to_string();
            // Separar por "if", "when", "under", "si", "cuando"
            for separator in &["if ", "when ", "under ", "si ", "cuando "] {
                if let Some(sep_pos) = after.find(separator) {
                    return (
                        after[..sep_pos].trim().to_string(),
                        after[sep_pos..].trim().to_string(),
                    );
                }
            }
            (after, String::new())
        } else {
            (String::new(), String::new())
        }
    }

    fn extract_exceptions(line: &str) -> Vec<String> {
        let mut exceptions = Vec::new();
        let lower = line.to_lowercase();
        
        for marker in &["except", "unless", "but ", "however", "excepto", "salvo", "pero "] {
            if let Some(pos) = lower.find(marker) {
                exceptions.push(line[pos..].trim().to_string());
            }
        }
        
        // Paréntesis como excepciones: "(First move can be 2 squares)"
        if let Some(paren_start) = line.find('(') {
            if let Some(paren_end) = line.find(')') {
                if paren_end > paren_start {
                    exceptions.push(line[paren_start+1..paren_end].to_string());
                }
            }
        }
        
        exceptions
    }
}
