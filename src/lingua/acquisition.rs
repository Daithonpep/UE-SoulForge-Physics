//! LINGUA — Fase 1: Adquisición de Conocimiento Lingüístico
//! 
//! Carga vocabulario desde fuentes externas (Wiktionary API) y locales.
//! Opera en modo dual: online (API) y offline (caché JSON local).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ────────────────────────────────────────────────────────────────
//  TIPOS DE DATOS
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Gender { Masculine, Feminine, Neutral }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WordNumber { Singular, Plural }

/// Entrada completa de una palabra adquirida
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquiredWord {
    pub word: String,
    pub language: String,
    pub part_of_speech: Vec<String>,
    pub meanings: Vec<String>,
    pub examples: Vec<String>,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
    pub related_terms: Vec<String>,
    pub gender: Gender,
    pub number: WordNumber,
    /// Definición estructural (geometría de bajo nivel: plano, cilindro, etc)
    pub structural_features: Vec<String>,
    /// Categoría semántica inferida para diseño
    pub design_category: Option<DesignCategory>,
    /// ¿Se ha asociado este concepto con una forma 3D o experiencia física?
    pub is_visually_grounded: bool,
    /// ID del concepto cargado en el motor ARCHETYPE si está anclado
    pub grounded_concept_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DesignCategory {
    Furniture,
    Material,
    Shape,
    Dimension,
    Style,
    Function,
    Property,
    Finish,
    Color,
    Connector,    // con, sin, para, de
    DesignVerb,   // diseñar, crear, generar
    Quantifier,   // uno, dos, varios
    Uncategorized,
}

// ────────────────────────────────────────────────────────────────
//  MOTOR DE ADQUISICIÓN
// ────────────────────────────────────────────────────────────────

/// Motor principal de adquisición de vocabulario
pub struct AcquisitionEngine {
    vocabulary: HashMap<String, AcquiredWord>,
    cache_path: String,
}

impl AcquisitionEngine {
    pub fn new() -> Self {
        Self {
            vocabulary: HashMap::new(),
            cache_path: "lingua_cache".to_string(),
        }
    }

    /// Inicializar: cargar caché local o construir vocabulario base
    pub fn initialize(&mut self) -> usize {
        let cache_file = format!("{}/vocabulary.json", self.cache_path);
        
        if let Ok(content) = std::fs::read_to_string(&cache_file) {
            if let Ok(cached) = serde_json::from_str::<HashMap<String, AcquiredWord>>(&content) {
                let count = cached.len();
                self.vocabulary = cached;
                log::info!("[LINGUA-ACQ] Vocabulario cargado desde caché: {} palabras", count);
                return count;
            }
        }

        // Sin caché → construir vocabulario base embebido
        self.build_embedded_vocabulary();
        let count = self.vocabulary.len();
        log::info!("[LINGUA-ACQ] Vocabulario embebido construido: {} palabras", count);
        
        // Guardar caché
        let _ = self.save_cache();
        count
    }

    /// Fetch asíncrono desde Wiktionary API
    pub async fn fetch_from_wiktionary(&mut self, word: &str) -> Result<AcquiredWord, String> {
        let url = format!(
            "https://en.wiktionary.org/api/rest_v1/page/definition/{}",
            word
        );
        
        let response = reqwest::get(&url).await
            .map_err(|e| format!("HTTP error: {}", e))?;
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| format!("JSON parse error: {}", e))?;
        
        let mut meanings = Vec::new();
        let mut pos_tags = Vec::new();
        
        // Intentar parsear sección española o inglesa
        for lang_key in &["es", "en"] {
            if let Some(defs) = json.get(lang_key).and_then(|v| v.as_array()) {
                for def_block in defs {
                    if let Some(pos) = def_block.get("partOfSpeech").and_then(|v| v.as_str()) {
                        pos_tags.push(pos.to_string());
                    }
                    if let Some(def_list) = def_block.get("definitions").and_then(|v| v.as_array()) {
                        for d in def_list {
                            if let Some(meaning) = d.get("definition").and_then(|v| v.as_str()) {
                                meanings.push(meaning.to_string());
                            }
                        }
                    }
                }
                if !meanings.is_empty() { break; }
            }
        }

        let category = Self::infer_design_category(word, &pos_tags);

        let mut gender = Gender::Masculine;
        let mut number = WordNumber::Singular;
        let w = word.to_lowercase();
        if w.ends_with('a') || w.ends_with("as") { gender = Gender::Feminine; }
        if w.ends_with('s') && !w.ends_with("es") || (w.ends_with("es") && w.len() > 3) { number = WordNumber::Plural; }

        let entry = AcquiredWord {
            word: word.to_string(),
            language: "es".to_string(),
            part_of_speech: pos_tags,
            meanings,
            examples: vec![],
            synonyms: vec![],
            antonyms: vec![],
            related_terms: vec![],
            gender,
            number,
            structural_features: vec![],
            design_category: Some(category),
            is_visually_grounded: false,
            grounded_concept_id: None,
        };

        self.vocabulary.insert(word.to_string(), entry.clone());
        Ok(entry)
    }

    /// Obtener una palabra aleatoria de Wiktionary e integrarla
    pub async fn fetch_random_wiktionary(&mut self) -> Result<AcquiredWord, String> {
        let url = "https://es.wiktionary.org/api/rest_v1/page/random/summary";
        let response = reqwest::get(url).await
            .map_err(|e| format!("HTTP error: {}", e))?;
            
        let json: serde_json::Value = response.json().await
            .map_err(|e| format!("JSON parse error: {}", e))?;
            
        if let Some(title) = json.get("title").and_then(|v| v.as_str()) {
            let normalized_title = title.replace(" ", "_");
            return self.fetch_from_wiktionary(&normalized_title).await;
        }

        Err("No se pudo obtener un título aleatorio".to_string())
    }

    /// Fetch batch con rate limiting
    pub async fn fetch_batch(&mut self, words: &[&str]) -> Vec<Result<AcquiredWord, String>> {
        let mut results = Vec::new();
        
        for (i, word) in words.iter().enumerate() {
            if self.vocabulary.contains_key(*word) {
                results.push(Ok(self.vocabulary[*word].clone()));
                continue;
            }
            
            let result = self.fetch_from_wiktionary(word).await;
            results.push(result);
            
            // Rate limiting: 100ms entre requests
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            if (i + 1) % 50 == 0 {
                log::info!("[LINGUA-ACQ] Progreso: {}/{} palabras", i + 1, words.len());
            }
        }
        
        let _ = self.save_cache();
        results
    }

    /// Construir vocabulario base embebido (sin red)
    fn build_embedded_vocabulary(&mut self) {
        let word_defs: Vec<(&str, &[&str], DesignCategory, &[&str])> = vec![
            ("mesa",         &["noun"],      DesignCategory::Furniture,  &["Mueble con superficie plana horizontal sostenida por patas"]),
            ("silla",        &["noun"],      DesignCategory::Furniture,  &["Asiento con respaldo para una persona"]),
            ("sofá",         &["noun"],      DesignCategory::Furniture,  &["Asiento largo y acolchado para varias personas"]),
            ("puente",       &["noun"],      DesignCategory::Furniture,  &["Estructura que permite salvar un obstáculo físico"]),
            ("columna",      &["noun"],      DesignCategory::Furniture,  &["Elemento de soporte vertical y alargado"]),
            ("casa",         &["noun"],      DesignCategory::Furniture,  &["Edificación destinada a ser habitada"]),
            ("torre",        &["noun"],      DesignCategory::Furniture,  &["Edificio mucho más alto que ancho"]),
            ("muro",         &["noun"],      DesignCategory::Furniture,  &["Pared gruesa de piedra o albañilería"]),
            ("armario",      &["noun"],      DesignCategory::Furniture,  &["Mueble con puertas usado para almacenar"]),
            ("escritorio",   &["noun"],      DesignCategory::Furniture,  &["Mesa diseñada para trabajo de oficina"]),
            ("estantería",   &["noun"],      DesignCategory::Furniture,  &["Mueble con estantes para almacenar objetos"]),
            ("cama",         &["noun"],      DesignCategory::Furniture,  &["Mueble para dormir"]),
            ("taburete",     &["noun"],      DesignCategory::Furniture,  &["Asiento sin respaldo"]),
            ("banco",        &["noun"],      DesignCategory::Furniture,  &["Asiento largo para varias personas, sin respaldo acolchado"]),
            ("aparador",     &["noun"],      DesignCategory::Furniture,  &["Mueble bajo usado para servir y almacenar vajilla"]),
            ("mueble",       &["noun"],      DesignCategory::Furniture,  &["Objeto móvil que equipa un espacio habitable"]),
            ("pata",         &["noun"],      DesignCategory::Furniture,  &["Soporte vertical de un mueble"]),
            ("respaldo",     &["noun"],      DesignCategory::Furniture,  &["Parte de un asiento que soporta la espalda"]),
            ("superficie",   &["noun"],      DesignCategory::Furniture,  &["Cara exterior de un objeto; parte plana superior de una mesa"]),
            ("cajón",        &["noun"],      DesignCategory::Furniture,  &["Compartimento deslizable dentro de un mueble"]),
            // ─── MATERIALES ───
            ("madera",       &["noun"],      DesignCategory::Material,   &["Material orgánico extraído de los árboles"]),
            ("acero",        &["noun"],      DesignCategory::Material,   &["Aleación de hierro y carbono, resistente"]),
            ("vidrio",       &["noun"],      DesignCategory::Material,   &["Material transparente y frágil"]),
            ("piedra",       &["noun"],      DesignCategory::Material,   &["Material mineral sólido y duro"]),
            ("metal",        &["noun"],      DesignCategory::Material,   &["Elemento químico conductor, generalmente sólido"]),
            ("plástico",     &["noun"],      DesignCategory::Material,   &["Material sintético moldeable"]),
            ("cuero",        &["noun"],      DesignCategory::Material,   &["Piel animal curtida"]),
            ("mármol",       &["noun"],      DesignCategory::Material,   &["Piedra caliza metamórfica, pulible y decorativa"]),
            ("concreto",     &["noun"],      DesignCategory::Material,   &["Mezcla de cemento, agua y agregados"]),
            ("aluminio",     &["noun"],      DesignCategory::Material,   &["Metal ligero y resistente a la corrosión"]),
            // ─── FORMAS ───
            ("rectangular",  &["adjective"], DesignCategory::Shape,      &["Con forma de rectángulo"]),
            ("cuadrado",     &["adjective"], DesignCategory::Shape,      &["Con cuatro lados iguales y ángulos rectos"]),
            ("circular",     &["adjective"], DesignCategory::Shape,      &["Con forma de círculo"]),
            ("ovalado",      &["adjective"], DesignCategory::Shape,      &["Con forma de óvalo"]),
            ("curvo",        &["adjective"], DesignCategory::Shape,      &["Que tiene curvas, no recto"]),
            ("recto",        &["adjective"], DesignCategory::Shape,      &["Sin curvas ni desviaciones"]),
            ("angular",      &["adjective"], DesignCategory::Shape,      &["Con ángulos marcados"]),
            ("orgánico",     &["adjective"], DesignCategory::Shape,      &["Con formas naturales e irregulares"]),
            ("geométrico",   &["adjective"], DesignCategory::Shape,      &["Con formas regulares y matemáticas"]),
            // ─── DIMENSIONES ───
            ("alto",         &["adjective"], DesignCategory::Dimension,  &["De gran altura"]),
            ("bajo",         &["adjective"], DesignCategory::Dimension,  &["De poca altura"]),
            ("ancho",        &["adjective"], DesignCategory::Dimension,  &["De gran anchura"]),
            ("estrecho",     &["adjective"], DesignCategory::Dimension,  &["De poca anchura"]),
            ("profundo",     &["adjective"], DesignCategory::Dimension,  &["De gran profundidad"]),
            ("delgado",      &["adjective"], DesignCategory::Dimension,  &["De poco grosor"]),
            ("grueso",       &["adjective"], DesignCategory::Dimension,  &["De gran grosor"]),
            ("grande",       &["adjective"], DesignCategory::Dimension,  &["De gran tamaño"]),
            ("pequeño",      &["adjective"], DesignCategory::Dimension,  &["De poco tamaño"]),
            // ─── ESTILOS ───
            ("minimalista",     &["adjective"], DesignCategory::Style,   &["Estilo con mínimos elementos decorativos"]),
            ("moderno",         &["adjective"], DesignCategory::Style,   &["De la época actual; con estilo contemporáneo"]),
            ("clásico",         &["adjective"], DesignCategory::Style,   &["De estilo tradicional y atemporal"]),
            ("industrial",      &["adjective"], DesignCategory::Style,   &["Estilo que expone materiales crudos y estructuras"]),
            ("rústico",         &["adjective"], DesignCategory::Style,   &["De estilo rural, natural y sin refinar"]),
            ("contemporáneo",   &["adjective"], DesignCategory::Style,   &["Perteneciente al diseño del presente"]),
            ("escandinavo",     &["adjective"], DesignCategory::Style,   &["Estilo nórdico: líneas simples, maderas claras"]),
            ("barroco",         &["adjective"], DesignCategory::Style,   &["Estilo ornamentado y recargado del siglo XVII"]),
            // ─── FUNCIONES ───
            ("sentarse",     &["verb"],      DesignCategory::Function,   &["Apoyar el cuerpo sobre un asiento"]),
            ("apoyar",       &["verb"],      DesignCategory::Function,   &["Sostener algo sobre una superficie"]),
            ("almacenar",    &["verb"],      DesignCategory::Function,   &["Guardar objetos en un espacio"]),
            ("sostener",     &["verb"],      DesignCategory::Function,   &["Mantener algo en una posición"]),
            ("contener",     &["verb"],      DesignCategory::Function,   &["Guardar algo dentro de un espacio cerrado"]),
            ("exhibir",      &["verb"],      DesignCategory::Function,   &["Mostrar algo de manera visible"]),
            // ─── PROPIEDADES ───
            ("estable",      &["adjective"], DesignCategory::Property,   &["Que no se cae ni se mueve fácilmente"]),
            ("resistente",   &["adjective"], DesignCategory::Property,   &["Que soporta fuerzas sin romperse"]),
            ("duradero",     &["adjective"], DesignCategory::Property,   &["Que dura mucho tiempo"]),
            ("cómodo",       &["adjective"], DesignCategory::Property,   &["Que proporciona confort"]),
            ("ergonómico",   &["adjective"], DesignCategory::Property,   &["Diseñado para la comodidad del cuerpo humano"]),
            ("ligero",       &["adjective"], DesignCategory::Property,   &["De poco peso"]),
            ("pesado",       &["adjective"], DesignCategory::Property,   &["De mucho peso"]),
            ("flexible",     &["adjective"], DesignCategory::Property,   &["Que se dobla sin romperse"]),
            ("rígido",       &["adjective"], DesignCategory::Property,   &["Que no se dobla"]),
            // ─── ACABADOS ───
            ("pulido",       &["adjective"], DesignCategory::Finish,     &["Con superficie lisa y brillante"]),
            ("mate",         &["adjective"], DesignCategory::Finish,     &["Sin brillo"]),
            ("brillante",    &["adjective"], DesignCategory::Finish,     &["Que refleja la luz"]),
            ("texturizado",  &["adjective"], DesignCategory::Finish,     &["Con textura perceptible al tacto"]),
            ("liso",         &["adjective"], DesignCategory::Finish,     &["Sin rugosidades"]),
            // ─── COLORES ───
            ("blanco",       &["adjective"], DesignCategory::Color,      &["Color de la luz completa"]),
            ("negro",        &["adjective"], DesignCategory::Color,      &["Ausencia de color, absorbe toda la luz"]),
            ("gris",         &["adjective"], DesignCategory::Color,      &["Color intermedio entre blanco y negro"]),
            ("marrón",       &["adjective"], DesignCategory::Color,      &["Color de la tierra o la madera"]),
            ("rojo",         &["adjective"], DesignCategory::Color,      &["Color primario cálido"]),
            ("azul",         &["adjective"], DesignCategory::Color,      &["Color primario frío"]),
            ("verde",        &["adjective"], DesignCategory::Color,      &["Color de la vegetación"]),
            // ─── CONECTORES ───
            ("con",          &["preposition"], DesignCategory::Connector, &["Indica compañía o instrumento"]),
            ("sin",          &["preposition"], DesignCategory::Connector, &["Indica ausencia"]),
            ("para",         &["preposition"], DesignCategory::Connector, &["Indica finalidad"]),
            ("de",           &["preposition"], DesignCategory::Connector, &["Indica procedencia, material o pertenencia"]),
            ("en",           &["preposition"], DesignCategory::Connector, &["Indica ubicación"]),
            ("sobre",        &["preposition"], DesignCategory::Connector, &["Indica posición superior"]),
            ("y",            &["conjunction"],  DesignCategory::Connector, &["Une elementos"]),
            ("o",            &["conjunction"],  DesignCategory::Connector, &["Indica alternativa"]),
            // ─── VERBOS DE DISEÑO ───
            ("diseñar",      &["verb"],      DesignCategory::DesignVerb, &["Crear un plan para un objeto"]),
            ("crear",        &["verb"],      DesignCategory::DesignVerb, &["Producir algo nuevo"]),
            ("modificar",    &["verb"],      DesignCategory::DesignVerb, &["Cambiar parcialmente algo existente"]),
            ("ajustar",      &["verb"],      DesignCategory::DesignVerb, &["Cambiar una propiedad con precisión"]),
            ("generar",      &["verb"],      DesignCategory::DesignVerb, &["Producir automáticamente"]),
            ("optimizar",    &["verb"],      DesignCategory::DesignVerb, &["Mejorar al máximo"]),
            ("validar",      &["verb"],      DesignCategory::DesignVerb, &["Comprobar que es correcto"]),
            // ─── CUANTIFICADORES ───
            ("uno",          &["numeral"],   DesignCategory::Quantifier, &["Número 1"]),
            ("dos",          &["numeral"],   DesignCategory::Quantifier, &["Número 2"]),
            ("tres",         &["numeral"],   DesignCategory::Quantifier, &["Número 3"]),
            ("cuatro",       &["numeral"],   DesignCategory::Quantifier, &["Número 4"]),
            ("cinco",        &["numeral"],   DesignCategory::Quantifier, &["Número 5"]),
            ("seis",         &["numeral"],   DesignCategory::Quantifier, &["Número 6"]),
            ("varios",       &["adjective"], DesignCategory::Quantifier, &["Más de dos, indeterminado"]),
            ("muchos",       &["adjective"], DesignCategory::Quantifier, &["Gran cantidad"]),
            // ─── GRAMÁTICA FUNCIONAL (sin forma 3D) ───
            ("hola",         &["interjection"], DesignCategory::Uncategorized, &["Saludo"]),
            ("adiós",        &["interjection"], DesignCategory::Uncategorized, &["Despedida"]),
            ("por qué",      &["adverb"],       DesignCategory::Uncategorized, &["Pregunta causal"]),
            ("cómo",         &["adverb"],       DesignCategory::Uncategorized, &["Pregunta de modo"]),
            ("qué",          &["pronoun"],      DesignCategory::Uncategorized, &["Pregunta sobre identidad"]),
            ("el",           &["determiner"],   DesignCategory::Connector,     &["Artículo definido masculino"]),
            ("la",           &["determiner"],   DesignCategory::Connector,     &["Artículo definido femenino"]),
            ("un",           &["determiner"],   DesignCategory::Connector,     &["Artículo indefinido masculino"]),
            ("una",          &["determiner"],   DesignCategory::Connector,     &["Artículo indefinido femenino"]),
            ("es",           &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular del verbo ser"]),
            ("no",           &["adverb"],       DesignCategory::Uncategorized, &["Negación"]),
            ("sí",           &["adverb"],       DesignCategory::Uncategorized, &["Afirmación"]),
            ("más",          &["adverb"],       DesignCategory::Uncategorized, &["Comparativo de superioridad"]),
            ("menos",        &["adverb"],       DesignCategory::Uncategorized, &["Comparativo de inferioridad"]),
            ("muy",          &["adverb"],       DesignCategory::Uncategorized, &["Superlativo"]),
            ("bien",         &["adverb"],       DesignCategory::Uncategorized, &["Correctamente"]),
            ("mal",          &["adverb"],       DesignCategory::Uncategorized, &["Incorrectamente"]),
            ("gracias",      &["interjection"], DesignCategory::Uncategorized, &["Expresión de agradecimiento"]),
            // ─── PRONOMBRES PERSONALES ───
            ("yo",           &["pronoun"],      DesignCategory::Uncategorized, &["Primera persona singular, el hablante"]),
            ("tú",           &["pronoun"],      DesignCategory::Uncategorized, &["Segunda persona singular, el oyente"]),
            ("él",           &["pronoun"],      DesignCategory::Uncategorized, &["Tercera persona singular masculina"]),
            ("ella",         &["pronoun"],      DesignCategory::Uncategorized, &["Tercera persona singular femenina"]),
            ("nosotros",     &["pronoun"],      DesignCategory::Uncategorized, &["Primera persona plural"]),
            ("ellos",        &["pronoun"],      DesignCategory::Uncategorized, &["Tercera persona plural masculina"]),
            ("me",           &["pronoun"],      DesignCategory::Uncategorized, &["Pronombre reflexivo de primera persona"]),
            ("te",           &["pronoun"],      DesignCategory::Uncategorized, &["Pronombre reflexivo de segunda persona"]),
            ("se",           &["pronoun"],      DesignCategory::Uncategorized, &["Pronombre reflexivo de tercera persona"]),
            ("lo",           &["pronoun"],      DesignCategory::Uncategorized, &["Pronombre de objeto directo masculino"]),
            ("le",           &["pronoun"],      DesignCategory::Uncategorized, &["Pronombre de objeto indirecto"]),
            ("nos",          &["pronoun"],      DesignCategory::Uncategorized, &["Pronombre reflexivo de primera persona plural"]),
            ("mi",           &["determiner"],   DesignCategory::Uncategorized, &["Posesivo de primera persona"]),
            ("tu",           &["determiner"],   DesignCategory::Uncategorized, &["Posesivo de segunda persona"]),
            ("su",           &["determiner"],   DesignCategory::Uncategorized, &["Posesivo de tercera persona"]),
            ("ese",          &["pronoun"],      DesignCategory::Uncategorized, &["Demostrativo mediano masculino"]),
            ("eso",          &["pronoun"],      DesignCategory::Uncategorized, &["Demostrativo neutro"]),
            ("esto",         &["pronoun"],      DesignCategory::Uncategorized, &["Demostrativo cercano neutro"]),
            ("este",         &["pronoun"],      DesignCategory::Uncategorized, &["Demostrativo cercano masculino"]),
            ("esta",         &["pronoun"],      DesignCategory::Uncategorized, &["Demostrativo cercano femenino"]),
            ("algo",         &["pronoun"],      DesignCategory::Uncategorized, &["Cosa indeterminada"]),
            ("nada",         &["pronoun"],      DesignCategory::Uncategorized, &["Ninguna cosa"]),
            ("todo",         &["pronoun"],      DesignCategory::Uncategorized, &["La totalidad de algo"]),
            ("quien",        &["pronoun"],      DesignCategory::Uncategorized, &["Pregunta sobre persona"]),
            ("cual",         &["pronoun"],      DesignCategory::Uncategorized, &["Pregunta de selección"]),
            // ─── VERBOS BÁSICOS (infinitivos + conjugaciones comunes) ───
            ("ser",          &["verb"],         DesignCategory::Uncategorized, &["Existir, tener una cualidad permanente"]),
            ("soy",          &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de ser"]),
            ("eres",         &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de ser"]),
            ("somos",        &["verb"],         DesignCategory::Uncategorized, &["Primera persona plural de ser"]),
            ("estar",        &["verb"],         DesignCategory::Uncategorized, &["Encontrarse en un estado o lugar temporal"]),
            ("estoy",        &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de estar"]),
            ("estás",        &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de estar"]),
            ("está",         &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de estar"]),
            ("estaba",       &["verb"],         DesignCategory::Uncategorized, &["Pretérito imperfecto de estar"]),
            ("estado",       &["verb"],         DesignCategory::Uncategorized, &["Participio pasado de estar"]),
            ("estamos",      &["verb"],         DesignCategory::Uncategorized, &["Primera persona plural de estar"]),
            ("hacer",        &["verb"],         DesignCategory::Uncategorized, &["Realizar una acción o crear algo"]),
            ("hago",         &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de hacer"]),
            ("haces",        &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de hacer"]),
            ("hace",         &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de hacer"]),
            ("haciendo",     &["verb"],         DesignCategory::Uncategorized, &["Gerundio de hacer, acción en curso"]),
            ("has",          &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de haber"]),
            ("he",           &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de haber"]),
            ("tener",        &["verb"],         DesignCategory::Uncategorized, &["Poseer algo"]),
            ("tengo",        &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de tener"]),
            ("tienes",       &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de tener"]),
            ("tiene",        &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de tener"]),
            ("poder",        &["verb"],         DesignCategory::Uncategorized, &["Tener la capacidad de hacer algo"]),
            ("puedo",        &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de poder"]),
            ("puedes",       &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de poder"]),
            ("puede",        &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de poder"]),
            ("querer",       &["verb"],         DesignCategory::Uncategorized, &["Desear algo"]),
            ("quiero",       &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de querer"]),
            ("quieres",      &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de querer"]),
            ("ir",           &["verb"],         DesignCategory::Uncategorized, &["Moverse hacia un destino"]),
            ("voy",          &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de ir"]),
            ("vas",          &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de ir"]),
            ("va",           &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de ir"]),
            ("decir",        &["verb"],         DesignCategory::Uncategorized, &["Comunicar con palabras"]),
            ("digo",         &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de decir"]),
            ("dices",        &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de decir"]),
            ("dice",         &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de decir"]),
            ("saber",        &["verb"],         DesignCategory::Uncategorized, &["Tener conocimiento de algo"]),
            ("sé",           &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de saber"]),
            ("sabes",        &["verb"],         DesignCategory::Uncategorized, &["Segunda persona singular de saber"]),
            ("sabe",         &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de saber"]),
            ("ver",          &["verb"],         DesignCategory::Uncategorized, &["Percibir con los ojos"]),
            ("veo",          &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de ver"]),
            ("dar",          &["verb"],         DesignCategory::Uncategorized, &["Entregar algo a alguien"]),
            ("doy",          &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de dar"]),
            ("hablar",       &["verb"],         DesignCategory::Uncategorized, &["Comunicarse usando el lenguaje"]),
            ("hablo",        &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de hablar"]),
            ("entender",     &["verb"],         DesignCategory::Uncategorized, &["Comprender el significado de algo"]),
            ("entiendo",     &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de entender"]),
            ("significa",    &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de significar"]),
            ("significar",   &["verb"],         DesignCategory::Uncategorized, &["Tener un sentido o valor semántico"]),
            ("aprender",     &["verb"],         DesignCategory::Uncategorized, &["Adquirir conocimiento nuevo"]),
            ("aprendo",      &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de aprender"]),
            ("conocer",      &["verb"],         DesignCategory::Uncategorized, &["Tener experiencia o relación con algo"]),
            ("conozco",      &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de conocer"]),
            ("necesitar",    &["verb"],         DesignCategory::Uncategorized, &["Requerir algo"]),
            ("necesito",     &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de necesitar"]),
            ("pensar",       &["verb"],         DesignCategory::Uncategorized, &["Usar la mente para razonar"]),
            ("pienso",       &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de pensar"]),
            ("creer",        &["verb"],         DesignCategory::Uncategorized, &["Tener confianza en algo"]),
            ("creo",         &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de creer"]),
            ("gustar",       &["verb"],         DesignCategory::Uncategorized, &["Producir agrado"]),
            ("gusta",        &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de gustar"]),
            ("parecer",      &["verb"],         DesignCategory::Uncategorized, &["Tener apariencia de algo"]),
            ("parece",       &["verb"],         DesignCategory::Uncategorized, &["Tercera persona singular de parecer"]),
            ("llamar",       &["verb"],         DesignCategory::Uncategorized, &["Nombrar o invocar"]),
            ("llamo",        &["verb"],         DesignCategory::Uncategorized, &["Primera persona singular de llamar"]),
            ("tal",           &["adjective"],    DesignCategory::Uncategorized, &["Indica semejanza o cualidad imprecisa"]),
            ("bien",          &["adverb"],       DesignCategory::Uncategorized, &["De manera correcta o satisfactoria"]),
            ("mal",           &["adverb"],       DesignCategory::Uncategorized, &["De manera incorrecta"]),
            ("muy",           &["adverb"],       DesignCategory::Uncategorized, &["Indica grado elevado"]),
            ("bastante",      &["adverb"],       DesignCategory::Uncategorized, &["En cantidad o grado suficiente"]),
            ("poco",          &["adverb"],       DesignCategory::Uncategorized, &["En cantidad o grado escaso"]),
            ("mucho",         &["adverb"],       DesignCategory::Uncategorized, &["En gran cantidad"]),
            ("cada",          &["determiner"],   DesignCategory::Uncategorized, &["Indica todos los elementos de una serie"]),
            ("todo",          &["pronoun"],      DesignCategory::Uncategorized, &["La totalidad de algo"]),
            ("nada",          &["pronoun"],      DesignCategory::Uncategorized, &["Ninguna cosa"]),
            ("algo",          &["pronoun"],      DesignCategory::Uncategorized, &["Alguna cosa"]),
            ("alguien",       &["pronoun"],      DesignCategory::Uncategorized, &["Alguna persona"]),
            ("nadie",         &["pronoun"],      DesignCategory::Uncategorized, &["Ninguna persona"]),
            ("ninguno",       &["adjective"],    DesignCategory::Uncategorized, &["Ni uno solo"]),
            ("alguno",        &["adjective"],    DesignCategory::Uncategorized, &["Uno o varios"]),
            ("siempre",       &["adverb"],       DesignCategory::Uncategorized, &["En todo momento"]),
            ("nunca",         &["adverb"],       DesignCategory::Uncategorized, &["En ningún momento"]),
            ("jamás",         &["adverb"],       DesignCategory::Uncategorized, &["Nunca"]),
            ("quizás",        &["adverb"],       DesignCategory::Uncategorized, &["Tal vez, indica posibilidad"]),
            ("tal vez",       &["adverb"],       DesignCategory::Uncategorized, &["Indica duda o posibilidad"]),
            ("hoy",           &["adverb"],       DesignCategory::Uncategorized, &["En el día presente"]),
            ("ayer",          &["adverb"],       DesignCategory::Uncategorized, &["En el día anterior"]),
            ("mañana",        &["adverb"],       DesignCategory::Uncategorized, &["En el día siguiente"]),
            ("antes",         &["adverb"],       DesignCategory::Uncategorized, &["En tiempo anterior"]),
            ("después",       &["adverb"],       DesignCategory::Uncategorized, &["En tiempo posterior"]),
            ("pronto",        &["adverb"],       DesignCategory::Uncategorized, &["En poco tiempo"]),
            ("tarde",         &["adverb"],       DesignCategory::Uncategorized, &["Después del tiempo oportuno"]),
            ("aquí",          &["adverb"],       DesignCategory::Uncategorized, &["En este lugar"]),
            ("allí",          &["adverb"],       DesignCategory::Uncategorized, &["En aquel lugar"]),
            ("donde",         &["adverb"],       DesignCategory::Uncategorized, &["En el lugar en que"]),
            ("cuando",        &["conjunction"],  DesignCategory::Connector,     &["En el tiempo en que"]),
            ("mientras",      &["conjunction"],  DesignCategory::Connector,     &["Al mismo tiempo"]),
            ("aunque",        &["conjunction"],  DesignCategory::Connector,     &["Indica concesión"]),
            ("porque",        &["conjunction"],  DesignCategory::Connector,     &["Indica causa"]),
            ("pues",          &["conjunction"],  DesignCategory::Connector,     &["Indica consecuencia o motivo"]),
            ("claro",         &["adjective"],    DesignCategory::Uncategorized, &["Evidente o luminoso"]),
            ("perfecto",      &["adjective"],    DesignCategory::Uncategorized, &["Que tiene todas las cualidades"]),
            ("seguro",        &["adjective"],    DesignCategory::Uncategorized, &["Libre de peligro o cierto"]),
            ("cierto",        &["adjective"],    DesignCategory::Uncategorized, &["Verdadero"]),
            ("falso",         &["adjective"],    DesignCategory::Uncategorized, &["No verdadero"]),
            ("posible",       &["adjective"],    DesignCategory::Uncategorized, &["Que puede ser"]),
            ("imposible",     &["adjective"],    DesignCategory::Uncategorized, &["Que no puede ser"]),
            ("necesario",     &["adjective"],    DesignCategory::Uncategorized, &["Que hace falta obligatoriamente"]),
            ("interesante",   &["adjective"],    DesignCategory::Uncategorized, &["Que atrae la atención"]),
            ("importante",    &["adjective"],    DesignCategory::Uncategorized, &["De mucho valor"]),
            ("difícil",       &["adjective"],    DesignCategory::Uncategorized, &["Que requiere esfuerzo"]),
            ("fácil",         &["adjective"],    DesignCategory::Uncategorized, &["Que no requiere esfuerzo"]),
            ("grande",        &["adjective"],    DesignCategory::Uncategorized, &["De tamaño superior"]),
            ("pequeño",       &["adjective"],    DesignCategory::Uncategorized, &["De tamaño inferior"]),
            ("nuevo",         &["adjective"],    DesignCategory::Uncategorized, &["Recién hecho o descubierto"]),
            ("viejo",         &["adjective"],    DesignCategory::Uncategorized, &["Que tiene muchos años"]),
            ("primero",       &["adjective"],    DesignCategory::Uncategorized, &["Que precede a los demás"]),
            ("último",        &["adjective"],    DesignCategory::Uncategorized, &["Que no tiene nada después"]),
            ("mismo",         &["adjective"],    DesignCategory::Uncategorized, &["Idéntico"]),
            ("otro",          &["adjective"],    DesignCategory::Uncategorized, &["Diferente"]),
            ("nuestro",       &["determiner"],   DesignCategory::Uncategorized, &["Poseído por nosotros"]),
            ("vuestro",       &["determiner"],   DesignCategory::Uncategorized, &["Poseído por vosotros"]),
            ("mío",           &["determiner"],   DesignCategory::Uncategorized, &["Poseído por mí"]),
            ("tuyo",          &["determiner"],   DesignCategory::Uncategorized, &["Poseído por ti"]),
            ("suyo",          &["determiner"],   DesignCategory::Uncategorized, &["Poseído por él/ella/ellos"]),
            ("quién",         &["pronoun"],      DesignCategory::Uncategorized, &["Interrogativo de persona"]),
            ("cuál",          &["pronoun"],      DesignCategory::Uncategorized, &["Interrogativo de elección"]),
            ("cuánto",        &["adverb"],       DesignCategory::Uncategorized, &["Interrogativo de cantidad"]),
            ("dónde",         &["adverb"],       DesignCategory::Uncategorized, &["Interrogativo de lugar"]),
            ("cómo",          &["adverb"],       DesignCategory::Uncategorized, &["Interrogativo de modo"]),
            ("por qué",       &["adverb"],       DesignCategory::Uncategorized, &["Interrogativo de causa"]),
            ("verdad",        &["noun"],         DesignCategory::Uncategorized, &["Realidad de las cosas"]),
            ("ejemplo",       &["noun"],         DesignCategory::Uncategorized, &["Caso ilustrativo"]),
            ("tipo",          &["noun"],         DesignCategory::Uncategorized, &["Clase o categoría"]),
            ("forma",         &["noun"],         DesignCategory::Uncategorized, &["Configuración externa"]),
            ("manera",        &["noun"],         DesignCategory::Uncategorized, &["Modo de hacer"]),
            ("parte",         &["noun"],         DesignCategory::Uncategorized, &["Porción de un todo"]),
            ("tiempo",        &["noun"],         DesignCategory::Uncategorized, &["Duración temporal"]),
            ("mundo",         &["noun"],         DesignCategory::Uncategorized, &["Conjunto de todo lo que existe"]),
            ("vida",          &["noun"],         DesignCategory::Uncategorized, &["Existencia de los seres"]),
            ("trabajo",       &["noun"],         DesignCategory::Uncategorized, &["Actividad productiva"]),
            ("mano",          &["noun"],         DesignCategory::Uncategorized, &["Parte del cuerpo, instrumento de creación"]),
            ("palabra",       &["noun"],         DesignCategory::Uncategorized, &["Unidad de lenguaje"]),
            ("lenguaje",      &["noun"],         DesignCategory::Uncategorized, &["Sistema de comunicación"]),
            ("historia",      &["noun"],         DesignCategory::Uncategorized, &["Relato de sucesos"]),
            ("intelectualmente", &["adverb"],    DesignCategory::Uncategorized, &["Desde un punto de vista intelectual"]),
            ("hoy",           &["adverb"],       DesignCategory::Uncategorized, &["Día en curso"]),
        ];

        for (word, pos, category, meanings) in word_defs {
            let w = word.to_lowercase();
            let mut gender = Gender::Masculine;
            let mut number = WordNumber::Singular;

            // Inferencia automática de género/número para el seed
            if w.ends_with('a') || w.ends_with("as") { gender = Gender::Feminine; }
            if w.ends_with('s') && !w.ends_with("es") || (w.ends_with("es") && w.len() > 3) { number = WordNumber::Plural; }
            if pos.contains(&"verb") { gender = Gender::Neutral; }

            // Excepciones manuales rápidas para el seed
            if w == "mano" { gender = Gender::Feminine; }
            if w == "el" || w == "un" || w == "él" { gender = Gender::Masculine; }
            if w == "la" || w == "una" || w == "ella" { gender = Gender::Feminine; }

            // Definiciones estructurales específicas para el seed
            let structural_features = match w.as_str() {
                "mesa" => vec!["plano horizontal".to_string(), "soportes verticales".to_string(), "superficie rígida".to_string()],
                "silla" => vec!["asiento".to_string(), "respaldo".to_string(), "cuatro apoyos".to_string()],
                "sofá" => vec!["superficie mullida".to_string(), "respaldo largo".to_string(), "apoyabrazos".to_string()],
                "armario" => vec!["volumen cerrado".to_string(), "puertas batientes".to_string(), "estantes internos".to_string()],
                "columna" => vec!["fuste vertical".to_string(), "base sólida".to_string(), "capitel".to_string()],
                "puente" => vec!["tablero horizontal".to_string(), "arcos de soporte".to_string(), "paso libre inferior".to_string()],
                "casa" => vec!["volumen habitable".to_string(), "cerramientos".to_string(), "cubierta superior".to_string()],
                "torre" => vec!["esbeltez vertical".to_string(), "núcleo central".to_string(), "mirador".to_string()],
                "madera" => vec!["textura orgánica".to_string(), "densidad media".to_string()],
                "acero" => vec!["superficie reflectante".to_string(), "alta resistencia".to_string()],
                _ => vec![],
            };

            self.vocabulary.insert(word.to_string(), AcquiredWord {
                word: word.to_string(),
                language: "es".to_string(),
                part_of_speech: pos.iter().map(|s| s.to_string()).collect(),
                meanings: meanings.iter().map(|s| s.to_string()).collect(),
                examples: vec![],
                synonyms: vec![],
                antonyms: vec![],
                related_terms: vec![],
                gender,
                number,
                structural_features,
                is_visually_grounded: category == DesignCategory::Furniture || category == DesignCategory::Shape,
                grounded_concept_id: if category == DesignCategory::Furniture { Some(word.to_string()) } else { None },
                design_category: Some(category),
            });
        }

        // ─── ENTRENAMIENTO MASIVO (ESCUELA DE 9 AÑOS) ───
        use crate::lingua::dictionary_seed::MASSIVE_SEED;
        log::info!("[LINGUA] Iniciando entrenamiento masivo de {} palabras...", MASSIVE_SEED.len());
        for word in MASSIVE_SEED {
            if !self.vocabulary.contains_key(*word) {
                let _ = self.acquire_synthetic(word);
            }
        }
    }

    /// Inferir la categoría de diseño a partir de la palabra y su POS
    fn infer_design_category(word: &str, pos: &[String]) -> DesignCategory {
        let w = word.to_lowercase();
        let furniture = ["mesa","silla","sofá","armario","escritorio","estantería","cama","taburete","banco"];
        let materials = ["madera","acero","vidrio","piedra","metal","plástico","cuero","mármol","concreto","aluminio"];
        
        if furniture.iter().any(|f| w == *f) { return DesignCategory::Furniture; }
        if materials.iter().any(|m| w == *m) { return DesignCategory::Material; }
        if pos.iter().any(|p| p.contains("verb")) { return DesignCategory::DesignVerb; }
        if pos.iter().any(|p| p.contains("adjective")) { return DesignCategory::Property; }
        
        DesignCategory::Uncategorized
    }

    /// Adquisición sintética: Motor de Inferencia Morfológica
    pub fn acquire_synthetic(&mut self, word: &str) -> AcquiredWord {
        let w = word.to_lowercase();
        
        // Inferencia Morfológica Básica
        let mut pos = "unknown".to_string();
        let mut meaning = format!("Concepto inferido heurísticamente: {}", word);
        let mut gender = Gender::Masculine;
        let mut number = WordNumber::Singular;

        // Inferencia de género/número
        if w.ends_with('a') || w.ends_with("as") { gender = Gender::Feminine; }
        if w.ends_with('s') && !w.ends_with("es") || w.ends_with("es") && w.len() > 3 { number = WordNumber::Plural; }

        if w.ends_with("mente") {
            pos = "adverb".to_string();
            meaning = format!("Modificador temporal o de modo derivado de '{}'", &w[..w.len()-5]);
        } else if w.ends_with("ar") || w.ends_with("er") || w.ends_with("ir") {
            pos = "verb".to_string();
            meaning = "Acción o proceso dinámico en el espacio".to_string();
            gender = Gender::Neutral;
        } else if w.ends_with("ando") || w.ends_with("iendo") {
            pos = "verb".to_string(); 
            meaning = "Proceso o acción en estado continuo".to_string();
            gender = Gender::Neutral;
        } else if w.ends_with("ísimo") || w.ends_with("ísima") {
            pos = "adjective".to_string();
            meaning = "Propiedad escalar maximizada".to_string();
        } else if w.ends_with("ito") || w.ends_with("ita") {
            pos = "noun".to_string();
            meaning = "Entidad física de volumen reducido".to_string();
        } else if w == "encerio" || w == "enserio" {
            pos = "adverb".to_string();
            meaning = "Confirmación probabilística de veracidad".to_string();
        }
        
        let category = Self::infer_design_category(word, &[pos.clone()]);
        
        let entry = AcquiredWord {
            word: word.to_string(),
            language: "es".to_string(),
            part_of_speech: vec![pos],
            meanings: vec![meaning],
            examples: vec![],
            synonyms: vec![],
            antonyms: vec![],
            related_terms: vec![],
            gender,
            number,
            structural_features: vec![],
            design_category: Some(category),
            is_visually_grounded: false,
            grounded_concept_id: None,
        };
        self.vocabulary.insert(word.to_string(), entry.clone());
        entry
    }

    /// Guardar caché a disco
    pub fn save_cache(&self) -> Result<(), String> {
        let _ = std::fs::create_dir_all(&self.cache_path);
        let path = format!("{}/vocabulary.json", self.cache_path);
        let json = serde_json::to_string_pretty(&self.vocabulary)
            .map_err(|e| format!("Serialization error: {}", e))?;
        std::fs::write(&path, json)
            .map_err(|e| format!("Write error: {}", e))?;
        log::info!("[LINGUA-ACQ] Caché guardado: {} palabras -> {}", self.vocabulary.len(), path);
        Ok(())
    }

    /// Acceder al vocabulario completo
    pub fn vocabulary(&self) -> &HashMap<String, AcquiredWord> {
        &self.vocabulary
    }

    pub fn update_entry(&mut self, entry: AcquiredWord) {
        self.vocabulary.insert(entry.word.clone(), entry);
    }

    pub fn vocabulary_size(&self) -> usize {
        self.vocabulary.len()
    }

    /// Obtener palabras filtradas por categoría de diseño
    pub fn words_by_category(&self, category: &DesignCategory) -> Vec<&AcquiredWord> {
        self.vocabulary.values()
            .filter(|w| {
                if let Some(ref cat) = w.design_category {
                    std::mem::discriminant(cat) == std::mem::discriminant(category)
                } else {
                    false
                }
            })
            .collect()
    }
}
