use crate::prelude::*;

pub struct Camera {
    pub ray: Ray,
    pub z: f32,
    pub fov: f32,
}

impl Camera {}

impl Default for Camera {
    fn default() -> Self {
        Self {
            ray: Ray::new(Vector::ZERO, Vector::FORWARD),
            z: 0.5,
            fov: 90.0,
        }
    }
}
