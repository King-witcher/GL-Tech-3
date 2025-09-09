use crate::{vector::Vector, world::*};

// The Scene owns its entities and is responsible for dropping them when it goes out of scope. However, auxiliar structs
// like planes are owned by entities and the Scene only holds references to them for rendering and collision detection.
pub struct Scene {
    pub camera: Camera,
    pub(crate) planes: Vec<*mut Plane>,
    pub(crate) entities: Vec<EntityNode>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera {
                position: Vector(0.0, 0.0),
                rotation: 0.0,
            },
            planes: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: EntityNode) {
        let planes = node.planes.clone();
        self.planes.extend(planes);
        self.entities.push(node);
    }

    pub(crate) fn nearest_plane(&self, origin: Vector, direction: Vector) -> Option<(&Plane, f32)> {
        let mut nearest_distance = f32::INFINITY;
        let mut nearest_plane = None;

        for plane in self.planes.iter().map(|&p| unsafe { &*p }) {
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
