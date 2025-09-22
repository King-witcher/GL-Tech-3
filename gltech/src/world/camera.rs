use crate::prelude::*;

pub struct Camera {
    pub pos: Vector,
    pub dir: Vector,
    pub z: f32,
    pub fov: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Vector::ZERO,
            dir: Vector::FORWARD,
            z: 0.0,
            fov: 110.0,
        }
    }
}
