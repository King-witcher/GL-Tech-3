use crate::{vector::Vector, world::*};

pub struct Scene {
    pub camera: Camera,
    pub planes: Vec<Plane>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera {
                position: Vector(0.0, 0.0),
                rotation: 0.0,
            },
            planes: Vec::new(),
        }
    }
}
