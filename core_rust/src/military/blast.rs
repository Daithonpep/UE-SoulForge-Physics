use super::Vec3;

/// En vez de una simple esfera de presión, modela la física real de detonación
pub struct DetonationWave {
    /// Velocidad de detonación Chapman-Jouguet (m/s)
    pub velocity_cj: f64,
    /// Presión CJ del explosivo
    pub pressure_cj: f64,
    /// Perfil de Taylor: la presión NO es uniforme detrás del frente
    pub taylor_wave: TaylorExpansion,
}

pub struct TaylorExpansion {
    /// La presión decae detrás del frente de onda según Taylor
    /// P(r) = P_cj * (r / R_front)^n  donde n depende de la geometría
    pub exponent: f64, // ~3 para esférica, ~2 para cilíndrica
}

/// El frente de onda NO es instantáneo - tiene grosor finito
pub struct ShockFront {
    pub thickness: f64,          // Típicamente ~micrómetros pero importa para la física
    pub pressure_ratio: f64,     // Rankine-Hugoniot
    pub density_ratio: f64,
    pub temperature_ratio: f64,
    pub mach_number: f64,
}

impl ShockFront {
    /// Relaciones de Rankine-Hugoniot - esto es la física REAL del choque
    pub fn from_mach(mach: f64, gamma: f64) -> Self {
        let m2 = mach * mach;
        let pressure_ratio = 1.0 + (2.0 * gamma / (gamma + 1.0)) * (m2 - 1.0);
        let density_ratio = ((gamma + 1.0) * m2) / ((gamma - 1.0) * m2 + 2.0);
        let temperature_ratio = pressure_ratio / density_ratio;

        Self {
            thickness: 1e-6,
            pressure_ratio,
            density_ratio,
            temperature_ratio,
            mach_number: mach,
        }
    }
}

/// Predicción de sobrepresión según distancia escalada
/// Kingery-Bulmash es el estándar de la industria
pub struct KingeryBulmash;

impl KingeryBulmash {
    /// Distancia escalada Z = R / W^(1/3)
    /// R en metros, W en kg de TNT equivalente
    pub fn scaled_distance(distance_m: f64, charge_kg_tnt: f64) -> f64 {
        distance_m / charge_kg_tnt.cbrt().max(1e-6)
    }

    /// Sobrepresión pico (Pa) - ajuste polinomial de datos experimentales
    pub fn peak_overpressure(distance_m: f64, charge_kg_tnt: f64) -> f64 {
        let z = Self::scaled_distance(distance_m, charge_kg_tnt);
        let log_z = z.ln();

        if z < 0.05 {
            // Muy cerca - detonación
            return 1e9; // Limitado a ~1 GPa
        }

        // Polinomio de Kingery para campo libre (ajustado)
        let log_p = 7.2106
            - 1.5685 * log_z
            - 0.1882 * log_z.powi(2)
            + 0.0241 * log_z.powi(3)
            + 0.0049 * log_z.powi(4);

        log_p.exp()
    }

    /// Impulso positivo (Pa·s)
    pub fn positive_impulse(distance_m: f64, charge_kg_tnt: f64) -> f64 {
        let z = Self::scaled_distance(distance_m, charge_kg_tnt);
        let w_third = charge_kg_tnt.cbrt();

        // Impulso escalado
        let log_z = z.ln();
        let log_is = 5.522
            - 0.8757 * log_z
            - 0.1349 * log_z.powi(2)
            + 0.0112 * log_z.powi(3);

        log_is.exp() * w_third // Desescalar
    }

    /// Duración de fase positiva (s)
    pub fn positive_duration(distance_m: f64, charge_kg_tnt: f64) -> f64 {
        let z = Self::scaled_distance(distance_m, charge_kg_tnt);
        let w_third = charge_kg_tnt.cbrt();

        let log_z = z.ln();
        let log_td = -2.573
            + 0.3208 * log_z
            + 0.1302 * log_z.powi(2)
            - 0.0215 * log_z.powi(3);

        log_td.exp() * w_third
    }

    /// Perfil temporal de presión (Friedlander modificado)
    pub fn pressure_at_time(
        distance_m: f64,
        charge_kg_tnt: f64,
        time_after_arrival: f64,
    ) -> f64 {
        let ps = Self::peak_overpressure(distance_m, charge_kg_tnt);
        let td = Self::positive_duration(distance_m, charge_kg_tnt);

        if time_after_arrival < 0.0 || time_after_arrival > td {
            return 0.0; // Simplificación: ignora fase negativa
        }

        let t_ratio = time_after_arrival / td;

        // Forma de onda de Friedlander
        // P(t) = Ps * (1 - t/td) * exp(-α * t/td)
        let alpha = 1.5; // Parámetro de decaimiento (varía con Z)
        ps * (1.0 - t_ratio) * (-alpha * t_ratio).exp()
    }

    /// Tiempo de llegada de la onda
    pub fn arrival_time(distance_m: f64, charge_kg_tnt: f64) -> f64 {
        let z = Self::scaled_distance(distance_m, charge_kg_tnt);
        let w_third = charge_kg_tnt.cbrt();

        let log_z = z.ln();
        let log_ta = -0.4677
            + 1.1975 * log_z
            + 0.1468 * log_z.powi(2)
            - 0.0108 * log_z.powi(3);

        log_ta.exp() * w_third
    }
}

/// Cuando una onda de choque golpea una superficie, no solo "rebota"
/// A ciertos ángulos forma un tallo de Mach que es MUCHO más destructivo
pub struct BlastSurfaceInteraction {
    pub incident_overpressure: f64,
    pub surface_normal: Vec3,
    pub blast_direction: Vec3,
}

impl BlastSurfaceInteraction {
    /// Ángulo de incidencia
    pub fn incidence_angle(&self) -> f64 {
        self.blast_direction.dot(&self.surface_normal).acos()
    }

    /// Determina tipo de reflexión
    pub fn reflection_type(&self) -> ReflectionType {
        let angle = self.incidence_angle().to_degrees();

        // El ángulo crítico depende de la sobrepresión
        let critical_angle = self.critical_angle();

        if angle < critical_angle {
            // Reflexión regular - factor de reflexión ~2-8x
            let reflection_factor = self.regular_reflection_factor(angle);
            ReflectionType::Regular { reflection_factor }
        } else {
            // Reflexión de Mach - forma triple punto
            let mach_stem_height = self.mach_stem_height(angle);
            let enhanced_pressure = self.mach_stem_pressure();
            ReflectionType::MachStem {
                stem_height: mach_stem_height,
                enhanced_pressure,
            }
        }
    }

    fn critical_angle(&self) -> f64 {
        let p_ratio = self.incident_overpressure / 101325.0; // Normalizado a 1 atm
        (39.23 * (-0.04 * p_ratio).exp() + 10.0).clamp(10.0, 40.0)
    }

    fn regular_reflection_factor(&self, angle_deg: f64) -> f64 {
        let base = 2.0;
        let enhancement = 1.0 + 6.0 * (angle_deg / self.critical_angle().max(1.0)).powi(3);
        base * enhancement
    }

    fn mach_stem_pressure(&self) -> f64 {
        self.incident_overpressure * 2.5
    }

    fn mach_stem_height(&self, angle_deg: f64) -> f64 {
        let excess_angle = angle_deg - self.critical_angle();
        0.1 * excess_angle.to_radians().tan()
    }
}

pub enum ReflectionType {
    Regular { reflection_factor: f64 },
    MachStem { stem_height: f64, enhanced_pressure: f64 },
}

pub struct TntEquivalent;

impl TntEquivalent {
    pub fn factor(explosive: &str) -> f64 {
        match explosive {
            "TNT"       => 1.00,
            "C4"        => 1.34,
            "RDX"       => 1.60,
            "PETN"      => 1.66,
            "Semtex"    => 1.35,
            "ANFO"      => 0.82,
            "Dynamite"  => 0.75,
            "HMX"       => 1.70,
            "TATP"      => 0.55,
            "BlackPow"  => 0.40,
            _ => 1.0,
        }
    }
}
