/// Tracto vocal modelado como tubo de sección variable usando propagación de ondas bidireccional.
pub struct PhysicalVocalTract {
    #[allow(dead_code)]
    sample_rate: f64,
    num_sections: usize,
    areas: Vec<f64>,
    forward_wave: Vec<f64>,
    backward_wave: Vec<f64>,
    reflection_coefficients: Vec<f64>,
    wall_losses: Vec<f64>,
    nasal_coupling: NasalCoupling,
    #[allow(dead_code)]
    tract_length: f64,
}

struct NasalCoupling {
    velum_opening: f64,
    nasal_areas: Vec<f64>,
    nasal_forward: Vec<f64>,
    nasal_backward: Vec<f64>,
    nasal_reflections: Vec<f64>,
}

impl PhysicalVocalTract {
    pub fn new(sample_rate: f64) -> Self {
        // 44 oral + 20 subglotal = 64 secciones
        let num_sections = 64;
        let subglottal_sections = 20;
        let tract_length = 22.0; // Extendemos longitud total
        
        let mut areas = vec![10.0; subglottal_sections]; // El pecho es ancho
        areas.extend(vec![4.0; 44]); // Tracto oral inicial neutral

        let mut tract = Self {
            sample_rate,
            num_sections,
            areas: areas.clone(),
            forward_wave: vec![0.0; num_sections],
            backward_wave: vec![0.0; num_sections],
            reflection_coefficients: vec![0.0; num_sections - 1],
            wall_losses: vec![0.0; num_sections],
            nasal_coupling: NasalCoupling {
                velum_opening: 0.0,
                nasal_areas: vec![2.0; 28],
                nasal_forward: vec![0.0; 28],
                nasal_backward: vec![0.0; 28],
                nasal_reflections: vec![0.0; 27],
            },
            tract_length,
        };

        tract.calculate_reflections();
        tract.calculate_wall_losses();
        tract
    }

    pub fn set_shape(&mut self, shape: &TractShape) {
        // Solo modificamos las 44 secciones orales (índices 20 a 63)
        for i in 20..self.num_sections {
            let position = (i - 20) as f64 / 44.0;
            let area = self.calculate_area_at_position(position, shape);
            self.areas[i] = area.max(0.01);
        }
        self.calculate_reflections();
        self.nasal_coupling.velum_opening = shape.velum_opening;
    }

    fn calculate_area_at_position(&self, position: f64, shape: &TractShape) -> f64 {
        if position < 0.15 {
            let pharynx_width = 2.0 + shape.tongue_root * 3.0;
            pharynx_width * pharynx_width * 0.5
        } else if position < 0.4 {
            let t = (position - 0.15) / 0.25;
            let tongue_effect = shape.tongue_body_height * (1.0 - t);
            let base = 3.0 + shape.jaw_opening * 2.0;
            (base - tongue_effect * 2.5).max(0.05)
        } else if position < 0.7 {
            let t = (position - 0.4) / 0.3;
            let tongue_pos = shape.tongue_body_position;
            let distance_from_tongue = (t - tongue_pos).abs();
            let constriction = shape.tongue_body_height * (-distance_from_tongue * distance_from_tongue * 20.0).exp();
            let base = 2.5 + shape.jaw_opening * 2.5;
            (base - constriction * 3.0).max(0.02)
        } else if position < 0.9 {
            let t = (position - 0.7) / 0.2;
            let tip_effect = shape.tongue_tip_height * (1.0 - t);
            let base = 2.0 + shape.jaw_opening * 2.0;
            (base - tip_effect * 2.0).max(0.05)
        } else {
            let lip_area = 0.5 + shape.lip_opening * 3.0;
            let lip_rounding = 1.0 - shape.lip_rounding * 0.5;
            lip_area * lip_rounding
        }.max(0.01)
    }

    fn calculate_reflections(&mut self) {
        for i in 0..self.num_sections - 1 {
            let a_l = self.areas[i];
            let a_r = self.areas[i + 1];
            self.reflection_coefficients[i] = (a_r - a_l) / (a_r + a_l);
        }
        for i in 0..self.nasal_coupling.nasal_reflections.len() {
            let a_l = self.nasal_coupling.nasal_areas[i];
            let a_r = self.nasal_coupling.nasal_areas[i+1];
            self.nasal_coupling.nasal_reflections[i] = (a_r - a_l) / (a_r + a_l);
        }
    }

    fn calculate_wall_losses(&mut self) {
        for i in 0..self.num_sections {
            let area = self.areas[i];
            let circumference = (area * std::f64::consts::PI * 4.0).sqrt();
            // Factor de pérdida: más pérdida en tubos estrechos
            // Aumentamos la constante de pérdida para mayor suavidad (0.0003 -> 0.0008)
            self.wall_losses[i] = 1.0 - (circumference * 0.0008).min(0.05);
        }
    }

    pub fn propagate(&mut self, glottal_input: f64) -> f64 {
        // Inyectamos la glotis en el punto de unión entre pecho y boca (índice 20)
        self.forward_wave[20] += glottal_input * 0.5;
        self.backward_wave[20] += glottal_input * -0.5; // La glotis radia hacia atrás también

        let mut n_f = vec![0.0; self.num_sections];
        let mut n_b = vec![0.0; self.num_sections];

        for i in 0..self.num_sections - 1 {
            let k = self.reflection_coefficients[i];
            let f_in = self.forward_wave[i];
            let b_in = self.backward_wave[i+1];
            n_f[i+1] = (1.0 + k) * f_in + k * b_in;
            n_b[i] = -k * f_in + (1.0 - k) * b_in;
            n_f[i+1] *= self.wall_losses[i+1];
            n_b[i] *= self.wall_losses[i];
        }

        // Labios (Lip output)
        let l_area = self.areas[self.num_sections - 1];
        let l_refl = (l_area - 8.0).max(-0.9).min(0.9) / (l_area + 8.0).max(0.1);
        let l_out = self.forward_wave[self.num_sections - 1] * (1.0 + l_refl);
        n_b[self.num_sections - 1] = self.forward_wave[self.num_sections - 1] * l_refl;
        
        // Pulmones (Cierre extremo posterior con absorción)
        n_f[0] = self.backward_wave[0] * 0.8; // Reflexión de pulmón

        let n_out = if self.nasal_coupling.velum_opening > 0.01 {
            self.propagate_nasal(&mut n_f, &mut n_b)
        } else { 0.0 };

        self.forward_wave = n_f;
        self.backward_wave = n_b;
        l_out + n_out * self.nasal_coupling.velum_opening
    }

    fn propagate_nasal(&mut self, m_f: &mut Vec<f64>, _m_b: &mut Vec<f64>) -> f64 {
        // El velum está al final del tracto oral (al ~75% de las 44 secciones orales)
        let cp = 20 + (44 * 3 / 4);
        let c = self.nasal_coupling.velum_opening;
        let e_to_n = m_f[cp] * c * 0.5;
        m_f[cp] *= 1.0 - c * 0.5;

        let n_secs = self.nasal_coupling.nasal_areas.len();
        self.nasal_coupling.nasal_forward[0] += e_to_n;
        for i in 0..n_secs - 1 {
            let k = self.nasal_coupling.nasal_reflections[i];
            let fw = self.nasal_coupling.nasal_forward[i];
            let bw = self.nasal_coupling.nasal_backward[i+1];
            self.nasal_coupling.nasal_forward[i+1] = (1.0 + k) * fw + k * bw;
            self.nasal_coupling.nasal_backward[i] = -k * fw + (1.0 - k) * bw;
            self.nasal_coupling.nasal_forward[i+1] *= 0.985;
            self.nasal_coupling.nasal_backward[i] *= 0.985;
        }
        let n_out = self.nasal_coupling.nasal_forward[n_secs-1] * 0.6;
        self.nasal_coupling.nasal_backward[n_secs-1] = self.nasal_coupling.nasal_forward[n_secs-1] * 0.3;
        n_out
    }
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TractShape {
    pub jaw_opening: f64,
    pub tongue_body_height: f64,
    pub tongue_body_position: f64,
    pub tongue_tip_height: f64,
    pub tongue_root: f64,
    pub lip_opening: f64,
    pub lip_rounding: f64,
    pub velum_opening: f64,
    pub constriction_position: f64,
    pub constriction_degree: f64,
}

impl Default for TractShape {
    fn default() -> Self {
        Self {
            jaw_opening: 0.5,
            tongue_body_height: 0.5,
            tongue_body_position: 0.5,
            tongue_tip_height: 0.3,
            tongue_root: 0.5,
            lip_opening: 0.5,
            lip_rounding: 0.0,
            velum_opening: 0.0,
            constriction_position: 0.5,
            constriction_degree: 0.0,
        }
    }
}
