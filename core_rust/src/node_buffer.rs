use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use parking_lot::RwLock;
use rayon::prelude::*;

pub const FLAG_ACTIVE: u8 = 0b00000001;

/// Buffer de nodos pre-asignado y optimizado para multi-threading
pub struct NodeBuffer {
    pub capacity: usize,
    pub count: AtomicUsize,
    
    // SoA Principal
    pub pos_x: Vec<f32>,
    pub pos_y: Vec<f32>,
    pub pos_z: Vec<f32>,
    pub vel_x: Vec<f32>,
    pub vel_y: Vec<f32>,
    pub vel_z: Vec<f32>,
    pub force_x: Vec<f64>,
    pub force_y: Vec<f64>,
    pub force_z: Vec<f64>,
    
    // Propiedades secundarias
    pub mass: Vec<f32>,
    pub temperature: Vec<f32>,
    pub material_id: Vec<u16>,
    pub state_flags: Vec<u8>,
    pub category: Vec<i32>,
    pub damage_type: Vec<i32>,
    pub scale_x: Vec<f32>,
    pub scale_y: Vec<f32>,
    pub scale_z: Vec<f32>,
    pub lifetime: Vec<f32>,
    
    dirty: AtomicBool,
    
    // Double buffering para Unreal
    pub read_buffer: Arc<RwLock<CompactNodeData>>,
    pub write_buffer: Arc<RwLock<CompactNodeData>>,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct CompactNodeData {
    pub count: u32,
    pub positions: Vec<f32>, // [x,y,z, x,y,z, ...]
    pub colors: Vec<u32>,    // ABGR packed
    pub scales: Vec<f32>,
}

impl NodeBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            count: AtomicUsize::new(0),
            pos_x: vec![0.0; capacity],
            pos_y: vec![0.0; capacity],
            pos_z: vec![0.0; capacity],
            vel_x: vec![0.0; capacity],
            vel_y: vec![0.0; capacity],
            vel_z: vec![0.0; capacity],
            force_x: vec![0.0; capacity],
            force_y: vec![0.0; capacity],
            force_z: vec![0.0; capacity],
            mass: vec![1.0; capacity],
            temperature: vec![300.0; capacity],
            material_id: vec![0; capacity],
            state_flags: vec![0; capacity],
            category: vec![0; capacity],
            damage_type: vec![0; capacity],
            scale_x: vec![1.0; capacity],
            scale_y: vec![1.0; capacity],
            scale_z: vec![1.0; capacity],
            lifetime: vec![0.0; capacity],
            dirty: AtomicBool::new(false),
            read_buffer: Arc::new(RwLock::new(CompactNodeData::default())),
            write_buffer: Arc::new(RwLock::new(CompactNodeData::default())),
        }
    }

    pub fn add_node(&self, pos: [f32; 3], vel: [f32; 3], mass: f32, mat: u16, life: f32, scale: [f32; 3], cat: i32, dmg_type: i32) -> Option<usize> {
        let idx = self.count.fetch_add(1, Ordering::Relaxed);
        if idx >= self.capacity {
            self.count.fetch_sub(1, Ordering::Relaxed);
            return None;
        }

        unsafe {
            let p = self as *const Self as *mut Self;
            (&mut (*p).pos_x)[idx] = pos[0];
            (&mut (*p).pos_y)[idx] = pos[1];
            (&mut (*p).pos_z)[idx] = pos[2];
            (&mut (*p).vel_x)[idx] = vel[0];
            (&mut (*p).vel_y)[idx] = vel[1];
            (&mut (*p).vel_z)[idx] = vel[2];
            (&mut (*p).mass)[idx] = mass;
            (&mut (*p).material_id)[idx] = mat;
            (&mut (*p).state_flags)[idx] = FLAG_ACTIVE;
            (&mut (*p).category)[idx] = cat;
            (&mut (*p).damage_type)[idx] = dmg_type;
            (&mut (*p).scale_x)[idx] = scale[0];
            (&mut (*p).scale_y)[idx] = scale[1];
            (&mut (*p).scale_z)[idx] = scale[2];
            (&mut (*p).lifetime)[idx] = life;
            if mat == 100 { (&mut (*p).temperature)[idx] = 5000.0; }
        }
        self.dirty.store(true, Ordering::Relaxed);
        Some(idx)
    }

    pub fn step_parallel(&mut self, dt: f32, ground_z: f32) {
        let n = self.count.load(Ordering::Relaxed).min(self.capacity);
        let dt_f64 = dt as f64;
        let gravity = -980.0; // cm/s^2 (Gravedad estándar de Unreal)
        let damping = 0.998_f32; // Resistencia al aire (Drag)

        (0..n).into_par_iter().for_each(|i| {
            if self.state_flags[i] & FLAG_ACTIVE != 0 {
                let inv_mass = 1.0 / self.mass[i] as f64;
                
                // 1. Acumular Gravedad y Fuerzas
                let ax = self.force_x[i] * inv_mass;
                let ay = self.force_y[i] * inv_mass;
                let az = (self.force_z[i] * inv_mass) + gravity as f64;

                unsafe {
                    let p = self as *const Self as *mut Self;
                    
                    // 2. Integrar Velocidad
                    (&mut (*p).vel_x)[i] += (ax * dt_f64) as f32;
                    (&mut (*p).vel_y)[i] += (ay * dt_f64) as f32;
                    (&mut (*p).vel_z)[i] += (az * dt_f64) as f32;
                    
                    // 3. Aplicar Damping (Rozamiento con el aire)
                    (&mut (*p).vel_x)[i] *= damping;
                    (&mut (*p).vel_y)[i] *= damping;
                    (&mut (*p).vel_z)[i] *= damping;

                    // 4. Integrar Posición
                    (&mut (*p).pos_x)[i] += (&mut (*p).vel_x)[i] * dt;
                    (&mut (*p).pos_y)[i] += (&mut (*p).vel_y)[i] * dt;
                    (&mut (*p).pos_z)[i] += (&mut (*p).vel_z)[i] * dt;
                    
                    // 5. Colisión con el suelo (Nativa y veloz)
                    if (&(*p).pos_z)[i] < ground_z {
                        (&mut (*p).pos_z)[i] = ground_z;
                        
                        let mat_id = self.material_id[i];
                        // Material: 0=Acero, 1=Concreto, 2=Madera, 3=Cristal, 4=Carne
                        let (bounce, friction) = match mat_id {
                            0 => (0.05, 0.3), // Acero: Cae en seco (casi 0 rebote), alto rozamiento rápido
                            1 => (0.1,  0.5), // Concreto: Rebota muy poco
                            2 => (0.25, 0.7), // Madera: Rebota moderado
                            3 => (0.3,  0.8), // Cristal: Rebota bastante
                            4 => (0.01, 0.2), // Carne: Splat instantáneo
                            _ => (0.3,  0.7), // Default
                        };

                        let bounce_vel = -(&mut (*p).vel_z)[i] * bounce;
                        if bounce_vel < 30.0 {
                            (&mut (*p).vel_z)[i] = 0.0;
                        } else {
                            (&mut (*p).vel_z)[i] = bounce_vel;
                        }
                        
                        (&mut (*p).vel_x)[i] *= (1.0 - friction);
                        (&mut (*p).vel_y)[i] *= (1.0 - friction);

                        
                        // Si no tiene velocidad, lo dejamos quieto pero VISIBLE (no quitamos FLAG_ACTIVE)
                        if (&(*p).vel_z)[i] == 0.0 && 
                           (&(*p).vel_x)[i].abs() < 1.0 && 
                           (&(*p).vel_y)[i].abs() < 1.0 {
                             (&mut (*p).vel_x)[i] = 0.0;
                             (&mut (*p).vel_y)[i] = 0.0;
                        }
                    }

                    // 6. Resetear Fuerzas para el siguiente tick
                    (&mut (*p).force_x)[i] = 0.0;
                    (&mut (*p).force_y)[i] = 0.0;
                    (&mut (*p).force_z)[i] = 0.0;
                    
                    if (&(*p).lifetime)[i] > 0.0 {
                        (&mut (*p).lifetime)[i] -= dt;
                        if (&(*p).lifetime)[i] <= 0.0 { (&mut (*p).state_flags)[i] &= !FLAG_ACTIVE; }
                    }
                }
            }
        });
        self.dirty.store(true, Ordering::Relaxed);
    }

    pub fn prepare_render_data(&self) {
        if !self.dirty.swap(false, Ordering::Relaxed) { return; }
        
        let n = self.count.load(Ordering::Relaxed);
        let mut write = self.write_buffer.write();
        
        write.positions.clear();
        write.colors.clear();
        write.scales.clear();
        
        for i in 0..n {
            if self.state_flags[i] & FLAG_ACTIVE != 0 {
                write.positions.push(self.pos_x[i]);
                write.positions.push(self.pos_y[i]);
                write.positions.push(self.pos_z[i]);
                
                // Color (ABGR)
                let temp = self.temperature[i];
                let color = if self.material_id[i] == 100 {
                    0xFFFFFF00 // Cyan Brillante
                } else if temp > 1000.0 {
                    0xFF0055FF // Naranja/Rojo
                } else {
                    0xFF888888 // Gris
                };
                write.colors.push(color);
                
                // Escala
                let scale = if self.material_id[i] == 100 { 2.0 } else { 5.0 };
                write.scales.push(scale);
            }
        }
        write.count = (write.positions.len() / 3) as u32;
        drop(write);
        
        // Swap buffers
        let mut read = self.read_buffer.write();
        let mut write2 = self.write_buffer.write();
        std::mem::swap(&mut *read, &mut *write2);
    }
}
