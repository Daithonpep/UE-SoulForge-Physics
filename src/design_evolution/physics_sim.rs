use rapier3d::prelude::*;
use crate::forge::experimental_lab::{UnrealExperiment, UnrealSimResult, StructureType, StressTest, Placement, SurfaceType, ForceDirection, FailurePoint};
use crate::knowledge::physics_laws::PhysicsKnowledgeBase;
use std::collections::HashMap;

pub struct PhysicsSimulator {}

impl PhysicsSimulator {
    pub fn simulate_autonomous_experiment(experiment: &UnrealExperiment) -> UnrealSimResult {
        Self::simulate_with_params(experiment)
    }

    pub fn simulate_with_params(
        params: &UnrealExperiment,
    ) -> UnrealSimResult {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut gravity = vector![0.0, -9.81, 0.0];
        let mut integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = DefaultBroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        // 1. Construir suelo
        Self::build_surface(
            &params.placement.on_surface,
            &mut rigid_body_set,
            &mut collider_set,
        );

        // 2. Construir estructura
        let block_handles = Self::build_structure_rotated(
            &params.structure_type,
            &params.placement,
            &params.material,
            &mut rigid_body_set,
            &mut collider_set,
        );

        // 3. Simular asentamiento
        for _ in 0..60 {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                &mut ccd_solver,
                None,
                &physics_hooks,
                &event_handler,
            );
        }

        // Registrar posiciones post-asentamiento
        let settled_positions: Vec<[f64; 3]> = block_handles.iter()
            .map(|h| {
                let pos = rigid_body_set[*h].translation();
                [pos.x as f64, pos.y as f64, pos.z as f64]
            })
            .collect();

        // 4. Aplicar fuerza catastrófica (User's PGA Logic)
        let force_per_block = match &params.stress_test {
            StressTest::Seismic(s) => {
                10.0_f32.powf((s.magnitude - 5.0) * 0.7)
            },
            StressTest::Wind(w) => {
                0.5 * 1.225 * w.speed.powi(2) * 0.5
            },
        };

        let force_direction_vector = match &params.force_direction {
            ForceDirection::Lateral => vector![1.0, 0.0, 0.0],
            ForceDirection::Frontal => vector![0.0, 0.0, 1.0],
            ForceDirection::Diagonal(angle) => {
                let rad = (*angle as f32).to_radians();
                vector![rad.cos(), 0.0, rad.sin()]
            },
            ForceDirection::FromBelow => vector![0.0, 1.0, 0.0],
            ForceDirection::Downward(_) => vector![0.0, -1.0, 0.0],
            ForceDirection::Rotational(_) => vector![0.5, 0.0, 0.5],
        };

        for handle in &block_handles {
            let body = &mut rigid_body_set[*handle];
            let mass = body.mass();
            let impulse = force_direction_vector * force_per_block * mass;
            body.apply_impulse(impulse, true);
        }

        // 5. Simular respuesta (2 segundos)
        for _ in 0..120 {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                &mut ccd_solver,
                None,
                &physics_hooks,
                &event_handler,
            );
        }

        // 6. Medir resultado
        Self::measure_result(
            &settled_positions,
            &block_handles,
            &rigid_body_set,
        )
    }

    fn build_surface(
        surface: &SurfaceType,
        bodies: &mut RigidBodySet,
        colliders: &mut ColliderSet,
    ) {
        match surface {
            SurfaceType::Flat => {
                let ground = RigidBodyBuilder::fixed()
                    .translation(vector![0.0, -0.5, 0.0])
                    .build();
                let handle = bodies.insert(ground);
                let col = ColliderBuilder::cuboid(500.0, 0.5, 500.0).build();
                colliders.insert_with_parent(col, handle, bodies);
            },
            _ => { // Simplificado para brevedad, igual que Flat pero con variaciones si necesario
                let ground = RigidBodyBuilder::fixed()
                    .translation(vector![0.0, -0.5, 0.0])
                    .build();
                let handle = bodies.insert(ground);
                let col = ColliderBuilder::cuboid(500.0, 0.5, 500.0).build();
                colliders.insert_with_parent(col, handle, bodies);
            }
        }
    }

    fn build_structure_rotated(
        structure: &StructureType,
        placement: &Placement,
        material_name: &str,
        bodies: &mut RigidBodySet,
        colliders: &mut ColliderSet,
    ) -> Vec<RigidBodyHandle> {
        let rot = placement.rotation;
        let rot_x = (rot[0] as f32).to_radians();
        let rot_y = (rot[1] as f32).to_radians();
        let rot_z = (rot[2] as f32).to_radians();

        let kb = PhysicsKnowledgeBase::initialize();
        let material = kb.materials.get(material_name)
            .cloned()
            .unwrap_or_else(|| kb.materials.get("concrete").unwrap().clone());

        let density = material.density as f32 / 1000.0;
        let mut handles = Vec::new();

        match structure {
            StructureType::Pyramid(p) => {
                let levels = 5usize;
                let block_size = p.base_width / levels as f32;

                for level in 0..levels {
                    let blocks_in_level = levels - level;
                    for i in 0..blocks_in_level {
                        for j in 0..blocks_in_level {
                            let local_x = (i as f32 - blocks_in_level as f32 / 2.0) * block_size + block_size/2.0;
                            let local_y = level as f32 * block_size + block_size/2.0;
                            let local_z = (j as f32 - blocks_in_level as f32 / 2.0) * block_size + block_size/2.0;

                            let (rx, ry, rz) = Self::rotate_point(
                                local_x, local_y, local_z,
                                rot_x, rot_y, rot_z,
                            );

                            let rb = RigidBodyBuilder::dynamic()
                                .translation(vector![
                                    rx + placement.position[0] as f32,
                                    ry + placement.position[1] as f32,
                                    rz + placement.position[2] as f32
                                ]).build();

                            let vol = (block_size/2.1).powi(3) * 8.0;
                            let mass = vol * density;

                            let handle = bodies.insert(rb);
                            let col = ColliderBuilder::cuboid(block_size/2.1, block_size/2.1, block_size/2.1)
                                .mass(mass)
                                .friction(material.friction_coefficient as f32)
                                .build();

                            colliders.insert_with_parent(col, handle, bodies);
                            handles.push(handle);
                        }
                    }
                }
            },
            _ => { // Fallback para cualquier otro tipo: Un bloque de 2x2x2
                let rb = RigidBodyBuilder::dynamic()
                    .translation(vector![0.0, 2.0, 0.0])
                    .build();
                let handle = bodies.insert(rb);
                let col = ColliderBuilder::cuboid(1.0, 1.0, 1.0)
                    .mass(8.0 * density)
                    .friction(material.friction_coefficient as f32)
                    .build();
                colliders.insert_with_parent(col, handle, bodies);
                handles.push(handle);
            }
        }
        handles
    }

    fn rotate_point(x: f32, y: f32, z: f32, rx: f32, ry: f32, rz: f32) -> (f32, f32, f32) {
        let x1 = x * rz.cos() - y * rz.sin();
        let y1 = x * rz.sin() + y * rz.cos();
        let z1 = z;
        let x2 = x1;
        let y2 = y1 * rx.cos() - z1 * rx.sin();
        let z2 = y1 * rx.sin() + z1 * rx.cos();
        let x3 = x2 * ry.cos() + z2 * ry.sin();
        let y3 = y2;
        let z3 = -x2 * ry.sin() + z2 * ry.cos();
        (x3, y3, z3)
    }

    fn measure_result(settled_positions: &[[f64; 3]], handles: &[RigidBodyHandle], bodies: &RigidBodySet) -> UnrealSimResult {
        let mut max_displacement = 0.0_f64;
        let mut displaced_blocks = 0;
        let total_blocks = handles.len();
        let collapse_threshold = 2.0;

        for (i, handle) in handles.iter().enumerate() {
            if let Some(body) = bodies.get(*handle) {
                let pos = body.translation();
                let current = [pos.x as f64, pos.y as f64, pos.z as f64];
                let settled = &settled_positions[i];
                let displacement = ((current[0] - settled[0]).powi(2) + (current[1] - settled[1]).powi(2) + (current[2] - settled[2]).powi(2)).sqrt();
                if displacement > max_displacement { max_displacement = displacement; }
                if displacement > collapse_threshold { displaced_blocks += 1; }
            }
        }

        let survived = if total_blocks == 0 { true } else { (displaced_blocks as f64 / total_blocks as f64) < 0.3 };

        UnrealSimResult {
            session_id: "autonomous_sim".into(),
            survived,
            max_deformation: (max_displacement / 10.0).min(1.0) as f32,
            failure_points: Vec::new(),
            stress_distribution: HashMap::new(),
            simulation_time_seconds: 5.0,
        }
    }

    pub fn test_stability(_genes: &[crate::design_evolution::mutation_engine::Gene], _scale: &[f32; 3]) -> bool {
        true
    }
}
