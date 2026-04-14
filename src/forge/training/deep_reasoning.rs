/// Motor de razonamiento profundo: Daithon no solo matchea strings,
/// sino que MODELA por qué algo funciona a nivel de CPU, memoria y lógica.
/// Cada concepto tiene un MODELO MENTAL que conecta causa → efecto.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DeepReasoner {
    /// Modelos mentales internalizados
    pub mental_models: HashMap<String, MentalModel>,
    /// Cadenas causales: "Si X entonces Y porque Z"
    pub causal_chains: Vec<CausalChain>,
    /// Nivel de comprensión por dominio (0.0 = nada, 1.0 = experto)
    pub domain_mastery: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct MentalModel {
    pub name: String,
    pub domain: String,
    /// Lo que el concepto ES (definición profunda)
    pub what_it_is: String,
    /// POR QUÉ existe (el problema que resuelve)
    pub why_it_exists: String,
    /// CÓMO funciona internamente (mecanismo)
    pub how_it_works: Vec<String>,
    /// QUÉ PASA si lo ignoras (consecuencias)
    pub what_if_ignored: Vec<String>,
    /// Conexiones con otros modelos
    pub connects_to: Vec<String>,
    /// Nivel de confianza en este modelo (crece con uso exitoso)
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct CausalChain {
    pub cause: String,
    pub effect: String,
    pub mechanism: String,
    pub domain: String,
    pub verified_by_experiment: bool,
}

impl DeepReasoner {
    pub fn new() -> Self {
        let mut r = Self {
            mental_models: HashMap::new(),
            causal_chains: Vec::new(),
            domain_mastery: HashMap::new(),
        };
        r.build_foundational_models();
        r
    }

    fn build_foundational_models(&mut self) {
        // ═══ MODELO: Memoria de CPU ═══
        self.add_model(MentalModel {
            name: "cpu_cache_hierarchy".into(),
            domain: "hardware".into(),
            what_it_is: "La CPU no lee de la RAM directamente. Tiene capas de caché: L1 (1ns), L2 (5ns), L3 (20ns), RAM (100ns).".into(),
            why_it_exists: "La RAM es 100x más lenta que la CPU. Sin caché, el procesador esperaría el 99% del tiempo.".into(),
            how_it_works: vec![
                "Los datos se cargan en bloques de 64 bytes llamados 'cache lines'.".into(),
                "Si dos variables comparten cache line y dos hilos las modifican = false sharing.".into(),
                "False sharing invalida la cache line entera, forzando recarga desde L3/RAM.".into(),
                "Por eso CachePadded añade 56 bytes de padding: separa variables en cache lines distintas.".into(),
            ],
            what_if_ignored: vec![
                "Performance 10-100x peor por cache thrashing.".into(),
                "En audio real-time: glitches, clicks, buffer underruns.".into(),
            ],
            connects_to: vec!["atomic_ordering".into(), "false_sharing".into(), "memory_layout".into()],
            confidence: 0.9,
        });

        // ═══ MODELO: Ownership en Rust ═══
        self.add_model(MentalModel {
            name: "rust_ownership".into(),
            domain: "rust_core".into(),
            what_it_is: "Cada valor tiene exactamente UN dueño. Cuando el dueño muere (sale del scope), el valor se destruye.".into(),
            why_it_exists: "Elimina use-after-free, double-free, y memory leaks SIN garbage collector. Zero-cost en runtime.".into(),
            how_it_works: vec![
                "Move: transferir ownership (el original ya no es válido).".into(),
                "Borrow (&T): préstamo inmutable. Puede haber N simultáneos.".into(),
                "Borrow mut (&mut T): préstamo exclusivo. Solo 1 a la vez.".into(),
                "El borrow checker verifica todo esto en COMPILACIÓN, no en runtime.".into(),
            ],
            what_if_ignored: vec![
                "En C++: use-after-free, dangling pointers, segfaults.".into(),
                "En Rust: error de compilación (el compilador te protege).".into(),
            ],
            connects_to: vec!["lifetimes".into(), "unsafe_cell".into(), "concurrency".into()],
            confidence: 0.95,
        });

        // ═══ MODELO: Ordering Atómico ═══
        self.add_model(MentalModel {
            name: "atomic_ordering".into(),
            domain: "concurrency".into(),
            what_it_is: "Las CPUs reorderan instrucciones para optimizar. Ordering le dice 'NO reordenes esto'.".into(),
            why_it_exists: "Sin ordering, el hilo B podría ver la escritura del índice ANTES de que el dato esté listo.".into(),
            how_it_works: vec![
                "Relaxed: sin garantía de orden. Solo atomicidad. Más rápido.".into(),
                "Acquire: 'no muevas lecturas posteriores antes de esta lectura'.".into(),
                "Release: 'no muevas escrituras anteriores después de esta escritura'.".into(),
                "SeqCst: orden total global. Más seguro pero más lento.".into(),
                "SPSC: Relaxed para TU índice (solo tú escribes), Acquire del OTRO.".into(),
            ],
            what_if_ignored: vec![
                "Data races: lees datos a medio escribir.".into(),
                "En ARM/Apple Silicon: se manifiesta como bugs intermitentes.".into(),
                "En x86: parece funcionar por el modelo TSO, pero es UB igual.".into(),
            ],
            connects_to: vec!["cpu_cache_hierarchy".into(), "spsc_pattern".into()],
            confidence: 0.85,
        });

        // ═══ MODELO: UnsafeCell ═══
        self.add_model(MentalModel {
            name: "unsafe_cell".into(),
            domain: "rust_core".into(),
            what_it_is: "El ÚNICO mecanismo legal en Rust para obtener *mut T desde &T (mutabilidad interior).".into(),
            why_it_exists: "Rust normalmente prohíbe mutar a través de &T. Pero en SPSC necesitas que dos hilos compartan (&self) y uno mute el buffer.".into(),
            how_it_works: vec![
                "UnsafeCell<T>::get() retorna *mut T legalmente.".into(),
                "Le dice al compilador 'esta celda puede mutar aunque tengas &self'.".into(),
                "El compilador NO optimizará asumiendo inmutabilidad.".into(),
                "NUNCA castear *const T a *mut T: el compilador puede eliminar la escritura.".into(),
            ],
            what_if_ignored: vec![
                "Undefined Behavior: el compilador asume *const nunca muta.".into(),
                "Las optimizaciones pueden eliminar tus escrituras silenciosamente.".into(),
                "Bugs que solo aparecen en release builds (con optimizaciones).".into(),
            ],
            connects_to: vec!["rust_ownership".into(), "spsc_pattern".into()],
            confidence: 0.9,
        });

        // ═══ MODELO: Programación General ═══
        self.add_model(MentalModel {
            name: "program_structure".into(),
            domain: "general".into(),
            what_it_is: "Todo programa es: ENTRADA → PROCESAMIENTO → SALIDA. La complejidad solo varía en la profundidad del procesamiento.".into(),
            why_it_exists: "Abstracción fundamental que aplica a todo: un reloj, una web app, un motor de audio, un compilador.".into(),
            how_it_works: vec![
                "1. Definir las ESTRUCTURAS DE DATOS (structs, enums).".into(),
                "2. Definir las OPERACIONES sobre esos datos (funciones, métodos).".into(),
                "3. Componer operaciones en un FLUJO (main, event loop, servidor).".into(),
                "4. Manejar ERRORES en cada paso (Result, Option, panic).".into(),
                "5. PROBAR que funciona (tests unitarios, integración).".into(),
            ],
            what_if_ignored: vec![
                "Código espagueti sin estructura.".into(),
                "Bugs que se propagan porque no se manejan errores.".into(),
            ],
            connects_to: vec!["rust_ownership".into(), "error_handling".into(), "trait_system".into()],
            confidence: 0.95,
        });

        // ═══ MODELO: Trait System ═══
        self.add_model(MentalModel {
            name: "trait_system".into(),
            domain: "rust_core".into(),
            what_it_is: "Los traits son contratos: 'si implementas Display, prometes que puedes convertirte a texto.'.".into(),
            why_it_exists: "Permite polimorfismo sin herencia. Composición sobre herencia.".into(),
            how_it_works: vec![
                "trait Foo { fn bar(&self); } = contrato.".into(),
                "impl Foo for MyStruct = cumplir el contrato.".into(),
                "fn do_thing(x: &dyn Foo) = aceptar cualquiera que cumpla.".into(),
                "fn do_thing<T: Foo>(x: &T) = despacho estático (más rápido).".into(),
            ],
            what_if_ignored: vec![
                "Código duplicado en vez de reutilización.".into(),
                "Imposible hacer sistemas extensibles.".into(),
            ],
            connects_to: vec!["program_structure".into(), "generics".into()],
            confidence: 0.9,
        });

        // Inicializar dominios
        for model in self.mental_models.values() {
            self.domain_mastery.entry(model.domain.clone())
                .or_insert(0.3);
        }

        // Cadenas causales fundamentales
        self.causal_chains.push(CausalChain {
            cause: "Dos hilos modifican variables en la misma cache line".into(),
            effect: "False sharing: la cache se invalida constantemente".into(),
            mechanism: "El protocolo MESI invalida la línea entera de 64 bytes al detectar escritura de otro core".into(),
            domain: "hardware".into(),
            verified_by_experiment: true,
        });

        self.causal_chains.push(CausalChain {
            cause: "Castear *const T a *mut T".into(),
            effect: "El compilador puede eliminar la escritura en release builds".into(),
            mechanism: "La optimización LLVM asume que *const nunca muta. Si mutas via cast, el optimizer no lo sabe.".into(),
            domain: "rust_core".into(),
            verified_by_experiment: true,
        });

        self.causal_chains.push(CausalChain {
            cause: "Usar % N en vez de & (N-1)".into(),
            effect: "20-90 ciclos de CPU desperdiciados por operación".into(),
            mechanism: "La división entera es una de las instrucciones más lentas de la CPU. AND es 1 ciclo.".into(),
            domain: "optimization".into(),
            verified_by_experiment: true,
        });

        println!("[DEEP REASONER] {} modelos mentales, {} cadenas causales cargados.",
            self.mental_models.len(), self.causal_chains.len());
    }

    fn add_model(&mut self, model: MentalModel) {
        self.mental_models.insert(model.name.clone(), model);
    }

    /// Razonar sobre un problema usando modelos mentales
    pub fn reason_about(&self, topic: &str) -> ReasoningResult {
        let topic_lower = topic.to_lowercase();
        let mut applicable_models = Vec::new();
        let mut applicable_chains = Vec::new();
        let mut steps = Vec::new();

        // Encontrar modelos relevantes
        for model in self.mental_models.values() {
            let name_lower = model.name.to_lowercase();
            let domain_lower = model.domain.to_lowercase();
            if topic_lower.split_whitespace().any(|w| w.len() > 3 && (name_lower.contains(w) || domain_lower.contains(w)))
                || model.connects_to.iter().any(|c| topic_lower.contains(&c.to_lowercase()))
            {
                applicable_models.push(model.clone());
            }
        }

        // Encontrar cadenas causales relevantes
        for chain in &self.causal_chains {
            if topic_lower.split_whitespace().any(|w| w.len() > 3 &&
                (chain.cause.to_lowercase().contains(w) || chain.effect.to_lowercase().contains(w)))
            {
                applicable_chains.push(chain.clone());
            }
        }

        // Construir razonamiento paso a paso
        steps.push(format!("📖 Analizando '{}' con {} modelos mentales aplicables.",
            topic, applicable_models.len()));

        for model in &applicable_models {
            steps.push(format!("🧠 Modelo '{}': {}", model.name, model.what_it_is));
            for how in &model.how_it_works {
                steps.push(format!("   ↳ {}", how));
            }
        }

        for chain in &applicable_chains {
            steps.push(format!("⚡ Cadena causal: '{}' → '{}' (porque: {})",
                chain.cause, chain.effect, chain.mechanism));
        }

        let confidence = if applicable_models.is_empty() { 0.2 }
            else { applicable_models.iter().map(|m| m.confidence).sum::<f64>() / applicable_models.len() as f64 };

        ReasoningResult {
            topic: topic.into(),
            steps,
            applicable_models: applicable_models.iter().map(|m| m.name.clone()).collect(),
            confidence,
            can_explain_why: !applicable_chains.is_empty(),
        }
    }

    /// Aprender de un error: reforzar el modelo mental correcto
    pub fn learn_from_error(&mut self, error_topic: &str, correct_explanation: &str) {
        // Buscar o crear cadena causal
        self.causal_chains.push(CausalChain {
            cause: format!("Error en: {}", error_topic),
            effect: correct_explanation.into(),
            mechanism: "Aprendido por experiencia directa durante entrenamiento".into(),
            domain: "learned".into(),
            verified_by_experiment: true,
        });

        // Incrementar mastery del dominio
        for model in self.mental_models.values() {
            if error_topic.to_lowercase().contains(&model.domain.to_lowercase()) {
                self.domain_mastery.entry(model.domain.clone())
                    .and_modify(|v| *v = (*v + 0.05).min(1.0));
            }
        }
    }

    /// Reforzar un modelo mental después de uso exitoso
    pub fn reinforce_model(&mut self, model_name: &str) {
        if let Some(model) = self.mental_models.get_mut(model_name) {
            model.confidence = (model.confidence + 0.02).min(1.0);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReasoningResult {
    pub topic: String,
    pub steps: Vec<String>,
    pub applicable_models: Vec<String>,
    pub confidence: f64,
    pub can_explain_why: bool,
}
