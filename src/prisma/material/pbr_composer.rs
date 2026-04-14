use nalgebra::Vector3;

/// Material PBR completo
#[derive(Clone, Debug)]
pub struct PBRMaterial {
    pub name: String,
    
    // Albedo/Base Color
    pub base_color: Vector3<f32>,
    pub base_color_texture: Option<TextureReference>,
    
    // Metallic workflow
    pub metallic: f32,
    pub metallic_texture: Option<TextureReference>,
    
    // Roughness
    pub roughness: f32,
    pub roughness_texture: Option<TextureReference>,
    
    // Normal mapping
    pub normal_texture: Option<TextureReference>,
    pub normal_strength: f32,
    
    // Ambient Occlusion
    pub ao_texture: Option<TextureReference>,
    pub ao_strength: f32,
    
    // Emissive
    pub emissive_color: Vector3<f32>,
    pub emissive_strength: f32,
    pub emissive_texture: Option<TextureReference>,
    
    // Subsurface Scattering (para materiales translúcidos)
    pub subsurface_color: Option<Vector3<f32>>,
    pub subsurface_radius: f32,
    
    // Clearcoat (barniz)
    pub clearcoat: f32,
    pub clearcoat_roughness: f32,
    
    // Anisotropy (para metales cepillados)
    pub anisotropy: f32,
    pub anisotropy_rotation: f32,
    
    // Sheen (para telas)
    pub sheen: f32,
    pub sheen_tint: f32,
    
    // IOR (Index of Refraction)
    pub ior: f32,
    
    // Opacity
    pub opacity: f32,
    pub opacity_texture: Option<TextureReference>,
    
    // Displacement
    pub displacement_texture: Option<TextureReference>,
    pub displacement_scale: f32,
}

#[derive(Clone, Debug)]
pub struct TextureReference {
    pub path: String,
    pub uv_channel: usize,
    pub tiling: [f32; 2],
    pub offset: [f32; 2],
}

impl Default for PBRMaterial {
    fn default() -> Self {
        Self {
            name: "Default".into(),
            base_color: Vector3::new(0.8, 0.8, 0.8),
            base_color_texture: None,
            metallic: 0.0,
            metallic_texture: None,
            roughness: 0.5,
            roughness_texture: None,
            normal_texture: None,
            normal_strength: 1.0,
            ao_texture: None,
            ao_strength: 1.0,
            emissive_color: Vector3::zeros(),
            emissive_strength: 0.0,
            emissive_texture: None,
            subsurface_color: None,
            subsurface_radius: 0.0,
            clearcoat: 0.0,
            clearcoat_roughness: 0.03,
            anisotropy: 0.0,
            anisotropy_rotation: 0.0,
            sheen: 0.0,
            sheen_tint: 0.5,
            ior: 1.45,
            opacity: 1.0,
            opacity_texture: None,
            displacement_texture: None,
            displacement_scale: 0.0,
        }
    }
}

/// Compositor de materiales para Daithon
pub struct MaterialComposer {
    material_library: Vec<PBRMaterial>,
    _current_material_id: usize,
}

impl Default for MaterialComposer {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialComposer {
    pub fn new() -> Self {
        let mut composer = Self {
            material_library: Vec::new(),
            _current_material_id: 0,
        };

        // Cargar materiales predefinidos
        composer.load_default_materials();
        composer
    }

    fn load_default_materials(&mut self) {
        self.material_library.push(PBRMaterial {
            name: "Oak Wood".into(),
            base_color: Vector3::new(0.6, 0.4, 0.25),
            roughness: 0.7,
            metallic: 0.0,
            normal_strength: 0.5,
            ..Default::default()
        });

        self.material_library.push(PBRMaterial {
            name: "Brushed Steel".into(),
            base_color: Vector3::new(0.75, 0.75, 0.75),
            roughness: 0.3,
            metallic: 1.0,
            anisotropy: 0.8,
            ..Default::default()
        });

        self.material_library.push(PBRMaterial {
            name: "White Marble".into(),
            base_color: Vector3::new(0.95, 0.95, 0.92),
            roughness: 0.2,
            metallic: 0.0,
            subsurface_color: Some(Vector3::new(0.9, 0.9, 0.85)),
            subsurface_radius: 0.5,
            ..Default::default()
        });

        self.material_library.push(PBRMaterial {
            name: "Glass".into(),
            base_color: Vector3::new(1.0, 1.0, 1.0),
            roughness: 0.0,
            metallic: 0.0,
            opacity: 0.1,
            ior: 1.52,
            ..Default::default()
        });

        self.material_library.push(PBRMaterial {
            name: "Polished Concrete".into(),
            base_color: Vector3::new(0.5, 0.5, 0.5),
            roughness: 0.4,
            metallic: 0.0,
            ..Default::default()
        });

        self.material_library.push(PBRMaterial {
            name: "Oxidized Copper".into(),
            base_color: Vector3::new(0.45, 0.65, 0.6),
            roughness: 0.6,
            metallic: 0.8,
            ..Default::default()
        });

        self.material_library.push(PBRMaterial {
            name: "Translucent Resin".into(),
            base_color: Vector3::new(0.9, 0.95, 1.0),
            roughness: 0.1,
            metallic: 0.0,
            opacity: 0.4,
            subsurface_color: Some(Vector3::new(0.8, 0.9, 1.0)),
            subsurface_radius: 1.0,
            ..Default::default()
        });

        self.material_library.push(PBRMaterial {
            name: "Carbon Fiber".into(),
            base_color: Vector3::new(0.05, 0.05, 0.05),
            roughness: 0.25,
            metallic: 0.0,
            anisotropy: 0.6,
            clearcoat: 0.8,
            ..Default::default()
        });
    }

    pub fn get_material(&self, id: usize) -> Option<&PBRMaterial> {
        self.material_library.get(id)
    }

    pub fn get_material_by_name(&self, name: &str) -> Option<&PBRMaterial> {
        self.material_library.iter().find(|m| m.name == name)
    }

    pub fn list_materials(&self) -> Vec<String> {
        self.material_library.iter().map(|m| m.name.clone()).collect()
    }

    pub fn apply_material_to_mesh(
        &self,
        mesh: &mut crate::prisma::vertex::mesh_generator::HighQualityMesh,
        material_id: usize,
        slot_index: usize,
    ) {
        if let Some(slot) = mesh.material_slots.get_mut(slot_index) {
            slot.material_id = material_id;
        }
    }

    pub fn export_to_unreal_format(&self, material_id: usize) -> Option<String> {
        let mat = self.get_material(material_id)?;

        let mut code = format!(
            r#"
// Material: {}
// Generated by Daithon PRISMA

Material {{
    BaseColor: ({}, {}, {}),
    Metallic: {},
    Roughness: {},
    Normal: {},
    AmbientOcclusion: {},
"#,
            mat.name,
            mat.base_color.x, mat.base_color.y, mat.base_color.z,
            mat.metallic,
            mat.roughness,
            mat.normal_strength,
            mat.ao_strength,
        );

        if mat.emissive_strength > 0.0 {
            code.push_str(&format!(
                "    EmissiveColor: ({}, {}, {}),\n    EmissiveStrength: {},\n",
                mat.emissive_color.x, mat.emissive_color.y, mat.emissive_color.z,
                mat.emissive_strength,
            ));
        }

        if let Some(sss) = &mat.subsurface_color {
            code.push_str(&format!(
                "    SubsurfaceColor: ({}, {}, {}),\n    SubsurfaceRadius: {},\n",
                sss.x, sss.y, sss.z,
                mat.subsurface_radius,
            ));
        }

        code.push_str("}\n");

        Some(code)
    }
}
