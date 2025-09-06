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

    pub(crate) fn nearest_plane(&self, origin: Vector, direction: Vector) -> Option<(&Plane, f32)> {
        let mut nearest_distance = f32::INFINITY;
        let mut nearest_plane = None;

        for plane in &self.planes {
            if let Some(hit) = plane.test_ray(origin, direction) {
                if hit.distance < nearest_distance {
                    nearest_distance = hit.distance;
                    nearest_plane = Some(plane);
                }
            }
        }

        Some((nearest_plane?, nearest_distance))
    }
}
