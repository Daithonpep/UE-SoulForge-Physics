use nalgebra::{Point3, Vector3, Matrix4, Perspective3, Isometry3, UnitQuaternion};

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Point3::new(3.0, 3.0, 3.0),
            target: Point3::origin(),
            up: Vector3::new(0.0, 1.0, 0.0),
            fov: 60.0,
            near: 0.1,
            far: 1000.0,
            aspect_ratio,
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let eye = &self.position;
        let target = &self.target;
        let up = &self.up;

        Isometry3::look_at_rh(eye, target, up).to_homogeneous()
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Perspective3::new(
            self.aspect_ratio,
            self.fov.to_radians(),
            self.near,
            self.far,
        ).to_homogeneous()
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let to_target = self.target - self.position;
        let distance = to_target.norm();

        let yaw_rotation = UnitQuaternion::from_axis_angle(
            &Vector3::y_axis(),
            delta_yaw,
        );

        let right = to_target.cross(&self.up).normalize();
        let pitch_rotation = UnitQuaternion::from_axis_angle(
            &nalgebra::Unit::new_normalize(right),
            delta_pitch,
        );

        let rotation = yaw_rotation * pitch_rotation;
        let new_direction = rotation * (-to_target.normalize());

        self.position = self.target + new_direction * distance;
    }

    pub fn zoom(&mut self, delta: f32) {
        let to_target = self.target - self.position;
        let distance = to_target.norm();
        let new_distance = (distance + delta).max(0.5).min(50.0);

        self.position = self.target - to_target.normalize() * new_distance;
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward);

        let offset = right * delta_x + up * delta_y;

        self.position += offset;
        self.target += offset;
    }
}

#[derive(Clone, Debug)]
pub enum ViewportMode {
    Perspective,
    Orthographic,
    Top,
    Front,
    Side,
    Quad,
}

pub struct LensViewportManager {
    cameras: Vec<Camera>,
    active_camera: usize,
    pub mode: ViewportMode,
}

impl Default for LensViewportManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LensViewportManager {
    pub fn new() -> Self {
        let mut manager = Self {
            cameras: vec![Camera::new(16.0 / 9.0)],
            active_camera: 0,
            mode: ViewportMode::Perspective,
        };

        manager.setup_cameras();
        manager
    }

    fn setup_cameras(&mut self) {
        self.cameras[0].position = Point3::new(3.0, 3.0, 3.0);
        self.cameras[0].target = Point3::origin();

        let mut top = Camera::new(1.0);
        top.position = Point3::new(0.0, 10.0, 0.0);
        top.target = Point3::origin();
        top.up = Vector3::new(0.0, 0.0, -1.0);
        self.cameras.push(top);

        let mut front = Camera::new(1.0);
        front.position = Point3::new(0.0, 0.0, 10.0);
        front.target = Point3::origin();
        self.cameras.push(front);

        let mut side = Camera::new(1.0);
        side.position = Point3::new(10.0, 0.0, 0.0);
        side.target = Point3::origin();
        self.cameras.push(side);
    }

    pub fn active_camera(&self) -> &Camera {
        &self.cameras[self.active_camera]
    }

    pub fn active_camera_mut(&mut self) -> &mut Camera {
        &mut self.cameras[self.active_camera]
    }

    pub fn set_mode(&mut self, mode: ViewportMode) {
        self.mode = mode.clone();
        
        self.active_camera = match mode {
            ViewportMode::Perspective => 0,
            ViewportMode::Top | ViewportMode::Orthographic => 1,
            ViewportMode::Front => 2,
            ViewportMode::Side => 3,
            ViewportMode::Quad => 0,
        };
    }

    pub fn focus_on_object(&mut self, center: Point3<f32>, size: f32) {
        let camera = self.active_camera_mut();
        camera.target = center;
        
        let distance = size * 2.5;
        let direction = (camera.position - camera.target).normalize();
        camera.position = camera.target + direction * distance;
    }
}
