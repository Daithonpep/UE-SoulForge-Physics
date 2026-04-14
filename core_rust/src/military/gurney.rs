/// La velocidad inicial de fragmentos NO es arbitraria
/// Gurney (1943) la predice con precisión sorprendente
pub struct GurneyModel {
    /// √(2E) de Gurney del explosivo (m/s)
    pub gurney_constant: f64,
    /// Ratio masa carcasa / masa explosivo
    pub mass_ratio: f64, // M/C
    /// Geometría del contenedor
    pub geometry: ChargeGeometry,
}

pub enum ChargeGeometry {
    Cylinder,      // Bomba típica
    Sphere,        // Granada
    Sandwich,      // Placa
    OpenFace,      // Shaped charge
}

impl GurneyModel {
    /// Velocidad inicial de fragmentos
    pub fn fragment_velocity(&self) -> f64 {
        let mc = self.mass_ratio; // M/C

        match self.geometry {
            ChargeGeometry::Sphere => {
                // V = √(2E) / √(3/5 + M/C)
                self.gurney_constant / (0.6 + mc).sqrt()
            }
            ChargeGeometry::Cylinder => {
                // V = √(2E) / √(1/2 + M/C)
                self.gurney_constant / (0.5 + mc).sqrt()
            }
            ChargeGeometry::Sandwich => {
                // V = √(2E) / √(1/3 + M/C)
                self.gurney_constant / (1.0 / 3.0 + mc).sqrt()
            }
            ChargeGeometry::OpenFace => {
                // V = √(2E) / √(1/3 + M/C)  (con corrección)
                self.gurney_constant / (1.0 / 3.0 + mc).sqrt() * 0.9
            }
        }
    }

    /// Distribución angular de velocidad
    pub fn velocity_at_angle(&self, angle_from_equator_rad: f64) -> f64 {
        let v0 = self.fragment_velocity();
        match self.geometry {
            ChargeGeometry::Cylinder => {
                v0 * angle_from_equator_rad.cos()
            }
            ChargeGeometry::Sphere => {
                v0 * (1.0 - 0.1 * (1.0 - angle_from_equator_rad.cos()))
            }
            _ => v0,
        }
    }
}
