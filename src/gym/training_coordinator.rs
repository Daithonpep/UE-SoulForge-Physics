// gym/training_coordinator.rs
use crate::gym::gym_director::*;
use crate::gym::data_librarian::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TrainingCoordinator {
    gym: Arc<RwLock<GymDirector>>,
    librarian: Arc<RwLock<DataLibrarian>>,
    wm_coordinator: Arc<RwLock<crate::world_model::coordinator::WorldModelCoordinator>>,
}

impl TrainingCoordinator {
    pub fn new(
        gym: Arc<RwLock<GymDirector>>, 
        librarian: Arc<RwLock<DataLibrarian>>,
        wm_coordinator: Arc<RwLock<crate::world_model::coordinator::WorldModelCoordinator>>
    ) -> Self {
        Self {
            gym,
            librarian,
            wm_coordinator,
        }
    }

    /// Ciclo principal de entrenamiento
    pub async fn run_training_cycle(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        log::info!("🏋️ INICIANDO TRAINING CYCLE - SOULFORGE GYM");
        log::info!("═══════════════════════════════════════════\n");

        loop {
            let max_epochs;
            let current_epoch;
            let is_paused;
            
            // Scope for borrowing gym
            {
                let gym_read = self.gym.read().await;
                max_epochs = gym_read.max_epochs;
                current_epoch = gym_read.current_epoch;
                is_paused = gym_read.is_paused;
            }

            if is_paused {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                continue;
            }

            if current_epoch >= max_epochs {
                // Auto-pause instead of dying - user can start a new session from dashboard
                let mut gym_write = self.gym.write().await;
                if !gym_write.is_paused {
                    gym_write.is_paused = true;
                    gym_write.add_log(format!("✅ Sesión completada: {} épocas. Listo para nuevo ciclo.", max_epochs));
                }
                drop(gym_write);
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                continue;
            }

            // 1. GENERAR PROMPT (o sacar de la cola manual)
            let prompt = {
                let mut gym_write = self.gym.write().await;
                if let Some(manual) = gym_write.manual_queue.pop_front() {
                    manual
                } else {
                    gym_write.generate_next_prompt()
                }
            };
            
            log::info!("\n📋 EPOCH {}/{}", prompt.epoch, max_epochs);
            log::info!("   Tipo: {:?}", prompt.prompt_type);
            log::info!("   Nivel: {:?}", prompt.difficulty_level);
            log::info!("   Tarea: {}", prompt.description);

            // 2. PREPARAR REFERENCIA
            let reference = match self.prepare_reference(&prompt).await {
                Ok(r) => r,
                Err(e) => {
                    let err_msg = e.to_string();
                    log::error!("Error crítico preparando referencia: {}", err_msg);
                    let mut gym_write = self.gym.write().await;
                    gym_write.add_log(format!("❌ FALLIDO: Error cargando referencia. {}", err_msg));
                    continue;
                }
            };
            log::info!("   ✓ Referencia cargada: {} puntos", reference.points.len());

            // 3. ENVIAR A DAITHON (aquí se conectaría con tu puente a Unreal)
            log::info!("   → Enviando a Daithon...");
            
            // Simular construcción (en producción, esperar respuesta de Unreal)
            let daithon_output = self.simulate_daithon_construction(&reference).await?;

            // 4. COMPARAR RESULTADO
            let comparison = {
                let lib_read = self.librarian.read().await;
                lib_read.compare(&reference, &daithon_output)
            };
            log::info!("   ⚖️ Similitud: {:.1}%", comparison.similarity_score * 100.0);

            // 5. CONSTRUIR RESULTADO (simular métricas de Unreal)
            let result = TrainingResult {
                prompt_id: prompt.id,
                visual_similarity: comparison.similarity_score,
                fps_stability: 0.95, // Placeholder
                draw_calls: 350,
                collision_errors: 0,
                memory_leaks: false,
                aesthetic_score: 0.85,
                execution_time_ms: 1500,
            };

            // 6. CALCULAR RECOMPENSA Y REGISTRAR
            {
                let mut gym_write = self.gym.write().await;
                let is_last_epoch = prompt.epoch >= gym_write.max_epochs;
                let is_eval_epoch = is_last_epoch || prompt.epoch % 10 == 0;

                let mut reward = gym_write.calculate_reward(&result);
                
                // --- INTEGRACIÓN CURIOSIDAD (MOLDEADO DE MUNDO) ---
                // Registramos la transición en el World Model y obtenemos el bono de curiosidad
                use crate::world_model::state::{StateTransition, WorldState, VisualState, PhysicsState};
                use crate::world_model::state::AgentAction; // ensure correct import

                let mut wm = self.wm_coordinator.write().await;
                
                // Creamos un transition dummy para el World Model
                let transition = StateTransition {
                    state_before: WorldState {
                        timestamp: prompt.epoch,
                        agent_action: AgentAction {
                            action_type: crate::world_model::state::ActionType::SpawnAsset,
                            parameters: crate::world_model::state::ActionParameters {
                                primitive_type: None,
                                asset_id: Some("auto_architect".to_string()),
                                asset_path: None,
                                position: [0.0, 0.0, 0.0],
                                rotation: [0.0, 0.0, 0.0],
                                scale: [1.0, 1.0, 1.0],
                                pcg_seed: None,
                                pcg_density: None,
                            },
                        },
                        visual_state: VisualState { feature_vector: vec![0.5; 16], object_count: 0, scene_complexity: 0.0, dominant_colors: vec![], spatial_distribution: vec![] },
                        physics_state: PhysicsState { static_objects: 0, dynamic_objects: 0, collision_active: false, gravity_enabled: true, total_mass: 0.0 },
                        performance_metrics: crate::world_model::state::PerformanceMetrics { fps: 60.0, draw_calls: 0, triangles: 0, memory_mb: 2000.0 },
                    },
                    state_after: WorldState {
                        timestamp: prompt.epoch + 1,
                        agent_action: AgentAction {
                            action_type: crate::world_model::state::ActionType::SpawnAsset,
                            parameters: crate::world_model::state::ActionParameters {
                                primitive_type: None,
                                asset_id: Some("auto_architect".to_string()),
                                asset_path: None,
                                position: [0.0, 0.0, 0.0],
                                rotation: [0.0, 0.0, 0.0],
                                scale: [1.0, 1.0, 1.0],
                                pcg_seed: None,
                                pcg_density: None,
                            },
                        },
                        visual_state: VisualState { feature_vector: vec![result.visual_similarity; 16], object_count: 10, scene_complexity: result.visual_similarity, dominant_colors: vec![], spatial_distribution: vec![] },
                        physics_state: PhysicsState { static_objects: 9, dynamic_objects: 1, collision_active: true, gravity_enabled: true, total_mass: 100.0 },
                        performance_metrics: crate::world_model::state::PerformanceMetrics { fps: result.fps_stability * 60.0, draw_calls: result.draw_calls, triangles: result.draw_calls * 100, memory_mb: 2500.0 },
                    },
                    success: result.visual_similarity > 0.6,
                    reward: reward.total_reward,
                };

                // Inyectamos curiosidad
                if let Some(report) = wm.record_transition(transition) {
                    if report.overall_discrepancy > 0.1 {
                        let bonus = (report.overall_discrepancy * 0.1).min(0.2);
                        reward.total_reward += bonus;
                        gym_write.add_log(format!("✨ Bono de Curiosidad: +{:.3} (Exploración detectada)", bonus));
                    }
                }
                drop(wm);

                log::info!("   🎯 Recompensa Final (Total): {:.3}", reward.total_reward);
                
                gym_write.add_log(format!("Época {} completa: Recompensa {:.3}. {}", prompt.epoch, reward.total_reward, reward.feedback_message));
                
                if is_eval_epoch {
                    gym_write.add_log(format!("📊 SYNC VISUAL: Actualizando Matriz en Época {}", prompt.epoch));
                    gym_write.latest_clouds = Some((reference.clone(), daithon_output.clone(), daithon_output.clone()));
                    
                    // --- SYNTHETIC COLLISION MONITOR ---
                    // Genera plan real de cajas de colisión para el dashboard
                    let architect = crate::agents::architect::Architect::new();
                    let mut geometer = crate::agents::geometer::Geometer::new();
                    let task_graph = architect.analyze_and_plan(&prompt.description);
                    let plan = geometer.execute_plan(task_graph);
                    gym_write.latest_plan = Some(plan);
                } else {
                    // Update heartbeat for UI but not the heavy clouds
                    gym_write.add_log(format!("⚙️ Entrenando... (Optimización en curso)"));
                }
                
                // Track geometry weakness if it's synthetic
                if let ReferenceData::SyntheticGeometry { ref formula, .. } = prompt.reference_data {
                    let geo_type = formula.split('(').next().unwrap_or("unknown");
                    gym_write.track_weakness(geo_type, result.visual_similarity);
                }

                gym_write.record_result(&prompt.description, result, reward);

                // 8. CHECKPOINT CADA 10 EPOCHS
                if prompt.epoch % 10 == 0 {
                    let path = format!("checkpoints/epoch_{}.json", prompt.epoch);
                    if let Err(e) = gym_write.save_checkpoint(&path) {
                        log::error!("Fallo al guardar checkpoint: {}", e);
                    }
                }
            }
            
            // Artificial delay (reducido para entrenamiento rápido)
            let delay = if prompt.epoch % 10 == 0 { 200 } else { 50 };
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }
    }

    /// Prepara la referencia según el tipo de prompt
    async fn prepare_reference(&self, prompt: &TrainingPrompt) -> Result<PointCloud, Box<dyn std::error::Error + Send + Sync>> {
        match &prompt.reference_data {
            ReferenceData::MeshFile { path, .. } => {
                let mut lib_write = self.librarian.write().await;
                match lib_write.fetch_reference(path).await {
                    Ok(pc) => Ok(pc),
                    Err(e) => {
                        log::error!("❌ Error cargando referencia {}: {}", path, e);
                        // Fallback a cubo por defecto si el archivo no existe para no romper el ciclo
                        Ok(SyntheticGeometryGenerator::default_cube())
                    }
                }
            }

            ReferenceData::SyntheticGeometry { formula, parameters, .. } => {
                Ok(SyntheticGeometryGenerator::generate(formula, parameters))
            }

            ReferenceData::HybridTask { base_mesh, procedural_modifications } => {
                let mut base = {
                    let mut lib_write = self.librarian.write().await;
                    match lib_write.fetch_reference(base_mesh).await {
                        Ok(pc) => pc,
                        Err(e) => {
                            log::error!("❌ Error cargando base mesh {}: {}", base_mesh, e);
                            SyntheticGeometryGenerator::default_cube()
                        }
                    }
                };
                
                // Aplicar modificaciones procedurales
                for modification in procedural_modifications {
                    base = self.apply_procedural_modification(base, modification)?;
                }

                Ok(base)
            }
        }
    }

    /// Aplica modificación procedural a una mesh base
    fn apply_procedural_modification(
        &self,
        mut base: PointCloud,
        modification: &str,
    ) -> Result<PointCloud, Box<dyn std::error::Error + Send + Sync>> {
        // Parsear comando
        if modification.contains("add_crystals") {
            // Extraer parámetros
            let count = 5;
            for _ in 0..count {
                let crystal = SyntheticGeometryGenerator::generate(
                    "star_extrusion",
                    &[6.0, 0.1, 0.3, 1.0],
                );
                base.points.extend(crystal.points);
            }
        }

        if modification.contains("apply_fracture_pattern") {
            // Añadir grietas procedurales
            for _ in 0..20 {
                let idx = fastrand::usize(0..base.points.len());
                let point = base.points[idx];
                let offset = [
                    (fastrand::f32() - 0.5) * 0.1,
                    (fastrand::f32() - 0.5) * 0.1,
                    (fastrand::f32() - 0.5) * 0.1,
                ];
                base.points.push([
                    point[0] + offset[0],
                    point[1] + offset[1],
                    point[2] + offset[2],
                ]);
            }
        }

        base.bounds = DataLibrarian::calculate_bounds(&base.points);

        Ok(base)
    }

    /// Simulación de construcción de Daithon (placeholder)
    /// Daithon "aprende" — el ruido se reduce conforme avanza el entrenamiento
    async fn simulate_daithon_construction(&self, reference: &PointCloud) -> Result<PointCloud, Box<dyn std::error::Error + Send + Sync>> {
        let epoch = {
            let gym = self.gym.read().await;
            gym.current_epoch
        };
        
        // Daithon improves: noise starts at 2.0 and decays toward 0.02 as epochs progress
        // Learning curve: fast initial improvement, slower refinement later
        let learning_progress = 1.0 - (-0.03 * epoch as f64).exp(); // 0→1 over ~100 epochs
        let noise_factor = (2.0 - 1.98 * learning_progress) as f32; // 2.0 → 0.02
        
        // Also randomly drop some points early on (Daithon misses parts of the shape)
        let keep_ratio = (0.5 + 0.5 * learning_progress) as f32; // 50% → 100%

        let noisy_points: Vec<[f32; 3]> = reference
            .points
            .iter()
            .filter(|_| fastrand::f32() < keep_ratio)
            .map(|p| {
                [
                    p[0] + (fastrand::f32() - 0.5) * noise_factor,
                    p[1] + (fastrand::f32() - 0.5) * noise_factor,
                    p[2] + (fastrand::f32() - 0.5) * noise_factor,
                ]
            })
            .collect();

        let bounds = DataLibrarian::calculate_bounds(&noisy_points);

        Ok(PointCloud {
            points: noisy_points,
            normals: vec![],
            bounds,
        })
    }
}
