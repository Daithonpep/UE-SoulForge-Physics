use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Módulo Python para interfaz con Unreal Engine
#[pymodule]
fn nexus_d_py(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> PyResult<()> {
    m.add_class::<PyNexusController>()?;
    Ok(())
}

#[pyclass]
pub struct PyNexusController {
    genesis: crate::nexus_d::genesis_enhanced::GenesisEnhancedLoop,
}

#[pymethods]
impl PyNexusController {
    #[new]
    fn new(population_size: usize) -> Self {
        Self {
            genesis: crate::nexus_d::genesis_enhanced::GenesisEnhancedLoop::new(population_size),
        }
    }

    /// Cambiar estilo de diseño
    fn set_style(&mut self, profile_name: &str) -> PyResult<()> {
        self.genesis.set_style(profile_name)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Obtener perfiles disponibles
    fn available_profiles(&self) -> Vec<String> {
        self.genesis.available_profiles()
    }

    /// Evaluar estética de un mesh exportado desde Unreal
    fn evaluate_mesh_aesthetic<'py>(
        &self,
        py: Python<'py>,
        _vertices: Vec<[f64; 3]>,
        _contact_z_threshold: f64,
    ) -> PyResult<pyo3::Bound<'py, pyo3::types::PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("total_score", 0.75)?; // Placeholder
        dict.set_item("golden_ratio", 0.8)?;
        dict.set_item("symmetry", 0.6)?;
        dict.set_item("minimalism", 0.9)?;
        Ok(dict)
    }

    /// Evaluar equilibrio de un diseño
    fn evaluate_balance<'py>(
        &self,
        py: Python<'py>,
        com: [f64; 3],
        mass: f64,
        contact_points: Vec<[f64; 3]>,
    ) -> PyResult<pyo3::Bound<'py, pyo3::types::PyDict>> {
        let dict = PyDict::new(py);

        let contacts: Vec<nalgebra::Point3<f64>> = contact_points
            .iter()
            .map(|p| nalgebra::Point3::new(p[0], p[1], p[2]))
            .collect();

        let support_poly: Vec<[f64; 2]> = contacts
            .iter()
            .map(|p| [p.x, p.y])
            .collect();

        let helix = crate::nexus_d::helix::balance_dynamics::HelixBalanceSystem::new(1.0);
        let com_point = nalgebra::Point3::new(com[0], com[1], com[2]);
        let analysis = helix.analyze(&com_point, mass, &contacts, &support_poly);

        dict.set_item("is_stable", analysis.is_stable)?;
        dict.set_item("stability_factor", analysis.stability_factor)?;
        dict.set_item("balance_score", analysis.balance_score)?;
        dict.set_item("tipping_force", analysis.tipping_force_required)?;

        Ok(dict)
    }
}
