use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use parking_lot::RwLock;
use rayon::prelude::*;

pub const FLAG_ACTIVE: u8 = 0b00000001;

/// Buffer de nodos pre-asignado y optimizado para multi-threading.
/// Solo datos puros y operaciones genéricas de SoA.
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

    // Nueva Física de Rotación (Realism 2.0)
    pub rot_x: Vec<f32>,
    pub rot_y: Vec<f32>,
    pub rot_z: Vec<f32>,
    pub ang_vel_x: Vec<f32>,
    pub ang_vel_y: Vec<f32>,
    pub ang_vel_z: Vec<f32>,

    // Origen de la explosión (para efectos Black Hole / Matrix)
    pub source_x: Vec<f32>,
    pub source_y: Vec<f32>,
    pub source_z: Vec<f32>,
    
    // Jupiter Realism: Factor de deformación / caos
    pub chaos_factor: Vec<f32>,
    pub proxy_hash: Vec<u32>, // Nuevo: Para filtrado Zero-Copy
    
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

static mut GLOBAL_CHAOS: f32 = 0.4; // Por defecto: Cubos irregulares

#[no_mangle]
pub extern "C" fn sf_set_global_chaos(val: f32) {
    unsafe { GLOBAL_CHAOS = val.clamp(0.0, 1.0); }
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
            rot_x: vec![0.0; capacity],
            rot_y: vec![0.0; capacity],
            rot_z: vec![0.0; capacity],
            ang_vel_x: vec![0.0; capacity],
            ang_vel_y: vec![0.0; capacity],
            ang_vel_z: vec![0.0; capacity],
            source_x: vec![0.0; capacity],
            source_y: vec![0.0; capacity],
            source_z: vec![0.0; capacity],
            chaos_factor: vec![0.0; capacity],
            proxy_hash: vec![0; capacity],
            dirty: AtomicBool::new(false),
            read_buffer: Arc::new(RwLock::new(CompactNodeData::default())),
            write_buffer: Arc::new(RwLock::new(CompactNodeData::default())),
        }
    }

    pub fn add_node(
        &mut self, 
        pos: [f32; 3], 
        vel: [f32; 3], 
        mass: f32, 
        mat: u16, 
        life: f32, 
        scale: [f32; 3], 
        cat: i32, 
        dmg_type: i32,
        source: [f32; 3]
    ) -> Option<usize> {
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
            
            // JUPITER REALISM V4: Deformación No-Cúbica
            let chaos = GLOBAL_CHAOS;
            let r_seed = (pos[0] * 123.0 + pos[1] * 456.0 + pos[2] * 789.0).abs() as u32;
            
            // Si nos pasan una escala física (desde proxy.rs), la respetamos.
            // Si no, usamos los valores por defecto.
            let mut rx = if scale[0] > 0.0 { scale[0] } else { if cat == 0 { 2.5 } else if cat == 1 { 1.0 } else { 0.4 } };
            let mut ry = if scale[1] > 0.0 { scale[1] } else { if cat == 0 { 2.5 } else if cat == 1 { 1.0 } else { 0.4 } };
            let mut rz = if scale[2] > 0.0 { scale[2] } else { if cat == 0 { 2.5 } else if cat == 1 { 1.0 } else { 0.4 } };

            if chaos > 0.0 {
                let chaos_mult = rx.min(ry).min(rz);
                rx += ((r_seed % 100) as f32 / 50.0 - 1.0) * chaos * chaos_mult;
                ry += (((r_seed / 7) % 100) as f32 / 50.0 - 1.0) * chaos * chaos_mult;
                rz += (((r_seed / 13) % 100) as f32 / 50.0 - 1.0) * chaos * chaos_mult;
            }

            (&mut (*p).scale_x)[idx] = rx;
            (&mut (*p).scale_y)[idx] = ry;
            (&mut (*p).scale_z)[idx] = rz;
            (&mut (*p).chaos_factor)[idx] = chaos;
            
            (&mut (*p).source_x)[idx] = source[0];
            (&mut (*p).source_y)[idx] = source[1];
            (&mut (*p).source_z)[idx] = source[2];

            (&mut (*p).rot_x)[idx] = (r_seed % 360) as f32;
            (&mut (*p).rot_y)[idx] = ((r_seed / 360) % 360) as f32;
            (&mut (*p).rot_z)[idx] = ((r_seed / 129600) % 360) as f32;
            
            let spin_scale = if mat == 100 { 1500.0 } else { 500.0 };
            (&mut (*p).ang_vel_x)[idx] = ((r_seed % 100) as f32 / 50.0 - 1.0) * spin_scale;
            (&mut (*p).ang_vel_y)[idx] = (((r_seed / 100) % 100) as f32 / 50.0 - 1.0) * spin_scale;
            (&mut (*p).ang_vel_z)[idx] = (((r_seed / 10000) % 100) as f32 / 50.0 - 1.0) * spin_scale;

            if mat == 100 { (&mut (*p).temperature)[idx] = 5000.0; }
        }
        self.dirty.store(true, Ordering::Relaxed);
        Some(idx)
    }

    /// La física se ha movido out of here para mayor modularidad.
    /// node_buffer ahora solo notifica que ha sido modificado.
    pub fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Relaxed);
    }

    pub fn prepare_render_data(&self) {
        if !self.dirty.swap(false, Ordering::Relaxed) { return; }
        let n = self.count.load(Ordering::Relaxed).min(self.capacity);
        let mut write = self.write_buffer.write();
        write.positions.clear();
        write.colors.clear();
        write.scales.clear();
        for i in 0..n {
            if self.state_flags[i] & FLAG_ACTIVE != 0 {
                write.positions.push(self.pos_x[i]);
                write.positions.push(self.pos_y[i]);
                write.positions.push(self.pos_z[i]);
                let temp = self.temperature[i];
                let color = if self.material_id[i] == 100 { 0xFFFFFF00 } else if temp > 1000.0 { 0xFF0055FF } else { 0xFF888888 };
                write.colors.push(color);
                
                // USAR ESCALAS REALES DEL NODO (Júpiter etc.)
                write.scales.push(self.scale_x[i]);
                write.scales.push(self.scale_y[i]);
                write.scales.push(self.scale_z[i]);
            }
        }
        write.count = (write.positions.len() / 3) as u32;
        drop(write);
        let mut read = self.read_buffer.write();
        let mut write2 = self.write_buffer.write();
        std::mem::swap(&mut *read, &mut *write2);
    }
}
