//! # Classic Solver – Física Estándar (Gravedad + Aire)
//! Este módulo contiene la integración básica que siempre se ejecuta.

use crate::node_buffer::NodeBuffer;

pub fn integrate_classic_modular(
    nodes: &mut NodeBuffer, 
    i: usize, 
    dt: f32, 
    gravity: f32, 
    dt_f64: f64,
    drag_scale: f32
) {
    let p = nodes as *mut NodeBuffer;
    unsafe {
        let inv_mass = 1.0 / (*((*p).mass.as_ptr().add(i))) as f64;

        // 1. Aceleración (Fuerzas externas + Gravedad)
        let ax = (*((*p).force_x.as_ptr().add(i))) * inv_mass;
        let ay = (*((*p).force_y.as_ptr().add(i))) * inv_mass;
        let az = (*((*p).force_z.as_ptr().add(i))) * inv_mass + (gravity as f64);

        // 2. Integrar Velocidad
        let vx = (*p).vel_x.as_mut_ptr().add(i);
        let vy = (*p).vel_y.as_mut_ptr().add(i);
        let vz = (*p).vel_z.as_mut_ptr().add(i);
        
        *vx += (ax * dt_f64) as f32;
        *vy += (ay * dt_f64) as f32;
        *vz += (az * dt_f64) as f32;
        
        // 3. Rozamiento de aire (Damping)
        *vx *= drag_scale;
        *vy *= drag_scale;
        *vz *= drag_scale;

        // 4. Integrar Posición
        *((*p).pos_x.as_mut_ptr().add(i)) += *vx * dt;
        *((*p).pos_y.as_mut_ptr().add(i)) += *vy * dt;
        *((*p).pos_z.as_mut_ptr().add(i)) += *vz * dt;
        
        // 5. Integrar Rotación
        let avx = (*p).ang_vel_x.as_mut_ptr().add(i);
        let avy = (*p).ang_vel_y.as_mut_ptr().add(i);
        let avz = (*p).ang_vel_z.as_mut_ptr().add(i);

        // Rozamiento Rotacional (Damping Angular)
        const ANG_DAMPING: f32 = 0.985;
        *avx *= ANG_DAMPING;
        *avy *= ANG_DAMPING;
        *avz *= ANG_DAMPING;

        *((*p).rot_x.as_mut_ptr().add(i)) += (*avx) * dt;
        *((*p).rot_y.as_mut_ptr().add(i)) += (*avy) * dt;
        *((*p).rot_z.as_mut_ptr().add(i)) += (*avz) * dt;
    }
}

pub fn handle_ground_collision(nodes: &mut NodeBuffer, i: usize, ground_z: f32) {
    let p = nodes as *mut NodeBuffer;
    unsafe {
        let pz = (*p).pos_z.as_mut_ptr().add(i);
        if *pz < ground_z {
            *pz = ground_z;
            let bounce = 0.4f32;
            let friction = 0.3f32;
            
            let vz = (*p).vel_z.as_mut_ptr().add(i);
            let b_vel = -(*vz) * bounce;

            if b_vel < 5.0 {
                *vz = 0.0;
                // Si la velocidad es muy baja, el fragmento está "durmiendo"
                // Cero las velocidades angulares para evitar temblores
                let avx = (*p).ang_vel_x.as_mut_ptr().add(i);
                let avy = (*p).ang_vel_y.as_mut_ptr().add(i);
                let avz = (*p).ang_vel_z.as_mut_ptr().add(i);
                *avx = 0.0;
                *avy = 0.0;
                *avz = 0.0;
            } else {
                *vz = b_vel;
                
                let px = *((*p).pos_x.as_ptr().add(i));
                let seed = ((px * 123.0) as i32).abs() as u32;
                let rx = ((seed % 100) as f32 / 50.0) - 1.0;
                
                *((*p).vel_x.as_mut_ptr().add(i)) += rx * b_vel * 0.2;
                *((*p).ang_vel_x.as_mut_ptr().add(i)) += rx * 1000.0;
            }
            
            *((*p).vel_x.as_mut_ptr().add(i)) *= 1.0 - friction;
            *((*p).vel_y.as_mut_ptr().add(i)) *= 1.0 - friction;
            
            // Freno Total en reposo (Evitar vibración)
            let vx = (*p).vel_x.as_ptr().add(i);
            let vy = (*p).vel_y.as_ptr().add(i);
            let avx_p = (*p).ang_vel_x.as_mut_ptr().add(i);
            let avy_p = (*p).ang_vel_y.as_mut_ptr().add(i);
            let avz_p = (*p).ang_vel_z.as_mut_ptr().add(i);
            
            // Si apenas se mueve horizontalmente, paramos la rotación
            if unsafe { (*vx).abs() < 10.0 && (*vy).abs() < 10.0 } {
                unsafe {
                    *avx_p *= 0.1;
                    *avy_p *= 0.1;
                    *avz_p *= 0.1;
                    
                    // Si está casi quieto, frenazo en seco
                    if (*vx).abs() < 2.0 && (*vy).abs() < 2.0 {
                        *((*p).vel_x.as_mut_ptr().add(i)) = 0.0;
                        *((*p).vel_y.as_mut_ptr().add(i)) = 0.0;
                        *avx_p = 0.0;
                        *avy_p = 0.0;
                        *avz_p = 0.0;
                    }
                }
            } else {
                unsafe {
                    *avx_p *= 0.8;
                    *avy_p *= 0.8;
                    *avz_p *= 0.8;
                }
            }
            
            *((*p).force_x.as_mut_ptr().add(i)) = 0.0;
            *((*p).force_y.as_mut_ptr().add(i)) = 0.0;
            *((*p).force_z.as_mut_ptr().add(i)) = 0.0;
        }
    }
}
