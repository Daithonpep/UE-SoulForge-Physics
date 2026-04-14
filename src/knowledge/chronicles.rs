// src/knowledge/chronicles.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    CatastrophicFailure,  // Teoria falló (FS alto -> Colapso)
    Breakthrough,         // Superó récord de estrés
    EngineeringFix,       // Logró sobrevivir tras N fallos
    NewRecord,            // Deformación mínima récord
    ParadigmShift,        // Chrome cambió de opinión drásticamente
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub kind: EventKind,
    pub title: String,
    pub description: String,
    pub engineering_data: String,
    pub takeaway: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SovereignRecords {
    pub max_seismic_survived: f32,
    pub max_wind_survived: f32,
    pub min_deformation_by_type: HashMap<String, f32>,
    pub breakthroughs_count: usize,
}

pub struct ChronicleEngine {
    pub entries: Vec<ChronicleEntry>,
    pub records: SovereignRecords,
    path: String,
    records_path: String,
}

impl ChronicleEngine {
    pub fn new() -> Self {
        // Intentar cargar records previos si existen
        let records = Self::load_records().unwrap_or_default();
        
        Self {
            entries: Vec::new(),
            records,
            path: "sovereign_chronicles.json".to_string(),
            records_path: "sovereign_records.json".to_string(),
        }
    }

    pub fn record_event(&mut self, entry: ChronicleEntry) {
        println!("\n📜 [CRÓNICA] Nuevo hito registrado: {}", entry.title);
        println!("   💡 Aprendizaje: {}", entry.takeaway);
        self.entries.push(entry);
        self.save();
    }

    pub fn check_for_milestone(
        &mut self,
        exp_id: &str,
        senku: &crate::forge::senku_calculator::SenkuAnalysis,
        rapier: &crate::forge::experimental_lab::UnrealSimResult,
        experiment: &crate::forge::experimental_lab::UnrealExperiment,
    ) {
        let mut new_event = None;

        // 1. Detección de Falla Catastrófica (Divergencia Teórica)
        if senku.stability_ratio > 4.0 && !rapier.survived {
            new_event = Some(ChronicleEntry {
                id: exp_id.into(),
                timestamp: Utc::now(),
                kind: EventKind::CatastrophicFailure,
                title: "Colapso Inesperado: El Límite de la Teoría".into(),
                description: format!("Una estructura con FS teórico de {:.2} colapsó totalmente bajo estrés.", senku.stability_ratio),
                engineering_data: format!("FS={:.2}, Rapier_Surv={}", senku.stability_ratio, rapier.survived),
                takeaway: "La estabilidad estática no garantiza la integridad dinámica bajo resonancia.".into(),
            });
        }

        // 2. Detección de Récord de Estrés
        match &experiment.stress_test {
            crate::forge::experimental_lab::StressTest::Seismic(s) => {
                if rapier.survived && s.magnitude > self.records.max_seismic_survived {
                    let old = self.records.max_seismic_survived;
                    self.records.max_seismic_survived = s.magnitude;
                    new_event = Some(ChronicleEntry {
                        id: exp_id.into(),
                        timestamp: Utc::now(),
                        kind: EventKind::Breakthrough,
                        title: "Nueva Frontera Sísmica Alcanzada".into(),
                        description: format!("Hemos superado el récord previo de M{:.2} al resistir M{:.2}.", old, s.magnitude),
                        engineering_data: format!("Nuevo Record: M{:.2}", s.magnitude),
                        takeaway: "La geometría actual demuestra una resiliencia superior a modelos anteriores.".into(),
                    });
                }
            },
            crate::forge::experimental_lab::StressTest::Wind(w) => {
                if rapier.survived && w.speed > self.records.max_wind_survived {
                    let old = self.records.max_wind_survived;
                    self.records.max_wind_survived = w.speed;
                    new_event = Some(ChronicleEntry {
                        id: exp_id.into(),
                        timestamp: Utc::now(),
                        kind: EventKind::Breakthrough,
                        title: "Resistencia Aeólica Récord".into(),
                        description: format!("Superado el límite previo de {:.1}m/s. Nueva marca: {:.1}m/s.", old, w.speed),
                        engineering_data: format!("Viento: {:.1}m/s", w.speed),
                        takeaway: "Aerodinámica optimizada detectada en la configuración actual.".into(),
                    });
                }
            }
        }

        // 3. Detección de Mejora en Deformación
        if rapier.survived {
            let type_key = format!("{:?}", experiment.structure_type).split('(').next().unwrap().to_string();
            let current_best = self.records.min_deformation_by_type.get(&type_key).cloned().unwrap_or(1.0);
            
            if rapier.max_deformation < current_best * 0.9 { // Al menos 10% mejor
                self.records.min_deformation_by_type.insert(type_key.clone(), rapier.max_deformation);
                new_event = Some(ChronicleEntry {
                    id: exp_id.into(),
                    timestamp: Utc::now(),
                    kind: EventKind::NewRecord,
                    title: format!("Optimización Maestra: {}", type_key),
                    description: format!("Mejora del {:.1}% en la deformación máxima estructural.", (1.0 - rapier.max_deformation/current_best) * 100.0),
                    engineering_data: format!("Deformación: {:.6} (Previo: {:.6})", rapier.max_deformation, current_best),
                    takeaway: "La iteración actual ha refinado las tolerancias a un nivel sin precedentes.".into(),
                });
            }
        }

        if let Some(e) = new_event {
            self.records.breakthroughs_count += 1;
            self.record_event(e);
            self.save_records();
        }
    }

    fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.entries) {
            let mut file = File::create(&self.path).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
    }

    fn save_records(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.records) {
            let mut file = File::create(&self.records_path).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
    }

    fn load_records() -> Option<SovereignRecords> {
        let path = "sovereign_records.json";
        if let Ok(content) = std::fs::read_to_string(path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
}
