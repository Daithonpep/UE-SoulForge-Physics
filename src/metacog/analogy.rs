use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sinth-Analog: Sintetizador de Analogías y Razonamiento Transversal
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalogyEngine {
    /// Dominios de transferencia conocidos (UE5, Arquitectura, Electrónica, etc.)
    pub domain_knowledge: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogyMap {
    pub source_concept: String,
    pub target_domain: String,
    pub mappings: Vec<TransferMapping>,
    pub conclusion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferMapping {
    pub attribute: String,       // Almacenamiento, Instrucción, etc.
    pub source_element: String,   // Nucleótidos, Genes
    pub target_element: String,   // DataTables, Scripts
    pub logic: String,            // Por qué coinciden
}

impl AnalogyEngine {
    pub fn new() -> Self {
        let mut domains = HashMap::new();
        
        domains.insert("Unreal Engine 5".to_string(), vec![
            "DataTables".to_string(), "Instanced Static Meshes".to_string(), 
            "Actors".to_string(), "Blueprints".to_string(), 
            "Construction Script".to_string(), "Draw Calls".to_string(), 
            "Shaders".to_string(), "Viewports".to_string(), 
            "Level Sequencer".to_string(), "Components".to_string(), 
            "GameMode".to_string(), "Tick".to_string()
        ]);

        domains.insert("Arquitectura".to_string(), vec![
            "Cimentación".to_string(), "Vigas".to_string(), "Columnas".to_string(), 
            "Arcos".to_string(), "Fachada".to_string(), 
            "Planos".to_string(), "Estructura de carga".to_string(), 
            "Instalaciones".to_string(), "Acabados".to_string()
        ]);

        Self { domain_knowledge: domains }
    }

    /// Generar un mapa de transferencia para un concepto
    pub fn synthesize_analogy(
        &self,
        concept: &str,
        target_domain: &str,
        concept_attributes: &HashMap<String, String>,
    ) -> AnalogyMap {
        let mut mappings = Vec::new();

        for (attr, source_el) in concept_attributes {
            let target_el = self.find_best_match(attr, target_domain);
            
            mappings.push(TransferMapping {
                attribute: attr.clone(),
                source_element: source_el.clone(),
                target_element: target_el.clone(),
                logic: self.generate_transfer_logic(attr, source_el, &target_el),
            });
        }

        let conclusion = if !mappings.is_empty() {
             format!("En esencia, {} opera como un sistema de {} en el dominio de {}.", concept, mappings[0].target_element, target_domain)
        } else {
             format!("No se pudieron establecer puentes lógicos suficientes para {}.", concept)
        };

        AnalogyMap {
            source_concept: concept.to_string(),
            target_domain: target_domain.to_string(),
            mappings,
            conclusion,
        }
    }

    fn find_best_match(&self, attribute: &str, domain: &str) -> String {
        let dc = attribute.to_lowercase();
        
        if domain.contains("Unreal") {
            if dc.contains("almacenamiento") || dc.contains("datos") { return "DataTable / DataAsset".into(); }
            if dc.contains("instrucción") || dc.contains("lógica") { return "Blueprint Graph / C++ Class".into(); }
            if dc.contains("réplica") || dc.contains("copia") { return "Instanced Static Mesh".into(); }
            if dc.contains("estructura") { return "Component Hierarchy".into(); }
            if dc.contains("ejecución") { return "Tick Function / Execution Pin".into(); }
        }

        if domain.contains("Arquitectura") {
            if dc.contains("almacenamiento") { return "Archivos/Biblioteca".into(); }
            if dc.contains("instrucción") { return "Planos Técnicos".into(); }
            if dc.contains("estructura") { return "Esqueleto de Vigas y Columnas".into(); }
            if dc.contains("réplica") { return "Prefabricados".into(); }
        }

        "Elemento genérico".into()
    }

    fn generate_transfer_logic(&self, attr: &str, source: &str, target: &str) -> String {
        format!("Ambos cumplen la función de {} al gestionar y convertir {} en un componente funcional ({}).", attr, source, target)
    }

    /// Formatear la analogía para Daithon
    pub fn format_for_daithon(&self, map: &AnalogyMap) -> String {
        let mut response = format!(
            "¡Kukuku! Me encanta este reto. Para entender **{}** desde la perspectiva de **{}**, debemos mapear su estructura fundamental:\n\n",
            map.source_concept, map.target_domain
        );

        for m in &map.mappings {
            response.push_str(&format!(
                "• **{}** ({}) → En {} equivale a **{}**. {}\n",
                m.attribute, m.source_element, map.target_domain, m.target_element, m.logic
            ));
        }

        response.push_str(&format!(
            "\nConclusión: No estamos ante una simple definición; estamos ante una **SÍNTESIS TRANSVERSAL**. ¿Ves cómo la lógica se transfiere perfectamente? Es fascinante pensar que la escala cambia, pero la arquitectura del sistema es casi idéntica.",
        ));

        response
    }
}
