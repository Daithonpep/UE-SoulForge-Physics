//! # Proxy – Destrucción de Proxies Físicos (v2 – Física Realista)
//!
//! ## Arquitectura de Fragmentos
//!
//! Cada fragmento tiene una **categoría** que determina:
//!   - Su **masa** (proporcional al volumen estimado del fragmento).
//!   - Su **respuesta al Cerebro de Enjambre**:
//!
//! ```
//! F_swarm = Potencia_Cerebro / masa_fragmento
//! ```
//!
//!   Los fragmentos pequeños (polvo/astillas) tienen poca masa → F_swarm alta
//!   → el enjambre los controla con facilidad.
//!   Los fragmentos grandes (losas/vigas) tienen mucha masa → F_swarm baja
//!   → tienen inercia propias; el enjambre apenas los perturba.
//!
//! ## Presets de Daño
//!
//! | Preset      | Tipo de daño   | Comportamiento                        |
//! |-------------|----------------|---------------------------------------|
//! | `Explosion` | Radial         | Onda expansiva; enjambre mantiene grupo |
//! | `Collapse`  | Gravedad       | Caída orgánica en "planchas"           |
//! | `Impact`    | Puntual        | Proyectil de alta velocidad            |

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::{Mutex, OnceLock};

// ════════════════════════════════════════════════════════════════════════════
// TIPOS PÚBLICOS
// ════════════════════════════════════════════════════════════════════════════

/// Categoría de un fragmento — determina masa, tamaño y respuesta al enjambre.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum FragmentCategory {
    /// Losa / viga — fragmento estructural masivo.
    /// Inercia alta; el enjambre apenas lo mueve.
    Slab   = 0,
    /// Cascote / trozo mediano.
    /// Balance entre inercia y control del enjambre.
    Chunk  = 1,
    /// Grava / gravilla fina.
    /// El enjambre la controla con facilidad.
    Gravel = 2,
    /// Polvo / partículas.
    /// Masa casi nula; sigue perfectamente al enjambre (VFX de Niagara).
    Dust   = 3,
}

impl FragmentCategory {
    /// Escala de tamaño relativa respecto al fragmento padre (radio normalizado).
    pub fn size_scale(&self) -> f32 {
        match self {
            FragmentCategory::Slab   => 0.55,  // 55% del tamaño original
            FragmentCategory::Chunk  => 0.30,
            FragmentCategory::Gravel => 0.12,
            FragmentCategory::Dust   => 0.03,
        }
    }

    /// Fracción de la masa total del proxy que absorbe esta categoría.
    pub fn mass_fraction(&self) -> f32 {
        match self {
            FragmentCategory::Slab   => 0.60,  // el 60% de la masa va a losas
            FragmentCategory::Chunk  => 0.28,
            FragmentCategory::Gravel => 0.10,
            FragmentCategory::Dust   => 0.02,
        }
    }

    /// Multiplicador de la fuerza del Enjambre aplicada a esta categoría.
    /// F_swarm_efectiva = F_swarm_base * swarm_influence / masa
    pub fn swarm_influence(&self) -> f32 {
        match self {
            FragmentCategory::Slab   => 0.10,  // casi inmune al enjambre
            FragmentCategory::Chunk  => 0.40,
            FragmentCategory::Gravel => 0.85,
            FragmentCategory::Dust   => 1.00,  // sigue perfectamente al enjambre
        }
    }
}

/// Preset de daño — controla la distribución y velocidad de los fragmentos.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum DamagePreset {
    /// Explosión radial: onda expansiva desde un epicentro.
    /// Los fragmentos salen en todas las direcciones; el enjambre los
    /// agrupa como una esfera de escombros expansiva.
    Explosion = 0,

    /// Colapso gravitacional: el edificio se desmorona en "planchas".
    /// La cohesión de los Boids mantiene grupos de escombros unidos
    /// (simula cables y varillas de refuerzo aún conectadas).
    Collapse  = 1,

    /// Impacto puntual: proyectil de alta energía.
    Impact = 2,

    /// Explosiones Direccionales (Shaped Charges)
    /// Para corte preciso y penetración táctica.
    ShapedCharge = 3,

    /// Implosión: los pedazos viajan HACIA el centro.
    Implosion = 4,

    /// Cinético: Efecto Matrix / Gravedad Cero y fricción masiva.
    Cinematic = 5,
}

/// Nivel de Detalle Físico (L.O.D.)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PhysicsLOD {
    /// Primario (Hero): Grady-Kipp + Johnson-Cook + Thor.
    Primary = 0,
    /// Secundario (Medio): Gurney + Mott.
    Secondary = 1,
    /// Terciario (Fondo): Cinematic Nodos puros.
    Tertiary = 2,
}

/// Fragmento físico resultante de la destrucción de un proxy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyFragment {
    pub fragment_id:   u32,
    pub category:      FragmentCategory,
    /// Centro del fragmento en espacio del mundo (cm, coordenadas de Unreal).
    pub position:      [f32; 3],
    /// Velocidad inicial (cm/s).
    pub velocity:      [f32; 3],
    /// Masa del fragmento (kg). Calculada como:
    ///   masa = masa_proxy × fraccion_categoria × variacion_aleatoria
    pub mass:          f32,
    /// Radio del fragmento (cm). Calculado a partir del volumen.
    pub radius:        f32,
    /// Escala 3D (para formas aleatorias: losas, agujas, cubos)
    pub scale:         [f32; 3],
    /// Multiplicador de fuerza de enjambre [0..1].
    /// Permite que Python / C++ sepan qué fragmentos registrar como agentes.
    pub swarm_influence: f32,
    /// Capa de profundidad de la que proviene (0 = exterior, 2 = interior)
    pub layer_depth:   u8,
}

impl ProxyFragment {
    /// Calcula el radio a partir de la masa asumiendo densidad constante.
    /// Usa la fórmula inversa de V = (4/3)π·r³ con densidad δ (kg/cm³).
    pub fn radius_from_mass(mass: f32, density: f32) -> f32 {
        let volume = mass / density.max(0.0001);
        (volume * 3.0 / (4.0 * PI)).cbrt()
    }
}

/// Resultado completo de la destrucción de un proxy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestructionResult {
    pub proxy_id:       String,
    pub preset:         DamagePreset,
    pub fragments:      Vec<ProxyFragment>,
    pub epicenter:      [f32; 3],
    /// Dirección del impacto (para Shaped Charges).
    pub direction:      [f32; 3],
    /// Energía total liberada (Joules estimados). Usada por el enjambre
    /// para escalar la fuerza de separación inicial.
    pub total_energy:   f32,
    pub timestamp:      f32,
}

// ════════════════════════════════════════════════════════════════════════════
// PROXY FÍSICO
// ════════════════════════════════════════════════════════════════════════════

/// Proxy AABB (caja alineada con los ejes) que representa un objeto destruible.
#[derive(Debug, Clone)]
pub struct PhysicsProxy {
    pub id:       String,
    pub position: [f32; 3],   // centro del proxy (cm)
    pub half_ext: [f32; 3],   // semi-extensiones de la AABB (cm)
    pub mass:     f32,         // masa total (kg)
    pub health:   f32,         // 1.0=intacto, 0.0=destruido
    /// Capas de Salud (Voxels de Profundidad):
    /// El primer impacto afecta a la capa 0 (exterior).
    /// Si un impacto es lo suficientemente fuerte, penetra a la 1 y luego a la 2.
    pub health_layers: [f32; 3],
    pub density:  f32,         // kg/cm³ (hormigón ≈ 0.0024, madera ≈ 0.0006)
    pub lod:      PhysicsLOD,
    pub material: crate::military::material::JohnsonCookMaterial,
}

impl PhysicsProxy {
    pub fn new(id: String, position: [f32; 3], half_ext: [f32; 3], density: f32) -> Self {
        // Masa = densidad × volumen de la AABB
        let volume = 8.0 * half_ext[0] * half_ext[1] * half_ext[2];
        let mass   = density * volume;
        
        // Selección de material por densidad aproximada
        let material = if density > 0.007 {
            crate::military::material::JohnsonCookMaterial::steel_4340()
        } else if density > 0.0025 {
            crate::military::material::JohnsonCookMaterial::aluminum_6061()
        } else {
            crate::military::material::JohnsonCookMaterial::concrete_35mpa()
        };

        Self { 
            id, position, half_ext, mass, 
            health: 1.0, 
            health_layers: [1.0, 1.0, 1.0], 
            density,
            lod: PhysicsLOD::Primary, // Por defecto alta fidelidad
            material
        }
    }

    /// Masa total calculada del proxy.
    pub fn mass(&self) -> f32 { self.mass }
}

// ════════════════════════════════════════════════════════════════════════════
// ESTADO GLOBAL
// ════════════════════════════════════════════════════════════════════════════

pub struct ProxyState {
    pub proxies:  HashMap<String, PhysicsProxy>,
}
impl ProxyState {
    fn new() -> Self { Self { proxies: HashMap::new() } }
}

static PROXY: OnceLock<Mutex<ProxyState>> = OnceLock::new();
pub fn state() -> &'static Mutex<ProxyState> {
    PROXY.get_or_init(|| Mutex::new(ProxyState::new()))
}

// ════════════════════════════════════════════════════════════════════════════
// API INTERNA
// ════════════════════════════════════════════════════════════════════════════

/// Registra un proxy con geometría AABB y densidad de material (Anclaje Inteligente por Nombre).
pub fn register_proxy(
    id: String,
    position: [f32; 3],
    half_ext: [f32; 3],
    density:  f32,
) -> Result<String, String> {
    let mut s = state().lock().map_err(|e| e.to_string())?;
    
    if let Some(proxy) = s.proxies.get_mut(&id) {
        proxy.position = position;
        crate::ffi::sf_log(&format!("🧠 Identidad reconocida: {}. Re-anclando posición.", id));
    } else {
        let proxy = PhysicsProxy::new(id.clone(), position, half_ext, density);
        crate::ffi::sf_log(&format!("✅ Nuevo objeto [{}] registrado en la Forja.", id));
        s.proxies.insert(id.clone(), proxy);
    }
    
    Ok(id)
}

/// Elimina todos los proxies registrados.
pub fn clear() {
    if let Ok(mut s) = state().lock() {
        s.proxies.clear();
        crate::ffi::sf_log("♻️ [RUST] Estado interno reseteado (DNI persistente habilitado).");
    }
}

pub fn remove(id: String) -> bool {
    if let Ok(mut s) = state().lock() {
        s.proxies.remove(&id).is_some()
    } else {
        false
    }
}

/// Simula la destrucción para el Laboratorio de Balística (Pre-visualización).
/// No elimina el proxy ni registra nada en la física.
pub fn preview_destruction(
    proxy_id: String,
    preset: DamagePreset,
    energy: f32,
    direction: [f32; 3],
    custom_fragment_count: u32,
) -> Result<Vec<ProxyFragment>, String> {
    let proxy = {
        let s = state().lock().map_err(|e| e.to_string())?;
        match s.proxies.get(&proxy_id) {
            Some(p) => p.clone(),
            None => return Err(format!("Proxy [{}] no encontrado", proxy_id)),
        }
    };

    let destroyed_layers = if preset == DamagePreset::ShapedCharge && energy > 1_000_000.0 { 3 } 
                          else if energy > 500_000.0 { 2 } else { 1 };

    Ok(generate_fragments_layered(&proxy, preset, destroyed_layers, direction, custom_fragment_count, energy))
}

/// Destruye el proxy usando el preset de daño indicado y la posición GPS real.
pub fn destroy(
    proxy_id: String, 
    preset: DamagePreset, 
    energy: f32,
    gps_position: [f32; 3], // Recibimos el GPS en vivo desde Unreal
    _radius: f32,
    custom_fragment_count: u32,
) -> Result<String, String> {
    let mut destroyed_layers = 1u8;
    let mut proxy = {
        let mut s = state().lock().map_err(|e| e.to_string())?;
        match s.proxies.remove(&proxy_id) {
            Some(p) => p,
            None => {
                crate::ffi::sf_log(&format!("❌ ERROR: No se encontró el objeto [{}] para detonar.", proxy_id));
                return Err(format!("Proxy [{}] no encontrado", proxy_id));
            }
        }
    };

    // ACTUALIZAMOS EL GPS: La realidad de Unreal manda sobre la memoria de Rust
    proxy.position = gps_position;

    crate::ffi::sf_log(&format!("💥 Detonando [{}] en coordenadas GPS: Z={}", proxy_id, gps_position[2]));

    // ── Voxel Penetration Logic ──────────────────────────────────────────────
    if preset == DamagePreset::ShapedCharge && energy > 1_000_000.0 {
        destroyed_layers = 3;
    } else if energy > 500_000.0 {
        destroyed_layers = 2;
    }

    // Dirección por defecto arriba para explosiones, o se podría calcular según el impacto
    let impact_dir = [0.0, 0.0, 1.0]; 

    let fragments = generate_fragments_layered(&proxy, preset, destroyed_layers, impact_dir, custom_fragment_count, energy);
    let total_simulation_energy = energy;

    log::info!(
        "[Proxy] [{}] impactado (GPS Accurate) | coords={:?} | nodos={} | capas_rotas={} | preset={:?} | E={:.0}",
        proxy_id, gps_position, custom_fragment_count, destroyed_layers, preset, energy
    );

    // ── Registrar explosión en el simulador de fragmentos (Air Blast) ────────
    crate::fragment_sim::add_explosion_event(proxy.position, energy as f64);

    let result = DestructionResult {
        proxy_id: proxy_id.clone(),
        preset,
        fragments,
        epicenter: proxy.position,
        direction: impact_dir,
        total_energy: total_simulation_energy,
        timestamp: 0.0,
    };

    // ── Auto-registrar fragmentos en FragSim ─────────────────────────────────
    for frag in &result.fragments {
        let _ = crate::fragment_sim::register_fragment(
            frag.category,
            frag.position,
            frag.velocity,
            frag.mass,
            frag.scale,
            preset as i32,
            proxy.material.id,
        );

        // Despertar agentes cercanos en el enjambre (Chain Reaction)
        crate::swarm::wake_agent_near(frag.position, 120.0);
    }

    crate::fragment_sim::trigger_auto_power(preset as i32, proxy.position, energy);

    crate::ffi::sf_log(&format!("✅ [{}] fragmentado en {} pedazos.", proxy_id, result.fragments.len()));

    serde_json::to_string(&result).map_err(|e| e.to_string())
}

// ════════════════════════════════════════════════════════════════════════════
// GENERACIÓN DE FRAGMENTOS POR PRESET
// ════════════════════════════════════════════════════════════════════════════

/// Conteo de fragmentos por categoría según el preset.
struct FragmentLayout {
    slabs:   u32,
    chunks:  u32,
    gravels: u32,
    dusts:   u32,
}

impl FragmentLayout {
    fn for_preset(preset: DamagePreset) -> Self {
        match preset {
            // Explosión: muchos fragmentos pequeños, pocos grandes
            DamagePreset::Explosion => Self { slabs: 2,  chunks: 6,  gravels: 12, dusts: 20 },
            // Colapso: predominan las losas (planchas estructurales)
            DamagePreset::Collapse  => Self { slabs: 6,  chunks: 8,  gravels: 8,  dusts: 10 },
            // Impacto: pocos fragmentos grandes, cono de escombros
            DamagePreset::Impact    => Self { slabs: 1,  chunks: 4,  gravels: 10, dusts: 15 },
            // Carga hueca: ultra focalizado
            DamagePreset::ShapedCharge => Self { slabs: 1,  chunks: 2,  gravels: 15, dusts: 30 },
            // Implosión: igual que explosión pero v se invertirá después
            DamagePreset::Implosion => Self { slabs: 2,  chunks: 6,  gravels: 12, dusts: 20 },
            // Cinético: Matrix
            DamagePreset::Cinematic => Self { slabs: 4,  chunks: 8,  gravels: 15, dusts: 25 },
        }
    }

    fn total(&self) -> u32 {
        self.slabs + self.chunks + self.gravels + self.dusts
    }
}

fn generate_fragments_layered(
    proxy: &PhysicsProxy, 
    preset: DamagePreset, 
    destroyed_layers: u8,
    direction: [f32; 3],
    custom_count: u32,
    energy: f32,
) -> Vec<ProxyFragment> {
    use crate::military::fracture::GradyFragmentation;
    use crate::military::gurney::{GurneyModel, ChargeGeometry};
    use crate::military::Vec3;
    use rand::thread_rng;

    let mut fragments = Vec::new();
    let mut rng = thread_rng();

    // --- LOGICA ORIGINAL SOULFORGE (Restaurada) ---
    let layout = FragmentLayout::for_preset(preset);
    let base_total = if custom_count > 0 { custom_count } else { layout.total() };
    
    let mut frag_id = 0u32;
    let total_layout = layout.total() as f32;
    let categories = [
        (FragmentCategory::Slab,   ((layout.slabs as f32 / total_layout) * base_total as f32) as u32),
        (FragmentCategory::Chunk,  ((layout.chunks as f32 / total_layout) * base_total as f32) as u32),
        (FragmentCategory::Gravel, ((layout.gravels as f32 / total_layout) * base_total as f32) as u32),
        (FragmentCategory::Dust,   ((layout.dusts as f32 / total_layout) * base_total as f32) as u32),
    ];

    // Multiplicador de velocidad basado en energía (Feeling original)
    // 1 MJ (1_000_000) es el estándar. Si envían 500M será una bestia. Si envían poco, al menos será 1.0
    let energy_mult = (energy / 1_000_000.0).sqrt().clamp(1.0, 50.0);
    let chaos_factor = 0.28; // El "Sabor" a destrucción real (28% de variabilidad)

    for layer in 0..destroyed_layers {
        for (cat, count) in &categories {
            let cat_mass_total = proxy.mass * cat.mass_fraction() / (destroyed_layers as f32);
            let mass_per_frag  = cat_mass_total / (*count).max(1) as f32;

            for i in 0..*count {
                // --- MODO CAOS (Jitter solicitado por el usuario) ---
                let mut vel_v = fragment_velocity_dir(proxy, preset, *cat, i, *count, direction);
                
                let jitter_force = 1.0 + (rng.gen::<f32>() - 0.5) * chaos_factor;
                let jitter_dir = [
                    (rng.gen::<f32>() - 0.5) * chaos_factor,
                    (rng.gen::<f32>() - 0.5) * chaos_factor,
                    (rng.gen::<f32>() - 0.5) * chaos_factor,
                ];

                vel_v[0] = (vel_v[0] * energy_mult + jitter_dir[0] * 500.0) * jitter_force;
                vel_v[1] = (vel_v[1] * energy_mult + jitter_dir[1] * 500.0) * jitter_force;
                vel_v[2] = (vel_v[2] * energy_mult + jitter_dir[2] * 500.0) * jitter_force;
                
                // --- FORMAS ALEATORIAS (Scale Variance) ---
                // Generamos escalas no uniformes para que no sean solo cubos
                let radius = ProxyFragment::radius_from_mass(mass_per_frag, proxy.density);
                let base_s = (radius * 2.0) / 100.0; // El mesh de Unreal es de 100cm, ajustamos la escala al diámetro real
                let scale_v = [
                    base_s * rng.gen_range(0.5..2.5),
                    base_s * rng.gen_range(0.5..2.5),
                    base_s * rng.gen_range(0.5..2.5),
                ];

                // Aplicamos el escalado de energía original
                vel_v[0] *= energy_mult;
                vel_v[1] *= energy_mult;
                vel_v[2] *= energy_mult;

                let pos = fragment_position(proxy, preset, *cat, i, *count);

                fragments.push(ProxyFragment {
                    fragment_id:    frag_id,
                    category:       *cat,
                    position:       pos,
                    velocity:       vel_v,
                    mass:           mass_per_frag,
                    radius,
                    scale: scale_v,
                    swarm_influence: cat.swarm_influence(),
                    layer_depth:    layer,
                });
                frag_id += 1;
            }
        }
    }

    fragments
}

// ════════════════════════════════════════════════════════════════════════════
// LÓGICA DE VELOCIDAD POR PRESET (CON DIRECCIÓN)
// ════════════════════════════════════════════════════════════════════════════

fn fragment_velocity_dir(
    _proxy: &PhysicsProxy,
    preset: DamagePreset,
    cat:    FragmentCategory,
    index:  u32,
    total:  u32,
    direction: [f32; 3],
) -> [f32; 3] {
    let speed_scale = match cat {
        FragmentCategory::Slab   => 0.4,
        FragmentCategory::Chunk  => 0.7,
        FragmentCategory::Gravel => 1.0,
        FragmentCategory::Dust   => 1.4,
    };

    match preset {
        DamagePreset::Explosion => {
            let base_speed = 400.0 * speed_scale;
            let dir = fibonacci_sphere_point(index, total.max(1));
            [dir[0] * base_speed, dir[1] * base_speed, dir[2] * base_speed]
        }
        DamagePreset::Collapse => {
            let t = index as f32 / total.max(1) as f32;
            let lateral = match cat {
                FragmentCategory::Slab => 30.0, FragmentCategory::Chunk => 80.0,
                FragmentCategory::Gravel => 150.0, FragmentCategory::Dust => 250.0,
            };
            let angle = t * 2.0 * PI;
            [angle.cos() * lateral, angle.sin() * lateral, -200.0 * speed_scale]
        }
        DamagePreset::Impact => {
            let base_speed = 600.0 * speed_scale;
            let _cone_angle = PI / 4.0; // 45°
            let _t = index as f32 / total.max(1) as f32;
            let random_dir = fibonacci_sphere_point(index, total.max(1));
            // Mix entre dirección del impacto y random cone
            [
                (direction[0] * 0.7 + random_dir[0] * 0.3) * base_speed,
                (direction[1] * 0.7 + random_dir[1] * 0.3) * base_speed,
                (direction[2] * 0.7 + random_dir[2] * 0.3) * base_speed,
            ]
        }
        DamagePreset::ShapedCharge => {
            let base_speed = 1200.0 * speed_scale; // Alta energía
            let _t = index as f32 / total.max(1) as f32;
            let random_dir = fibonacci_sphere_point(index, total.max(1));
            // Cono ultra cerrado (90% dirección pura)
            [
                (direction[0] * 0.9 + random_dir[0] * 0.1) * base_speed,
                (direction[1] * 0.9 + random_dir[1] * 0.1) * base_speed,
                (direction[2] * 0.9 + random_dir[2] * 0.1) * base_speed,
            ]
        }
        DamagePreset::Implosion => {
            let base_speed = 400.0 * speed_scale;
            let dir = fibonacci_sphere_point(index, total.max(1));
            // La inversión se hace en generate_fragments_layered, aquí mandamos velocidad radial
            [dir[0] * base_speed, dir[1] * base_speed, dir[2] * base_speed]
        }
        DamagePreset::Cinematic => {
            let base_speed = 300.0 * speed_scale;
            let dir = fibonacci_sphere_point(index, total.max(1));
            [dir[0] * base_speed, dir[1] * base_speed, dir[2] * base_speed]
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// LÓGICA DE POSICIÓN POR PRESET
// ════════════════════════════════════════════════════════════════════════════

fn fragment_position(
    proxy:  &PhysicsProxy,
    preset: DamagePreset,
    cat:    FragmentCategory,
    index:  u32,
    total:  u32,
) -> [f32; 3] {
    let c = proxy.position;
    let e = proxy.half_ext;
    let t = index as f32 / total.max(1) as f32;

    match preset {
        // Explosión: distribución esférica Fibonacci (Mucho más épico y uniforme)
        DamagePreset::Explosion => {
            let dir = fibonacci_sphere_point(index, total.max(1));
            // Radio aleatorio dentro del volumen para relleno
            let r = (index as f32 % 10.0) / 10.0;
            [
                c[0] + dir[0] * e[0] * r,
                c[1] + dir[1] * e[1] * r,
                c[2] + dir[2] * e[2] * r,
            ]
        }

        // Colapso: los fragmentos empiezan en su posición estructural original
        // (distribuidos verticalmente dentro del proxy)
        DamagePreset::Collapse => {
            let layer = t * 2.0 - 1.0; // [-1, 1] de abajo a arriba
            [
                c[0] + (t - 0.5) * e[0],
                c[1] + (t * 2.0 * PI).cos() * e[1] * 0.5,
                c[2] + layer * e[2],
            ]
        }

        // Impacto: todos los fragmentos nacen cerca del punto de impacto (+X del proxy)
        DamagePreset::Impact => {
            let jitter = (t - 0.5) * e[1];
            [
                c[0] + e[0],         // en la cara de impacto
                c[1] + jitter,
                c[2] + jitter * 0.5,
            ]
        }
        // Shaped Charge: similar a impacto pero más concentrado
        DamagePreset::ShapedCharge => {
            let offset = (t - 0.5) * e[1] * 0.2; // área más pequeña
            [
                c[0] + e[0], 
                c[1] + offset,
                c[2] + offset * 0.5,
            ]
        }
        DamagePreset::Implosion | DamagePreset::Cinematic => {
            let dir = fibonacci_sphere_point(index, total.max(1));
            // Radio basado en el tamaño del objeto para que nazcan en una esfera
            let r = e[0].max(e[1]).max(e[2]) * 0.8; 
            [
                c[0] + dir[0] * r,
                c[1] + dir[1] * r,
                c[2] + dir[2] * r,
            ]
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// UTILIDADES
// ════════════════════════════════════════════════════════════════════════════

/// Distribución de Fibonacci en esfera: puntos casi-uniformes sobre la esfera unidad.
/// Mucho mejor que una cuadrícula o distribución aleatoria simple.
fn fibonacci_sphere_point(index: u32, total: u32) -> [f32; 3] {
    let golden_ratio = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let i = index as f32;
    let n = total as f32;

    let theta = (1.0 - 2.0 * (i / n)).acos();          // polar
    let phi   = 2.0 * PI * i / golden_ratio;             // azimutal

    [
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos(),
    ]
}

/// Energía cinética total estimada en Joules.
/// E = ½ · Σ(mᵢ · vᵢ²)
/// Usada por el enjambre para escalar la fuerza de separación inicial.
fn estimate_energy(proxy: &PhysicsProxy, preset: DamagePreset) -> f32 {
    let base_speed: f32 = match preset {
        DamagePreset::Explosion => 400.0,
        DamagePreset::Collapse  => 200.0,
        DamagePreset::Impact    => 600.0,
        DamagePreset::ShapedCharge => 1200.0,
        DamagePreset::Implosion => 400.0,
        DamagePreset::Cinematic => 300.0,
    };
    // E ≈ ½ · masa_total · velocidad_media²
    0.5 * proxy.mass * base_speed * base_speed
}
