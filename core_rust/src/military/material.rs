/// Modelo constitutivo Johnson-Cook para materiales bajo impacto
#[derive(Debug, Clone)]
pub struct JohnsonCookMaterial {
    // Parámetros de fluencia
    pub a: f64,  // Límite elástico estático (Pa)
    pub b: f64,  // Coeficiente de endurecimiento
    pub n: f64,  // Exponente de endurecimiento
    pub c: f64,  // Coeficiente de sensibilidad a tasa de deformación
    pub m: f64,  // Exponente térmico
    pub id: u16, // Material ID para render y colisón

    // Propiedades del material
    pub density: f64,
    pub bulk_modulus: f64,
    pub shear_modulus: f64,
    pub melting_temperature: f64,
    pub room_temperature: f64,
    pub specific_heat: f64,

    // Parámetros de daño Johnson-Cook
    pub d1: f64, pub d2: f64, pub d3: f64, pub d4: f64, pub d5: f64,
    
    // Constantes de penetración (Thor)
    pub k_constant: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

impl JohnsonCookMaterial {
    pub fn steel_4340() -> Self {
        Self {
            a: 792e6, b: 510e6, n: 0.26, c: 0.014, m: 1.03, id: 0,
            density: 7830.0,
            bulk_modulus: 159e9,
            shear_modulus: 77e9,
            melting_temperature: 1793.0,
            room_temperature: 293.0,
            specific_heat: 477.0,
            d1: 0.05, d2: 3.44, d3: -2.12, d4: 0.002, d5: 0.61,
            k_constant: 1350.0, alpha: 0.906, beta: 0.0, gamma: 0.359,
        }
    }

    pub fn aluminum_6061() -> Self {
        Self {
            a: 324e6, b: 114e6, n: 0.42, c: 0.002, m: 1.34, id: 2, // usando aluminum para madera
            density: 2703.0,
            bulk_modulus: 69e9,
            shear_modulus: 26e9,
            melting_temperature: 925.0,
            room_temperature: 293.0,
            specific_heat: 885.0,
            d1: -0.77, d2: 1.45, d3: -0.47, d4: 0.0, d5: 1.6,
            k_constant: 1090.0, alpha: 0.903, beta: 0.0, gamma: 0.381,
        }
    }

    pub fn concrete_35mpa() -> Self {
        Self {
            a: 35e6, b: 2e6, n: 0.5, c: 0.007, m: 0.9, id: 1, // concreto
            density: 2400.0,
            bulk_modulus: 14e9,
            shear_modulus: 11e9,
            melting_temperature: 1500.0,
            room_temperature: 293.0,
            specific_heat: 880.0,
            d1: 0.04, d2: 1.0, d3: -1.5, d4: 0.0, d5: 0.5,
            k_constant: 800.0, alpha: 1.0, beta: 0.0, gamma: 0.5, // Estimados para concreto
        }
    }

    pub fn flow_stress(&self, plastic_strain: f64, strain_rate: f64, temperature: f64) -> f64 {
        let ref_strain_rate = 1.0;
        let strain_term = self.a + self.b * plastic_strain.powf(self.n);
        let rate_term = if strain_rate > ref_strain_rate {
            1.0 + self.c * (strain_rate / ref_strain_rate).ln()
        } else {
            1.0
        };
        let t_star = ((temperature - self.room_temperature) / (self.melting_temperature - self.room_temperature)).clamp(0.0, 1.0);
        let thermal_term = 1.0 - t_star.powf(self.m);
        strain_term * rate_term * thermal_term
    }

    pub fn fracture_strain(&self, stress_triaxiality: f64, strain_rate: f64, temperature: f64) -> f64 {
        let ref_strain_rate = 1.0;
        let t_star = ((temperature - self.room_temperature) / (self.melting_temperature - self.room_temperature)).clamp(0.0, 1.0);
        let triax_term = self.d1 + self.d2 * (self.d3 * stress_triaxiality).exp();
        let rate_term = if strain_rate > ref_strain_rate {
            1.0 + self.d4 * (strain_rate / ref_strain_rate).ln()
        } else {
            1.0
        };
        let thermal_term = 1.0 + self.d5 * t_star;
        triax_term * rate_term * thermal_term
    }
}
