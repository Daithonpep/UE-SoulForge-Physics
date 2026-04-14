// NEXUS - Neural Evolution eXperimental Universal System
// Sistema de entrenamiento híbrido: SOFIA pura + Cross-pollination + Caos controlado
//
// Fase 1 (primera mitad de generaciones): Entrenamiento SOFIA puro
// Fase 2 (segunda mitad): Entrenamiento híbrido con GENESIS + CRUCIBLE + ENTROPY

pub mod cross_pollination;
pub mod crucible;
pub mod entropy;

use cross_pollination::GENESISBridge;
use crucible::CRUCIBLE;
use entropy::ENTROPY;

pub struct NEXUS {
    pub genesis: GENESISBridge,
    pub crucible: CRUCIBLE,
    pub entropy: ENTROPY,
}

impl NEXUS {
    pub fn new() -> Self {
        log::info!("⚡ NEXUS SYSTEM INITIALIZED");
        log::info!("  🧬 GENESIS: Cross-category learning");
        log::info!("  🔥 CRUCIBLE: Hostile environment training");
        log::info!("  🌀 ENTROPY: Controlled chaos exploration");

        Self {
            genesis: GENESISBridge::new(),
            crucible: CRUCIBLE::new(),
            entropy: ENTROPY::new(),
        }
    }
}
