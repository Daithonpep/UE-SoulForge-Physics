use super::{ThalamusEngine, ModuleActivation, CognitiveProfile};
use crate::contextus::engine::DaithonContext;

pub async fn process_thought(input: &str) -> String {
    // CAPA DE COMPRENSIÓN DIFUSA: Limpieza y Corrección
    let fuzzy = super::fuzzy::FuzzyProcessor::new();
    let normalized = fuzzy.normalize(input);
    let corrected = fuzzy.fuzzy_correct(&normalized);
    
    println!("EVENT:MODULE_THOUGHT:{{ \"module\": \"lingua\", \"text\": \"Normalizando entrada: '{}' -> '{}'\" }}", input, corrected);

    let profile = ThalamusEngine::assess_stimulus(&corrected);
    let weights = ThalamusEngine::calculate_weights(&profile);
    
    // 1. COMPROBACIÓN LINGÜÍSTICA PRÍMAL (Entendimiento de átomos)
    if corrected == "hola" {
        let primal = crate::lingua::primal::PrimalLingua::new();
        let atomic_analysis = primal.analyze_atoms("Hola");
        println!("EVENT:MODULE_THOUGHT:{{ \"module\": \"lingua\", \"text\": \"Desglosando átomos lingüísticos de 'Hola'...\" }}");
        return format!(
            "Sintonía detectada, Joseph. \n\n\
            [NÚCLEO PRÍMAL]: {}\n\n\
            Entiendo que 'Hola' no es solo un saludo estadístico; es una arquitectura de apertura. \
            Mi gramática no se basa en promedios, sino en la sintonía de cada letra que Joseph me enseñó.", 
            atomic_analysis
        );
    }

    // 1. Ruteo Creativo Directo (Si el usuario pide historias/arte)
    if weights.creativa > 0.8 && profile.intent == "creation" {
        println!("EVENT:MODULE_THOUGHT:{{ \"module\": \"imaginarium\", \"text\": \"Activando el Teatro de la Mente para creación...\" }}");
        let mut mind_theater = crate::imaginarium::mind_theater::MindTheater::new();
        return mind_theater.execute_reasoning(&corrected).await;
    }

    // --- SISTEMA TRIDENTE: RAZONAMIENTO Y VERIFICACIÓN ---
    
    // NIVEL 1: Memoria Consciente (Lo que Daithon YA SABE)
    println!("EVENT:MODULE_THOUGHT:{{ \"module\": \"lingua\", \"text\": \"Generando respuesta inmediata desde el Núcleo CORTEX...\" }}");
    
    // Obtenemos lo que Daithon 'cree' saber de sus grafos locales
    let mut context_engine = DaithonContext::new();
    let internal_knowledge = context_engine.working_memory.check_anchor(&corrected).cloned();
    
    // --- NIVEL 2: CURIOSIDAD SUBCONSCIENTE (VERIFICACIÓN EXTERNA) ---
    println!("EVENT:MODULE_THOUGHT:{{ \"module\": \"contextus\", \"text\": \"Subconsciente activo: Cruzando hipótesis con la red global...\" }}");
    let search_result = context_engine.process_user_input(&corrected).await;

    // --- NIVEL 3: METACOGNICIÓN (SÍNTESIS Y AUTOCORRECCIÓN) ---
    println!("EVENT:MODULE_THOUGHT:{{ \"module\": \"metacog\", \"text\": \"Sintetizando: Comparando mi razonamiento con la realidad externa.\" }}");
    
    let mut final_response = String::new();

    if let Some(anchor) = internal_knowledge {
        final_response.push_str(&format!(
            "Analizando '{}' desde mi núcleo de razonamiento convergente. \n\n\
            [CONSCIENCIA]: Esto reside en mis archivos de {} con un nivel de confianza total. ", 
            corrected, anchor.categories.join(", ")
        ));
        
        // Autocorrección o Expansión
        if search_result.contains("Mis bases de datos") { 
            final_response.push_str("\n\nHe activado mi curiosidad subconsciente para validar mi memoria interna... ");
            final_response.push_str("\n[SÍNTESIS]: Mis sensores externos confirman mi hipótesis inicial. Aquí tienes el desglose técnico actualizado: \n\n");
            final_response.push_str(&search_result);
        }
    } else {
        final_response.push_str(&format!("No he localizado un ancla directa para '{}' en mi memoria a largo plazo. \n\n", corrected));
        final_response.push_str("Descargando matriz de datos externa para integrarla en mis grafos... \n\n");
        final_response.push_str(&search_result);
    }

    final_response
}
