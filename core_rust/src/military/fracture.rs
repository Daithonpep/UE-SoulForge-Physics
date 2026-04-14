use rand::Rng;
use super::Vec3;

/// Fragmentación basada en energía de Grady
/// Los fragmentos NO se generan aleatoriamente - siguen distribuciones físicas
pub struct GradyFragmentation {
    /// Tenacidad a la fractura del material (Pa·√m)
    pub fracture_toughness: f64,  // K_IC
    /// Densidad del material (kg/m3)
    pub density: f64,
    /// Velocidad del sonido en el material (m/s)
    pub sound_speed: f64,
    /// Tasa de deformación (1/s)
    pub strain_rate: f64,
    /// Masa total a fragmentar
    pub total_mass: f64,
}

impl GradyFragmentation {
    /// Tamaño promedio de fragmento según Grady (1982)
    pub fn average_fragment_size(&self) -> f64 {
        // s = (√(20) * K_IC / (ρ * c * ε̇))^(2/3)
        let numerator = 20.0_f64.sqrt() * self.fracture_toughness;
        let denominator = self.density * self.sound_speed * self.strain_rate.max(1e-6);
        (numerator / denominator).powf(2.0 / 3.0)
    }

    /// Distribución de Mott para tamaños de fragmento
    pub fn mott_distribution(&self, rng: &mut impl Rng) -> Vec<Fragment> {
        let avg_size = self.average_fragment_size();
        let mu = avg_size; // Parámetro de escala de Mott

        let mut fragments = Vec::new();
        let mut remaining_mass = self.total_mass;
        let min_mass = self.total_mass * 0.001; // 0.1% de la masa total

        while remaining_mass > min_mass && fragments.len() < 1000 {
            // Distribución de Mott: N(m) = N0 * exp(-√(m/μ))
            let u: f64 = rng.gen_range(0.001..1.0);
            let mass = mu * (-u.ln()).powi(2);
            let mass = mass.min(remaining_mass);

            fragments.push(Fragment {
                mass,
                size: (mass / self.density).powf(1.0/3.0), // Aproximación cúbica
                velocity: Vec3::ZERO,
                angular_velocity: Vec3::ZERO,
                shape: FragmentShape::Irregular { cd_estimate: 1.2 },
                temperature: 293.0,
            });

            remaining_mass -= mass;
        }

        fragments
    }
}

pub struct Fragment {
    pub mass: f64,
    pub size: f64,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub shape: FragmentShape,
    pub temperature: f64,
}

pub enum FragmentShape {
    Plate { aspect_ratio: f64 },
    Cube,
    Sphere,
    Irregular { cd_estimate: f64 },
}
