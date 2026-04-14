use nalgebra::{Point3, Vector3};

/// Configuración de iluminación global
#[derive(Clone, Debug)]
pub struct RadianceConfig {
    pub enable_gi: bool,
    pub gi_bounces: usize,
    pub gi_intensity: f32,
    pub probe_resolution: usize,
    pub soft_shadows: bool,
    pub shadow_quality: usize,
    pub enable_ssao: bool,
    pub ssao_radius: f32,
    pub enable_ssr: bool,
    pub enable_bloom: bool,
    pub bloom_threshold: f32,
    pub bloom_intensity: f32,
}

impl Default for RadianceConfig {
    fn default() -> Self {
        Self {
            enable_gi: true,
            gi_bounces: 2,
            gi_intensity: 1.0,
            probe_resolution: 32,
            soft_shadows: true,
            shadow_quality: 16,
            enable_ssao: true,
            ssao_radius: 0.5,
            enable_ssr: true,
            enable_bloom: true,
            bloom_threshold: 1.0,
            bloom_intensity: 0.5,
        }
    }
}

/// Fuente de luz
#[derive(Clone, Debug)]
pub struct Light {
    pub light_type: LightType,
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub color: Vector3<f32>,
    pub intensity: f32,
    pub range: f32,
    pub cast_shadows: bool,
}

#[derive(Clone, Debug)]
pub enum LightType {
    Directional,
    Point,
    Spot { inner_angle: f32, outer_angle: f32 },
    Area { width: f32, height: f32 },
    Skylight,
}

pub struct RadianceLightingEngine {
    config: RadianceConfig,
    lights: Vec<Light>,
    ambient_color: Vector3<f32>,
    _skybox_intensity: f32,
}

impl Default for RadianceLightingEngine {
    fn default() -> Self {
        Self::new(RadianceConfig::default())
    }
}

impl RadianceLightingEngine {
    pub fn new(config: RadianceConfig) -> Self {
        let mut engine = Self {
            config,
            lights: Vec::new(),
            ambient_color: Vector3::new(0.1, 0.1, 0.1),
            _skybox_intensity: 1.0,
        };

        engine.setup_studio_lighting();
        engine
    }

    fn setup_studio_lighting(&mut self) {
        self.lights.push(Light {
            light_type: LightType::Directional,
            position: Point3::new(5.0, 5.0, 5.0),
            direction: Vector3::new(-1.0, -1.0, -1.0).normalize(),
            color: Vector3::new(1.0, 0.98, 0.95),
            intensity: 5.0,
            range: f32::MAX,
            cast_shadows: true,
        });

        self.lights.push(Light {
            light_type: LightType::Point,
            position: Point3::new(-3.0, 2.0, 3.0),
            direction: Vector3::zeros(),
            color: Vector3::new(0.9, 0.95, 1.0),
            intensity: 2.0,
            range: 10.0,
            cast_shadows: false,
        });

        self.lights.push(Light {
            light_type: LightType::Spot {
                inner_angle: 30.0,
                outer_angle: 45.0,
            },
            position: Point3::new(0.0, 3.0, -5.0),
            direction: Vector3::new(0.0, -0.3, 1.0).normalize(),
            color: Vector3::new(1.0, 1.0, 1.0),
            intensity: 3.0,
            range: 15.0,
            cast_shadows: false,
        });

        self.lights.push(Light {
            light_type: LightType::Skylight,
            position: Point3::origin(),
            direction: Vector3::new(0.0, 1.0, 0.0),
            color: Vector3::new(0.5, 0.7, 1.0),
            intensity: 1.5,
            range: f32::MAX,
            cast_shadows: false,
        });
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn set_ambient(&mut self, color: Vector3<f32>) {
        self.ambient_color = color;
    }

    pub fn calculate_lighting_at_point(
        &self,
        position: &Point3<f32>,
        normal: &Vector3<f32>,
        view_dir: &Vector3<f32>,
        material_metallic: f32,
        material_roughness: f32,
    ) -> Vector3<f32> {
        let mut total_light = self.ambient_color;

        for light in &self.lights {
            let light_contribution = match &light.light_type {
                LightType::Directional => {
                    self.calculate_directional_light(
                        &light.direction,
                        &light.color,
                        light.intensity,
                        normal,
                        view_dir,
                        material_metallic,
                        material_roughness,
                    )
                }
                LightType::Point => {
                    let light_vec = light.position - position;
                    let distance = light_vec.norm();
                    
                    if distance > light.range {
                        Vector3::zeros()
                    } else {
                        let light_dir = light_vec.normalize();
                        let attenuation = self.calculate_attenuation(distance, light.range);
                        
                        self.calculate_directional_light(
                            &light_dir,
                            &light.color,
                            light.intensity * attenuation,
                            normal,
                            view_dir,
                            material_metallic,
                            material_roughness,
                        )
                    }
                }
                LightType::Spot { inner_angle, outer_angle } => {
                    let light_vec = light.position - position;
                    let distance = light_vec.norm();
                    
                    if distance > light.range {
                        Vector3::zeros()
                    } else {
                        let light_dir = light_vec.normalize();
                        let spot_effect = self.calculate_spot_effect(
                            &light_dir,
                            &light.direction,
                            *inner_angle,
                            *outer_angle,
                        );
                        
                        if spot_effect > 0.0 {
                            let attenuation = self.calculate_attenuation(distance, light.range);
                            
                            self.calculate_directional_light(
                                &light_dir,
                                &light.color,
                                light.intensity * attenuation * spot_effect,
                                normal,
                                view_dir,
                                material_metallic,
                                material_roughness,
                            )
                        } else {
                            Vector3::zeros()
                        }
                    }
                }
                LightType::Area { .. } => {
                    Vector3::zeros()
                }
                LightType::Skylight => {
                    light.color * light.intensity * normal.y.max(0.0)
                }
            };

            total_light += light_contribution;
        }

        total_light
    }

    fn calculate_directional_light(
        &self,
        light_dir: &Vector3<f32>,
        light_color: &Vector3<f32>,
        intensity: f32,
        normal: &Vector3<f32>,
        view_dir: &Vector3<f32>,
        metallic: f32,
        roughness: f32,
    ) -> Vector3<f32> {
        let n_dot_l = normal.dot(light_dir).max(0.0);

        if n_dot_l <= 0.0 {
            return Vector3::zeros();
        }

        let halfway = (light_dir + view_dir).normalize();
        let n_dot_h = normal.dot(&halfway).max(0.0);
        let n_dot_v = normal.dot(view_dir).max(0.0);

        let f0 = Vector3::new(0.04, 0.04, 0.04).lerp(light_color, metallic);
        let f = f0 + (Vector3::new(1.0, 1.0, 1.0) - f0) * (1.0 - n_dot_v).powi(5);

        let alpha = roughness * roughness;
        let alpha_sq = alpha * alpha;
        let denom = n_dot_h * n_dot_h * (alpha_sq - 1.0) + 1.0;
        let d = alpha_sq / (std::f32::consts::PI * denom * denom);

        let k = (roughness + 1.0).powi(2) / 8.0;
        let g = (n_dot_l / (n_dot_l * (1.0 - k) + k)) 
              * (n_dot_v / (n_dot_v * (1.0 - k) + k));

        let specular = (f * d * g) / (4.0 * n_dot_l * n_dot_v + 0.0001);

        let diffuse = light_color.component_mul(&(Vector3::new(1.0, 1.0, 1.0) - f)) 
                    * (1.0 - metallic) 
                    / std::f32::consts::PI;

        (diffuse + specular) * intensity * n_dot_l
    }

    fn calculate_attenuation(&self, distance: f32, range: f32) -> f32 {
        let ratio = distance / range;
        let ratio_sq = ratio * ratio;
        ((1.0 - ratio_sq).max(0.0)).powi(2)
    }

    fn calculate_spot_effect(
        &self,
        light_dir: &Vector3<f32>,
        spot_dir: &Vector3<f32>,
        inner_angle: f32,
        outer_angle: f32,
    ) -> f32 {
        let cos_angle = (-light_dir).dot(spot_dir);
        let cos_inner = (inner_angle.to_radians() / 2.0).cos();
        let cos_outer = (outer_angle.to_radians() / 2.0).cos();

        if cos_angle > cos_inner {
            1.0
        } else if cos_angle > cos_outer {
            (cos_angle - cos_outer) / (cos_inner - cos_outer)
        } else {
            0.0
        }
    }

    pub fn export_to_unreal(&self) -> String {
        let mut code = String::from("// Radiance Lighting Setup\n\n");

        for (i, light) in self.lights.iter().enumerate() {
            code.push_str(&format!("// Light {}\n", i));
            match &light.light_type {
                LightType::Directional => {
                    code.push_str(&format!(
                        "DirectionalLight {{\n  Intensity: {},\n  Color: ({}, {}, {}),\n  CastShadows: {},\n}}\n\n",
                        light.intensity,
                        light.color.x, light.color.y, light.color.z,
                        light.cast_shadows,
                    ));
                }
                LightType::Point => {
                    code.push_str(&format!(
                        "PointLight {{\n  Position: ({}, {}, {}),\n  Intensity: {},\n  Range: {},\n}}\n\n",
                        light.position.x, light.position.y, light.position.z,
                        light.intensity,
                        light.range,
                    ));
                }
                LightType::Skylight => {
                    code.push_str(&format!(
                        "SkyLight {{\n  Intensity: {},\n  Color: ({}, {}, {}),\n}}\n\n",
                        light.intensity,
                        light.color.x, light.color.y, light.color.z,
                    ));
                }
                _ => {}
            }
        }

        code.push_str(&format!(
            "// Post Process\nPostProcess {{\n  BloomIntensity: {},\n  AmbientOcclusion: {},\n}}\n",
            self.config.bloom_intensity,
            self.config.enable_ssao,
        ));

        code
    }
}
